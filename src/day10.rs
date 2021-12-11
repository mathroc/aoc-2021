use core::panic;
use std::{str::FromStr, fmt::{Display, Write}};

#[derive(Debug, PartialEq)]
struct Input {
    lines: Vec<Line>,
}

#[derive(Debug, PartialEq)]
enum Error {
    Corrupted {
        position: usize,
        expected: ChunkChar,
        found: ChunkChar,
    },
    Incomplete {
        expecteds: Vec<ChunkChar>,
    },
}

impl Error {
    pub fn score(&self) -> usize {
        match self {
            Self::Corrupted {
                position: _position,
                expected: _expected,
                found,
            } => found.error_score(),
            Self::Incomplete {
                expecteds
            } => expecteds.iter()
                .fold(0, |score, char| score * 5 + char.autocomplete_score()),
        }
    }
}

impl Input {
    pub fn parsed_lines(&self) -> Vec<Result<usize, Error>> {
        self.lines.iter().map(|line| line.parse(0)).collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ChunkChar {
    Left(ChunkCharType),
    Right(ChunkCharType),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ChunkCharType {
    Parenthesis,
    SquareBracket,
    CurlyBracket,
    AngleBracket,
}

impl ChunkCharType {
    pub fn error_score(&self) -> usize {
        match self {
            ChunkCharType::Parenthesis => 3,
            ChunkCharType::SquareBracket => 57,
            ChunkCharType::CurlyBracket => 1197,
            ChunkCharType::AngleBracket => 25137,
        }
    }

    pub fn autocomplete_score(&self) -> usize {
        match self {
            ChunkCharType::Parenthesis => 1,
            ChunkCharType::SquareBracket => 2,
            ChunkCharType::CurlyBracket => 3,
            ChunkCharType::AngleBracket => 4,
        }
    }
}

impl ChunkChar {
    pub fn error_score(&self) -> usize {
        match self {
            ChunkChar::Left(t) | ChunkChar::Right(t) => t.error_score()
        }
    }

    pub fn autocomplete_score(&self) -> usize {
        match self {
            ChunkChar::Left(t) | ChunkChar::Right(t) => t.autocomplete_score()
        }
    }

    pub fn matching(&self) -> ChunkChar {
        match self {
            ChunkChar::Left(t) => ChunkChar::Right(*t),
            ChunkChar::Right(t) => ChunkChar::Left(*t),
        }
    }
}

impl From<char> for ChunkChar {
    fn from(c: char) -> Self {
        match c {
            '(' => ChunkChar::Left(ChunkCharType::Parenthesis),
            '[' => ChunkChar::Left(ChunkCharType::SquareBracket),
            '{' => ChunkChar::Left(ChunkCharType::CurlyBracket),
            '<' => ChunkChar::Left(ChunkCharType::AngleBracket),
            ')' => ChunkChar::Right(ChunkCharType::Parenthesis),
            ']' => ChunkChar::Right(ChunkCharType::SquareBracket),
            '}' => ChunkChar::Right(ChunkCharType::CurlyBracket),
            '>' => ChunkChar::Right(ChunkCharType::AngleBracket),
            _ => panic!(),
        }
    }
}

impl Display for ChunkChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            ChunkChar::Left(ChunkCharType::Parenthesis) => '(',
            ChunkChar::Left(ChunkCharType::SquareBracket) => '[',
            ChunkChar::Left(ChunkCharType::CurlyBracket) => '{',
            ChunkChar::Left(ChunkCharType::AngleBracket) => '<',
            ChunkChar::Right(ChunkCharType::Parenthesis) => ')',
            ChunkChar::Right(ChunkCharType::SquareBracket) => ']',
            ChunkChar::Right(ChunkCharType::CurlyBracket) => '}',
            ChunkChar::Right(ChunkCharType::AngleBracket) => '>',
        })
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    chars: Vec<ChunkChar>,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line {
            chars: s.chars()
                .map(|c| c.into())
                .collect::<Vec<_>>(),
        })
    }
}

impl Line {
    pub fn parse(&self, position: usize) -> Result<usize, Error> {
        let mut l = 0;

        match self.chars.get(position) {
            None | Some(ChunkChar::Right(_)) => Ok(l),
            Some(current) => {
                let expected = current.matching();
                let n = self.parse(position + 1)
                    .map_err(|err| match err {
                        Error::Incomplete {
                            mut expecteds
                        } => Error::Incomplete {
                            expecteds: {
                                expecteds.push(expected);

                                expecteds
                            }
                        },
                        err => err,
                    })?;
                l += n;

                let m = position + l + 1;
                match self.chars.get(m) {
                    Some(&found) if found == expected => self
                        .parse(m + 1,)
                        .map(|n| n + l + 2),
                    Some(&found) => Err(Error::Corrupted {
                        position: m,
                        expected,
                        found,
                    }),
                    None => Err(Error::Incomplete { expecteds: vec![expected] }),
                }
            }
        }
    }
}

