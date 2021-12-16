use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    kind: PacketType,
    version: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Literal(usize),
    Operator(OpKind, Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
enum OpKind {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<&[u8]> for OpKind {
    fn from(input: &[u8]) -> Self {
        match input {
            [0, 0, 0] => OpKind::Sum,
            [0, 0, 1] => OpKind::Product,
            [0, 1, 0] => OpKind::Minimum,
            [0, 1, 1] => OpKind::Maximum,
            [1, 0, 1] => OpKind::GreaterThan,
            [1, 1, 0] => OpKind::LessThan,
            [1, 1, 1] => OpKind::EqualTo,
            _ => unreachable!(),
        }
    }
}

impl PacketType {
    fn apply(&self) -> usize {
        match self {
            PacketType::Literal(val) => *val,
            PacketType::Operator(OpKind::Sum, vals) => {
                vals.iter().map(|packet| packet.kind.apply()).sum()
            }
            PacketType::Operator(OpKind::Product, vals) => {
                vals.iter().map(|packet| packet.kind.apply()).product()
            }
            PacketType::Operator(OpKind::Minimum, vals) => {
                vals.iter().map(|packet| packet.kind.apply()).min().unwrap()
            }
            PacketType::Operator(OpKind::Maximum, vals) => {
                vals.iter().map(|packet| packet.kind.apply()).max().unwrap()
            }
            PacketType::Operator(OpKind::GreaterThan, vals) => {
                let mut iter = vals.iter().map(|packet| packet.kind.apply());
                (iter.next().expect("couldn't get first")
                    > iter.next().expect("couldn't get second")) as usize
            }
            PacketType::Operator(OpKind::LessThan, vals) => {
                let mut iter = vals.iter().map(|packet| packet.kind.apply());
                (iter.next().expect("couldn't get first")
                    < iter.next().expect("couldn't get second")) as usize
            }
            PacketType::Operator(OpKind::EqualTo, vals) => {
                let mut iter = vals.iter().map(|packet| packet.kind.apply());
                (iter.next().expect("couldn't get first")
                    == iter.next().expect("couldn't get second")) as usize
            }
        }
    }
}

impl Packet {
    fn apply(&self) -> usize {
        self.kind.apply()
    }
}

fn vec_to_num(input: &[u8]) -> usize {
    input.iter().fold(0, |mut memo, bit| {
        memo <<= 1;
        memo += *bit as usize;
        memo
    })
}

fn take_operator<'a>(bits: &'a [u8], opkind: &'a [u8]) -> Result<(PacketType, &'a [u8])> {
    let (length_type_id, mut rest) = bits.split_at(1);
    let packets = match length_type_id {
        [1] => {
            let (length, mut r) = rest.split_at(11);
            let num = vec_to_num(length);
            let mut packets = vec![];
            for _ in 0..num {
                let (packet, p) = take_packet(r)?;
                packets.push(packet);
                r = p;
            }
            rest = r;
            packets
        }
        [0] => {
            let (length, r) = rest.split_at(15);
            let num = vec_to_num(length);
            let mut packets = vec![];
            let (mut pstr, r) = r.split_at(num);

            while let Ok((packet, p)) = take_packet(pstr) {
                packets.push(packet);
                pstr = p;
                if pstr.iter().all(|num| num == &0) {
                    break;
                }
            }
            rest = r;
            packets
        }
        _ => unreachable!(),
    };

    Ok((PacketType::Operator(opkind.into(), packets), rest))
}

fn take_message(bits: &[u8]) -> Result<(PacketType, &[u8])> {
    let mut rest = bits;
    let mut message = vec![];
    loop {
        let (m, r) = rest.split_at(5);
        rest = r;
        match m {
            [1, msg @ ..] => {
                message.extend_from_slice(msg);
            }
            [0, msg @ ..] => {
                // TODO: ignore some number of 0s
                message.extend_from_slice(msg);
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok((PacketType::Literal(vec_to_num(&message)), rest))
}

fn take_packet(bits: &[u8]) -> Result<(Packet, &[u8])> {
    let (version, rest) = bits.split_at(3);
    let (ptype, rest) = rest.split_at(3);
    let (kind, rest) = match ptype {
        [1, 0, 0] => take_message(rest)?,
        opkind => take_operator(rest, opkind)?,
    };
    Ok((
        Packet {
            kind,
            version: vec_to_num(version),
        },
        rest,
    ))
}

fn parse_input(input: &str) -> Result<Packet> {
    let hex = input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(16)
                .with_context(|| format!("couldn't convert to binary {:?}", c))
        })
        .collect::<Result<Vec<_>>>()?;

    let bits = hex
        .into_iter()
        .flat_map(|num| {
            let mut num = num as u8;
            let mut container = [0u8; 4];
            for i in (0..4).rev() {
                container[i] = 1 & num;
                num >>= 1;
            }

            container
        })
        .collect::<Vec<u8>>();

    let (packet, bits) = take_packet(&bits)?;
    if bits.is_empty() {
        Ok(packet)
    } else {
        if bits.iter().all(|num| num == &0) {
            return Ok(packet);
        }
        None.with_context(|| format!("{:?}", bits))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_literal() {
        let input = "D2FE28";
        let packet = parse_input(input).unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 6,
                kind: PacketType::Literal(2021)
            }
        )
    }

    #[test]
    fn test_operator_packet_number() {
        let input = "EE00D40C823060";
        let packet = parse_input(input).unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 7,
                kind: PacketType::Operator(
                    OpKind::Maximum,
                    vec![
                        Packet {
                            kind: PacketType::Literal(1),
                            version: 2,
                        },
                        Packet {
                            kind: PacketType::Literal(2),
                            version: 4,
                        },
                        Packet {
                            kind: PacketType::Literal(3),
                            version: 1,
                        },
                    ]
                )
            }
        )
    }

    #[test]
    fn test_operator_bit_number() {
        let input = "38006F45291200";
        let packet = parse_input(input).unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 1,
                kind: PacketType::Operator(
                    OpKind::LessThan,
                    vec![
                        Packet {
                            kind: PacketType::Literal(10),
                            version: 6,
                        },
                        Packet {
                            kind: PacketType::Literal(20),
                            version: 2,
                        },
                    ]
                )
            }
        )
    }

    #[test]
    fn test_operator_sum() {
        fn input_output(input: &str, output: usize) {
            let packet = parse_input(input).unwrap();
            assert_eq!(packet.apply(), output);
        }

        input_output("C200B40A82", 3);
        input_output("04005AC33890", 54);
        input_output("880086C3E88112", 7);
        input_output("CE00C43D881120", 9);
        input_output("D8005AC2A8F0", 1);
        input_output("F600BC2D8F", 0);
        input_output("9C005AC2F8F0", 0);
        input_output("9C0141080250320F1802104A08", 1);
    }
}

fn version_sum(packet: &Packet) -> usize {
    let mut sum = 0;

    let Packet {
        version: v,
        kind: k,
    } = packet;
    sum += v;

    if let PacketType::Operator(_, packets) = k {
        sum += packets
            .iter()
            .map(|packet| version_sum(packet))
            .sum::<usize>()
    }

    sum
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let packet = parse_input(input)?;
    let version_sum = version_sum(&packet);
    println!("part1 {}", version_sum);
    println!("part2 {}", packet.apply());

    Ok(())
}
