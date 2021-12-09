#[derive(Debug, PartialEq)]
struct Input {
    lines: Vec<Line>
}

#[derive(Debug, PartialEq)]

struct Line {
    signal_patterns: Vec<u8>,
    output_values: Vec<u8>,
}

#[allow(unused_variables)]
#[aoc_generator(day8)]
fn input_generator(input: &str) -> Input {
    Input {
        lines: input.lines()
            .map(|line| {
                let (signal_patterns, output_values) = line.split_once(" | ").unwrap();

                Line {
                    signal_patterns: signal_patterns
                        .split(' ')
                        .map(|s| s.bytes().map(|b| 1 << (b - b'a')).sum())
                        .collect(),
                    output_values: output_values
                        .split(' ')
                        .map(|s| s.bytes().map(|b| 1 << (b - b'a')).sum())
                        .collect(),
                }
            })
            .collect(),
    }
}

type Output = usize;

#[allow(unused_variables)]
#[aoc(day8, part1)]

fn part1(input: &Input) -> Output {
    input.lines.iter()
        .map(|line| line.output_values.iter()
            .filter(|s| match s.count_ones() {
                2|4|3|7 => true,
                _ => false,
            })
            .count()
        )
        .sum()
}

#[allow(unused_variables)]
#[aoc(day8, part2)]

fn part2(input: &Input) -> Output {
    input.lines.iter()
        .map(|line| {
            let mut digits = [0; 10];

            for n in &line.signal_patterns {
                match n.count_ones() {
                    2 => digits[1] = *n,
                    4 => digits[4] = *n,
                    3 => digits[7] = *n,
                    7 => digits[8] = *n,
                    _ => (),
                }
            }

            for n in &line.signal_patterns {
                let one = (n & digits[1]).count_ones();
                let four = (n & digits[4]).count_ones();

                match (n.count_ones(), one, four) {
                    (6, 2, 3) => digits[0] = *n,
                    (5, 1, 2) => digits[2] = *n,
                    (5, 2, 3) => digits[3] = *n,
                    (5, 1, 3) => digits[5] = *n,
                    (6, 1, 3) => digits[6] = *n,
                    (6, 2, 4) => digits[9] = *n,
                    _ => (),
                }
            }

            line.output_values.iter()
                .map(|n| digits.iter().position(|d| d == n).unwrap())
                .fold(0, |sum, n| 10 * sum + n)
        })
        .sum()
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), 61229);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), 26);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input()), Input {
            lines: vec![
                Line {
                    signal_patterns: vec![
                        "be".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cfbegad".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cbdgef".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fgaecd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cgeb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fdcge".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "agebfd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fecdb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fabcd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "edb".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "fdgacbe".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cefdb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cefbgd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gcbe".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "edbfga".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "begcd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cbg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gc".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gcadebf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fbgde".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "acbgfd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "abcde".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gfcbed".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gfec".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "fcgedb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cgb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "dgebacf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gc".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "fgaebd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bdaec".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gdafb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "agbcfd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gdcbef".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bgcad".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gfac".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gcb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cdgabef".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "cg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fdcagb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cbg".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "fbegcd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cbd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "adcefb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "dageb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "afcb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bc".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "aefdc".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "ecdab".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fgdeca".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fcdbega".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "efabcd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cedba".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gadfec".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cb".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "aecbfdg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fbg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bafeg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "dbefa".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fcge".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gcbea".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fcaegb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "dgceab".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fcbdga".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "gecf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "egdcabf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bgf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bfgea".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "fgeab".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "ca".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "afcebg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bdacfeg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cfaedg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gcfdb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "baec".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bfadeg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bafgc".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "acf".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "gebdcfa".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "ecba".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "ca".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fadegcb".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "dbcfg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fgd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bdegcaf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fgec".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "aegbdf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "ecdfab".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fbedc".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "dacgb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gdcebf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gf".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "cefg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "dcbef".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fcge".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gbcadfe".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "bdfegc".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cbegaf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gecbf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "dfcage".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bdacg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "ed".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bedf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "ced".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "adcbefg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gebcd".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "ed".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bcgafe".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cdgba".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cbgef".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "egadfb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cdbfeg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cegd".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fecab".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cgb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gbdefca".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fgcdab".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "egfdb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bfceg".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "gbdfcae".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bgc".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cgb".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "gcafb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gcf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "dcaebfg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "ecagb".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gf".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "abcdeg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "gaef".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cafbge".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fdbac".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fegbdc".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                    output_values: vec![
                        "fgae".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "cfgab".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "fg".bytes().map(|b| 1 << (b - b'a')).sum(),
                        "bagce".bytes().map(|b| 1 << (b - b'a')).sum(),
                    ],
                },
            ]
        });
    }
}
