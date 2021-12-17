#[derive(Debug, PartialEq, Clone)]
struct Input {
    bits: Vec<bool>,
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        packets: Vec<Packet>,
    },
}

pub fn to_usize(bits: &[bool]) -> usize {
    bits.iter().fold(0, |n, &bit| (n * 2) + (if bit { 1 } else { 0 }))
}

pub fn read_literal_value(bits: &[bool]) -> (usize, usize) {
    let mut pos = 0;
    let mut n = 0;

    loop {
        let (first, nexts) = bits[pos..pos+5].split_first().unwrap();

        n <<= 4;
        n += to_usize(nexts);
        pos += 5;

        if !*first {
            return (n, pos);
        }
    }
}

impl Packet {
    pub fn sum_of_versions(&self) -> usize {
        match self {
            Packet::Literal {
                version,
                value: _value
            } => *version,
            Packet::Operator {
                version,
                type_id: _type_id,
                packets
            } => packets.iter()
                .map(|packet| packet.sum_of_versions())
                .sum::<usize>() + *version,
        }
    }

    pub fn eval(&self) -> usize {
        match self {
            Packet::Literal {
                version: _version,
                value
            } => *value,
            Packet::Operator {
                version: _version,
                type_id,
                packets
            } => {
                let mut values = packets.iter().map(Packet::eval);

                match type_id {
                    0 => values.sum(),
                    1 => values.product(),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 => (values.next().unwrap() > values.next().unwrap()) as usize,
                    6 => (values.next().unwrap() < values.next().unwrap()) as usize,
                    7 => (values.next().unwrap() == values.next().unwrap()) as usize,
                    _ => panic!("{}", type_id),
                }
            },
        }
    }
}

fn read_packets(bits: &[bool]) -> (Vec<Packet>, usize) {
    let mut packets = vec![];

    let mut total_length = usize::MAX;
    let mut number_of_subpacket = usize::MAX;
    let mut pos = 0;

    if bits[0] {
        number_of_subpacket = to_usize(&bits[1..12]);
        pos += 12;
    } else {
        pos += 16;
        total_length = pos + to_usize(&bits[1..16]);
    }

    while pos < total_length && packets.len() < number_of_subpacket {
        let (packet, n) = read_packet(&bits[pos..]);

        pos += n;
        packets.push(packet);
    }

    (packets, pos)
}

fn read_packet(bits: &[bool]) -> (Packet, usize) {
    let version = to_usize(&bits[0..3]);

    match to_usize(&bits[3..6]) {
        4 => {
            let (value, n) = read_literal_value(&bits[6..]);
            (Packet::Literal {
                version,
                value,
            }, 6 + n)
        },
        type_id => {
            let (packets, n) = read_packets(&bits[6..]);
            (Packet::Operator {
                version,
                type_id,
                packets,
            }, 6 + n)
        },
    }
}

impl From<Input> for Packet {
    fn from(input: Input) -> Self {
        read_packet(&input.bits).0
    }
}

#[allow(unused_variables)]
#[aoc_generator(day16)]
fn input_generator(input: &str) -> Input {
    Input {
        bits: input.chars()
            .flat_map(|c| match c {
                '0' => [false, false, false, false],
                '1' => [false, false, false, true],
                '2' => [false, false, true,  false],
                '3' => [false, false, true,  true],
                '4' => [false, true,  false, false],
                '5' => [false, true,  false, true],
                '6' => [false, true,  true,  false],
                '7' => [false, true,  true,  true],
                '8' => [true,  false, false, false],
                '9' => [true,  false, false, true],
                'A' => [true,  false, true,  false],
                'B' => [true,  false, true,  true],
                'C' => [true,  true,  false, false],
                'D' => [true,  true,  false,  true],
                'E' => [true,  true,  true,  false],
                'F' => [true,  true,  true, true],
                _ => panic!(),
            })
            .collect::<Vec<_>>()
    }
}

type Output = usize;

#[allow(unused_variables)]
#[aoc(day16, part1)]

fn part1(input: &Input) -> Output {
    Packet::from(input.clone()).sum_of_versions()
}

#[allow(unused_variables)]
#[aoc(day16, part2)]

fn part2(input: &Input) -> Output {
    Packet::from(input.clone()).eval()
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator("C200B40A82")), 3);
        assert_eq!(part2(&input_generator("04005AC33890")), 54);
        assert_eq!(part2(&input_generator("880086C3E88112")), 7);
        assert_eq!(part2(&input_generator("CE00C43D881120")), 9);
        assert_eq!(part2(&input_generator("D8005AC2A8F0")), 1);
        assert_eq!(part2(&input_generator("F600BC2D8F")), 0);
        assert_eq!(part2(&input_generator("9C005AC2F8F0")), 0);
        assert_eq!(part2(&input_generator("9C0141080250320F1802104A08")), 1);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator("8A004A801A8002F478")), 16);
        assert_eq!(part1(&input_generator("620080001611562C8802118E34")), 12);
        assert_eq!(part1(&input_generator("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part1(&input_generator("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn operator_length_type_1_packet() {
        assert_eq!(Packet::from(input_generator("EE00D40C823060")), Packet::Operator {
            version: 7,
            type_id: 3,
            packets: vec![
                Packet::Literal {
                    version: 2,
                    value: 1,
                },
                Packet::Literal {
                    version: 4,
                    value: 2,
                },
                Packet::Literal {
                    version: 1,
                    value: 3,
                },
            ],
        });
    }

    #[test]
    fn operator_length_type_0_packet() {
        assert_eq!(Packet::from(input_generator("38006F45291200")), Packet::Operator {
            version: 1,
            type_id: 6,
            packets: vec![
                Packet::Literal {
                    version: 6,
                    value: 10,
                },
                Packet::Literal {
                    version: 2,
                    value: 20,
                },
            ],
        });
    }

    #[test]
    fn literal_packet() {
        assert_eq!(Packet::from(input_generator("D2FE28")), Packet::Literal {
            version: 6,
            value: 2021,
        });
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator("D2FE28"), Input {
            bits: vec![
                true,
                true,
                false,
                true,
                false,
                false,
                true,
                false,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                false,
                false,
                false,
                true,
                false,
                true,
                false,
                false,
                false,
            ],
        });
    }
}
