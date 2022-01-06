use aoc_runner_derive::{aoc, aoc_generator};
use nom::multi::{many1, many_m_n};
use nom::IResult;
use nom::{bits, complete::take, sequence::tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketBody {
    Value(usize),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operator {
    typ: u8,
    len: usize,
    packets: Vec<Packet>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketHeader {
    version: u8,
    typ: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    header: PacketHeader,
    body: PacketBody,
}

fn parse_packet_header(packet: (&[u8], usize)) -> IResult<(&[u8], usize), PacketHeader> {
    let (rest, (version, typ)) = tuple((take(3usize), take(3usize)))(packet)?;

    Ok((rest, PacketHeader { version, typ }))
}

fn parse_literal_value(body: (&[u8], usize)) -> IResult<(&[u8], usize), PacketBody> {
    let mut acc = 0;

    let mut body = body;

    loop {
        body = {
            let (body, cont): (_, u8) = take(1usize)(body)?;
            let (body, val): (_, usize) = take(4usize)(body)?;

            acc = acc << 4 | val;

            if cont == 0 {
                return Ok((body, PacketBody::Value(acc)));
            }

            body
        };
    }
}

fn parse_packets_by_length(body: (&[u8], usize)) -> IResult<(&[u8], usize), Operator> {
    let (body, body_len): (_, usize) = take(15usize)(body)?;

    // println!("operator w/ bit len: {}", body_len);

    let body_final_offset = (body.1 + body_len) % 8;

    let subpacket_start = body.1;
    let subpacket_end = body.1 + body_len;
    let subpacket_end_rem = subpacket_end % 8;

    let subpacket_end_byte = subpacket_end / 8 + 1;

    let subpacket_body = (&body.0[..subpacket_end_byte], subpacket_start);
    let (_, packets) = many1(parse_packet_bits)(subpacket_body)?;

    let body = (
        &body.0[subpacket_end_byte - 1..],
        (subpacket_end_rem + subpacket_start) % 8,
    );

    Ok((
        (body.0, body_final_offset),
        Operator {
            typ: 0,
            len: body_len,
            packets,
        },
    ))
}

fn parse_packets_by_count(body: (&[u8], usize)) -> IResult<(&[u8], usize), Operator> {
    let (body, body_count) = take(11usize)(body)?;

    // println!("operator w/ count: {}", body_count);

    let (rest, packets) = many_m_n(body_count, body_count, parse_packet_bits)(body)?;

    Ok((
        rest,
        Operator {
            typ: 1,
            len: body_count,
            packets,
        },
    ))
}

fn parse_operator(body: (&[u8], usize)) -> IResult<(&[u8], usize), PacketBody> {
    let (body, length_flag): (_, u8) = take(1usize)(body)?;

    let (body, packets) = match length_flag {
        0 => parse_packets_by_length(body)?,
        1 => parse_packets_by_count(body)?,
        _ => unreachable!(),
    };

    Ok((body, PacketBody::Operator(packets)))
}

fn parse_packet_bits(packet: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    // println!("parsing {:?}", packet);
    let (body, header) = parse_packet_header(packet)?;

    // println!("header: {:?}", header);

    let (rest, body) = match header.typ {
        4 => parse_literal_value(body)?,
        _ => parse_operator(body)?,
    };

    // println!("body: {:?}", body);

    let packet = Packet { header, body };
    // println!("parsed: {:?}", packet);
    // println!("remaining: {:?}", rest);
    // println!("remaining:{} {:?}", rest.1, rest.0);
    Ok((rest, packet))
}

fn parse_packet_bytes(packet: &[u8]) -> IResult<&[u8], Packet> {
    bits(parse_packet_bits)(packet)
}

fn parse_input(input: &str) -> Packet {
    let bytes = hex::decode(input).unwrap();
    let (_, packet) = parse_packet_bytes(&bytes).unwrap();
    packet
}

fn sum_version(packet: &Packet) -> usize {
    packet.header.version as usize
        + match &packet.body {
            PacketBody::Value(_) => 0,
            PacketBody::Operator(Operator { packets, .. }) => packets.iter().map(sum_version).sum(),
        }
}

fn solve_p1(target: &Packet) -> usize {
    sum_version(target)
}

fn eval(packet: &Packet) -> usize {
    match &packet.body {
        PacketBody::Value(v) => {
            assert_eq!(packet.header.typ, 4);
            *v
        }
        PacketBody::Operator(Operator { packets, .. }) => {
            let mut packets_iter = packets.iter().map(eval);
            match packet.header.typ {
                0 => packets_iter.sum(),
                1 => packets_iter.reduce(|a, b| a * b).unwrap(),
                2 => packets_iter.min().unwrap(),
                3 => packets_iter.max().unwrap(),
                5 => {
                    if packets_iter.next().unwrap() > packets_iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets_iter.next().unwrap() < packets_iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets_iter.next().unwrap() == packets_iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn solve_p2(target: &Packet) -> usize {
    eval(target)
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Packet {
    parse_input(input)
}

#[aoc(day16, part1)]
pub fn wrapper_p1(input: &Packet) -> usize {
    solve_p1(input)
}

#[aoc(day16, part2)]
pub fn wrapper_p2(input: &Packet) -> usize {
    solve_p2(input)
}

#[cfg(test)]
mod tests {
    use crate::day16::{Operator, Packet, PacketBody, PacketHeader};

    #[test]
    fn test_value() {
        let input = "D2FE28";

        let parsed_input = super::input_generator(input);

        assert_eq!(
            Packet {
                header: PacketHeader { version: 6, typ: 4 },
                body: PacketBody::Value(2021)
            },
            parsed_input
        );

        assert_eq!(6, super::solve_p1(&parsed_input));
    }

    #[test]
    fn test_operator_len() {
        let input = "38006F45291200";
        let expect = Packet {
            header: PacketHeader { version: 1, typ: 6 },
            body: PacketBody::Operator(Operator {
                typ: 0,
                len: 27,
                packets: vec![
                    Packet {
                        header: PacketHeader { version: 6, typ: 4 },
                        body: PacketBody::Value(10),
                    },
                    Packet {
                        header: PacketHeader { version: 2, typ: 4 },
                        body: PacketBody::Value(20),
                    },
                ],
            }),
        };

        println!("expecting: {:?}", expect);
        let parsed_input = super::input_generator(input);

        assert_eq!(expect, parsed_input);

        assert_eq!(9, super::solve_p1(&parsed_input));
    }

    #[test]
    fn test_operator_count() {
        let input = "EE00D40C823060";
        let expect = Packet {
            header: PacketHeader { version: 7, typ: 3 },
            body: PacketBody::Operator(Operator {
                typ: 1,
                len: 3,
                packets: vec![
                    Packet {
                        header: PacketHeader { version: 2, typ: 4 },
                        body: PacketBody::Value(1),
                    },
                    Packet {
                        header: PacketHeader { version: 4, typ: 4 },
                        body: PacketBody::Value(2),
                    },
                    Packet {
                        header: PacketHeader { version: 1, typ: 4 },
                        body: PacketBody::Value(3),
                    },
                ],
            }),
        };

        println!("expecting: {:?}", expect);
        let parsed_input = super::input_generator(input);

        assert_eq!(expect, parsed_input);

        assert_eq!(14, super::solve_p1(&parsed_input));
    }

    #[test]
    fn test_p1() {
        let inputs = vec![
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        for (input, expect) in inputs {
            println!("input: {:?}", input);

            let parsed_input = super::input_generator(input);

            println!("parsed: {:?}", parsed_input);

            assert_eq!(expect, super::solve_p1(&parsed_input));
        }
    }

    #[test]
    fn test_p2() {
        let inputs = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (input, expect) in inputs {
            println!("input: {:?}", input);

            let parsed_input = super::input_generator(input);

            println!("parsed: {:?}", parsed_input);

            assert_eq!(expect, super::solve_p2(&parsed_input));
        }
    }
}