#[allow(unused_variables)]
#[aoc_generator(day10)]
fn input_generator(input: &str) -> Input {
    Input {
        lines: input.lines()
            .map(|line| Line {
                chars: line.chars()
                    .map(|char| char.into())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    }
}

type Output = usize;

#[allow(unused_variables)]
#[aoc(day10, part1)]

fn part1(input: &Input) -> Output {
    input.parsed_lines().iter()
        .filter_map(|res| match res {
            Err(err @ Error::Corrupted { position, expected, found }) => Some(err.score()),
            _ => None,
        })
        .sum()
}

#[allow(unused_variables)]
#[aoc(day10, part2)]

fn part2(input: &Input) -> Output {
    let mut scores = input.parsed_lines().iter()
        .filter_map(|res| match res {
            Err(err @ Error::Incomplete { expecteds }) => Some(err.score()),
            _ => None,
        })
        .collect::<Vec<_>>();

    scores.sort();

    scores[scores.len() / 2]
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), 288957);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), 26397);
    }

    #[test]
    fn parse_line() {
        assert_eq!(Line::from_str("(").unwrap().parse(0), Err(Error::Incomplete {
            expecteds: vec![ChunkChar::Right(ChunkCharType::Parenthesis)],
        }));
        assert_eq!(Line::from_str("([").unwrap().parse(0), Err(Error::Incomplete {
            expecteds: vec![
                ChunkChar::Right(ChunkCharType::SquareBracket),
                ChunkChar::Right(ChunkCharType::Parenthesis),
            ],
        }));
        assert_eq!(Line::from_str("").unwrap().parse(0), Ok(0));
        assert_eq!(Line::from_str("()").unwrap().parse(0), Ok(2));
        assert_eq!(Line::from_str("([])").unwrap().parse(0), Ok(4));
        assert_eq!(Line::from_str("([])").unwrap().parse(0), Ok(4));
        assert_eq!(Line::from_str("()[]").unwrap().parse(0), Ok(4));
        assert_eq!(Line::from_str("()[<{}>]").unwrap().parse(0), Ok(8));
        assert_eq!(Line::from_str("(]").unwrap().parse(0), Err(Error::Corrupted {
            expected: ChunkChar::Right(ChunkCharType::Parenthesis),
            found: ChunkChar::Right(ChunkCharType::SquareBracket),
            position: 1,
        }));
    }

    #[test]
    fn parse_input_line() {
        assert_eq!(Line::from_str("([<{)]>}"), Ok(Line {
            chars: vec![
                ChunkChar::Left(ChunkCharType::Parenthesis),
                ChunkChar::Left(ChunkCharType::SquareBracket),
                ChunkChar::Left(ChunkCharType::AngleBracket),
                ChunkChar::Left(ChunkCharType::CurlyBracket),
                ChunkChar::Right(ChunkCharType::Parenthesis),
                ChunkChar::Right(ChunkCharType::SquareBracket),
                ChunkChar::Right(ChunkCharType::AngleBracket),
                ChunkChar::Right(ChunkCharType::CurlyBracket),
            ],
        }));
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input()), Input {
            lines: vec![
                Line::from_str("[({(<(())[]>[[{[]{<()<>>").unwrap(),
                Line::from_str("[(()[<>])]({[<{<<[]>>(").unwrap(),
                Line::from_str("{([(<{}[<>[]}>{[]{[(<()>").unwrap(),
                Line::from_str("(((({<>}<{<{<>}{[]{[]{}").unwrap(),
                Line::from_str("[[<[([]))<([[{}[[()]]]").unwrap(),
                Line::from_str("[{[{({}]{}}([{[{{{}}([]").unwrap(),
                Line::from_str("{<[[]]>}<{[{[{[]{()[[[]").unwrap(),
                Line::from_str("[<(<(<(<{}))><([]([]()").unwrap(),
                Line::from_str("<{([([[(<>()){}]>(<<{{").unwrap(),
                Line::from_str("<{([{{}}[<[[[<>{}]]]>[]]").unwrap(),
            ],
        });
    }
}
