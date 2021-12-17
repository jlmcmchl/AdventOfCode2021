use aoc_runner_derive::{aoc, aoc_generator};

pub enum PacketContent {
    Literal(usize),
    Operator(),
}

pub enum LengthType {
    Bits(usize),
    Packets(usize),
}

pub struct OperatorContent {
    length: LengthType,
    packets: Vec<Packet>,
}

pub struct Packet {
    version: u8,
    typ: u8,
    content: PacketContent,
}

fn parse_input(input: &str) -> Packet {
    Packet {
        version: 0,
        typ: 0,
        content: PacketContent::Literal(0),
    }
}

fn solve_p1(input: &Packet) -> usize {
    Default::default()
}

fn solve_p2(input: &Packet) -> usize {
    Default::default()
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
    #[test]
    fn it_works() {
        let inputs = vec![
            ("D2FE28", 6),
            ("38006F45291200", 9),
            ("EE00D40C823060", 14),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        for (input, version_sum) in inputs {
            let parsed_input = super::input_generator(input);
            assert_eq!(version_sum, super::solve_p1(&parsed_input));
        }
    }
}
