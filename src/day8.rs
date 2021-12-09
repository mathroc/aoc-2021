#[derive(Debug, PartialEq)]
struct Input {
    lines: Vec<Line>
}

#[derive(Debug, PartialEq)]

struct Line {
    signal_patterns: Vec<Vec<char>>,
    output_values: Vec<Vec<char>>,
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
                        .map(|s| s.chars().collect::<Vec<_>>())
                        .collect(),
                    output_values: output_values
                        .split(' ')
                        .map(|s| s.chars().collect::<Vec<_>>())
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
            .filter(|s| match s.len() {
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
    todo!()
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
        assert_eq!(part2(&input_generator(exemple_raw_input())), todo!());
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
                        "be".chars().collect::<Vec<_>>(),
                        "cfbegad".chars().collect::<Vec<_>>(),
                        "cbdgef".chars().collect::<Vec<_>>(),
                        "fgaecd".chars().collect::<Vec<_>>(),
                        "cgeb".chars().collect::<Vec<_>>(),
                        "fdcge".chars().collect::<Vec<_>>(),
                        "agebfd".chars().collect::<Vec<_>>(),
                        "fecdb".chars().collect::<Vec<_>>(),
                        "fabcd".chars().collect::<Vec<_>>(),
                        "edb".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "fdgacbe".chars().collect::<Vec<_>>(),
                        "cefdb".chars().collect::<Vec<_>>(),
                        "cefbgd".chars().collect::<Vec<_>>(),
                        "gcbe".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "edbfga".chars().collect::<Vec<_>>(),
                        "begcd".chars().collect::<Vec<_>>(),
                        "cbg".chars().collect::<Vec<_>>(),
                        "gc".chars().collect::<Vec<_>>(),
                        "gcadebf".chars().collect::<Vec<_>>(),
                        "fbgde".chars().collect::<Vec<_>>(),
                        "acbgfd".chars().collect::<Vec<_>>(),
                        "abcde".chars().collect::<Vec<_>>(),
                        "gfcbed".chars().collect::<Vec<_>>(),
                        "gfec".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "fcgedb".chars().collect::<Vec<_>>(),
                        "cgb".chars().collect::<Vec<_>>(),
                        "dgebacf".chars().collect::<Vec<_>>(),
                        "gc".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "fgaebd".chars().collect::<Vec<_>>(),
                        "cg".chars().collect::<Vec<_>>(),
                        "bdaec".chars().collect::<Vec<_>>(),
                        "gdafb".chars().collect::<Vec<_>>(),
                        "agbcfd".chars().collect::<Vec<_>>(),
                        "gdcbef".chars().collect::<Vec<_>>(),
                        "bgcad".chars().collect::<Vec<_>>(),
                        "gfac".chars().collect::<Vec<_>>(),
                        "gcb".chars().collect::<Vec<_>>(),
                        "cdgabef".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "cg".chars().collect::<Vec<_>>(),
                        "cg".chars().collect::<Vec<_>>(),
                        "fdcagb".chars().collect::<Vec<_>>(),
                        "cbg".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "fbegcd".chars().collect::<Vec<_>>(),
                        "cbd".chars().collect::<Vec<_>>(),
                        "adcefb".chars().collect::<Vec<_>>(),
                        "dageb".chars().collect::<Vec<_>>(),
                        "afcb".chars().collect::<Vec<_>>(),
                        "bc".chars().collect::<Vec<_>>(),
                        "aefdc".chars().collect::<Vec<_>>(),
                        "ecdab".chars().collect::<Vec<_>>(),
                        "fgdeca".chars().collect::<Vec<_>>(),
                        "fcdbega".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "efabcd".chars().collect::<Vec<_>>(),
                        "cedba".chars().collect::<Vec<_>>(),
                        "gadfec".chars().collect::<Vec<_>>(),
                        "cb".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "aecbfdg".chars().collect::<Vec<_>>(),
                        "fbg".chars().collect::<Vec<_>>(),
                        "gf".chars().collect::<Vec<_>>(),
                        "bafeg".chars().collect::<Vec<_>>(),
                        "dbefa".chars().collect::<Vec<_>>(),
                        "fcge".chars().collect::<Vec<_>>(),
                        "gcbea".chars().collect::<Vec<_>>(),
                        "fcaegb".chars().collect::<Vec<_>>(),
                        "dgceab".chars().collect::<Vec<_>>(),
                        "fcbdga".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "gecf".chars().collect::<Vec<_>>(),
                        "egdcabf".chars().collect::<Vec<_>>(),
                        "bgf".chars().collect::<Vec<_>>(),
                        "bfgea".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "fgeab".chars().collect::<Vec<_>>(),
                        "ca".chars().collect::<Vec<_>>(),
                        "afcebg".chars().collect::<Vec<_>>(),
                        "bdacfeg".chars().collect::<Vec<_>>(),
                        "cfaedg".chars().collect::<Vec<_>>(),
                        "gcfdb".chars().collect::<Vec<_>>(),
                        "baec".chars().collect::<Vec<_>>(),
                        "bfadeg".chars().collect::<Vec<_>>(),
                        "bafgc".chars().collect::<Vec<_>>(),
                        "acf".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "gebdcfa".chars().collect::<Vec<_>>(),
                        "ecba".chars().collect::<Vec<_>>(),
                        "ca".chars().collect::<Vec<_>>(),
                        "fadegcb".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "dbcfg".chars().collect::<Vec<_>>(),
                        "fgd".chars().collect::<Vec<_>>(),
                        "bdegcaf".chars().collect::<Vec<_>>(),
                        "fgec".chars().collect::<Vec<_>>(),
                        "aegbdf".chars().collect::<Vec<_>>(),
                        "ecdfab".chars().collect::<Vec<_>>(),
                        "fbedc".chars().collect::<Vec<_>>(),
                        "dacgb".chars().collect::<Vec<_>>(),
                        "gdcebf".chars().collect::<Vec<_>>(),
                        "gf".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "cefg".chars().collect::<Vec<_>>(),
                        "dcbef".chars().collect::<Vec<_>>(),
                        "fcge".chars().collect::<Vec<_>>(),
                        "gbcadfe".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "bdfegc".chars().collect::<Vec<_>>(),
                        "cbegaf".chars().collect::<Vec<_>>(),
                        "gecbf".chars().collect::<Vec<_>>(),
                        "dfcage".chars().collect::<Vec<_>>(),
                        "bdacg".chars().collect::<Vec<_>>(),
                        "ed".chars().collect::<Vec<_>>(),
                        "bedf".chars().collect::<Vec<_>>(),
                        "ced".chars().collect::<Vec<_>>(),
                        "adcbefg".chars().collect::<Vec<_>>(),
                        "gebcd".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "ed".chars().collect::<Vec<_>>(),
                        "bcgafe".chars().collect::<Vec<_>>(),
                        "cdgba".chars().collect::<Vec<_>>(),
                        "cbgef".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "egadfb".chars().collect::<Vec<_>>(),
                        "cdbfeg".chars().collect::<Vec<_>>(),
                        "cegd".chars().collect::<Vec<_>>(),
                        "fecab".chars().collect::<Vec<_>>(),
                        "cgb".chars().collect::<Vec<_>>(),
                        "gbdefca".chars().collect::<Vec<_>>(),
                        "cg".chars().collect::<Vec<_>>(),
                        "fgcdab".chars().collect::<Vec<_>>(),
                        "egfdb".chars().collect::<Vec<_>>(),
                        "bfceg".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "gbdfcae".chars().collect::<Vec<_>>(),
                        "bgc".chars().collect::<Vec<_>>(),
                        "cg".chars().collect::<Vec<_>>(),
                        "cgb".chars().collect::<Vec<_>>(),
                    ],
                },
                Line {
                    signal_patterns: vec![
                        "gcafb".chars().collect::<Vec<_>>(),
                        "gcf".chars().collect::<Vec<_>>(),
                        "dcaebfg".chars().collect::<Vec<_>>(),
                        "ecagb".chars().collect::<Vec<_>>(),
                        "gf".chars().collect::<Vec<_>>(),
                        "abcdeg".chars().collect::<Vec<_>>(),
                        "gaef".chars().collect::<Vec<_>>(),
                        "cafbge".chars().collect::<Vec<_>>(),
                        "fdbac".chars().collect::<Vec<_>>(),
                        "fegbdc".chars().collect::<Vec<_>>(),
                    ],
                    output_values: vec![
                        "fgae".chars().collect::<Vec<_>>(),
                        "cfgab".chars().collect::<Vec<_>>(),
                        "fg".chars().collect::<Vec<_>>(),
                        "bagce".chars().collect::<Vec<_>>(),
                    ],
                },
            ]
        });
    }
}
