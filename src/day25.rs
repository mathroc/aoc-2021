use std::{str::FromStr, convert::Infallible, fmt::Debug};

use parse_display::{Display, FromStr};

#[derive(PartialEq, Clone)]
struct Input {
    seafloor: Vec<Vec<Option<SeaCucumber>>>,
}

#[derive(PartialEq, Display, FromStr, Clone, Copy)]
enum SeaCucumber {
    #[display("v")]
    South,
    #[display(">")]
    East,
}

#[allow(unused_variables)]
#[aoc_generator(day25)]
fn input_generator(input: &str) -> Input {
    input.parse::<Input>().unwrap()
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            seafloor: s.lines()
                .map(|line| line.chars().map(|c| c.to_string().parse::<SeaCucumber>().ok()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        })
    }
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple_input() {
    assert_eq!(input_generator(".>
vv"), Input {
    seafloor: vec![
        vec![None, Some(SeaCucumber::East)],
        vec![Some(SeaCucumber::South), Some(SeaCucumber::South)],
    ]
});
}

impl Debug for SeaCucumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::South => write!(f, "v"),
            Self::East => write!(f, ">"),
        }
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}", 
            self.seafloor.iter()
                .map(|line| line.iter()
                    .map(|sea_cucumber| sea_cucumber.map_or(".".to_string(), |sea_cucumber| format!("{:?}", sea_cucumber)))
                    .collect::<Vec<_>>()
                    .concat(),
                )   
                .collect::<Vec<_>>()
                .join("\n")
                .as_str(),
        )
        // f.debug_struct("Input").field("seafloor", &self.seafloor).finish()
    }
}

#[test]
fn test_debug() {
    assert_eq!(format!("{:?}", input_generator(".>
vv")), ".>
vv");
}

type Output = usize;

impl Input {
    pub fn step(&mut self) -> bool {
        let height = self.seafloor.len();
        let width = self.seafloor[0].len();

        let mut changes = false;

        for direction in [SeaCucumber::East, SeaCucumber::South] {
            let (row_count, col_count) = match direction {
                SeaCucumber::East => (height, width),
                SeaCucumber::South => (width, height),
            };

            for row in 0..row_count {
                let mut moves = vec![];

                for col in 0..col_count {
                    let from = match direction {
                        SeaCucumber::East => (row, col),
                        SeaCucumber::South => (col, row),
                    };
                    if let Some(d) = self.seafloor[from.0][from.1] {
                        if d == direction {
                            let target = (col + 1) % col_count;
                            let to = match direction {
                                SeaCucumber::East => (row, target),
                                SeaCucumber::South => (target, row),
                            };
                                
                            if self.seafloor[to.0][to.1].is_none() {
                                moves.push((from, to));
                            }
                        }
                    }
                }
                
                for (from, to) in moves {
                    self.seafloor[from.0][from.1] = None;
                    self.seafloor[to.0][to.1] = Some(direction);

                    changes = true;
                }
            }
        }

        changes
    }
}

#[test]
fn test_step() {
    let mut input = input_generator(">>");

    assert!(!input.step());
    assert_eq!(format!("{:?}", input), ">>");

    let mut input = input_generator(">>.");

    assert!(input.step());
    assert_eq!(format!("{:?}", input), ">.>");

    assert!(input.step());
    assert_eq!(format!("{:?}", input), ".>>");

    assert!(input.step());
    assert_eq!(format!("{:?}", input), ">>.");


    let mut input = input_generator("v
v
.");

    assert!(input.step());
    assert_eq!(format!("{:?}", input), "v
.
v");

    assert!(input.step());
    assert_eq!(format!("{:?}", input), ".
v
v");

    assert!(input.step());
    assert_eq!(format!("{:?}", input), "v
v
.");
}

#[allow(unused_variables)]
#[aoc(day25, part1)]

fn part1(input: &Input) -> Output {
    let mut step_count = 0;
    let mut input = input.clone();
    
    loop {
        let changes = *&mut input.step();

        step_count += 1;

        if !changes {
            return step_count;
        }
    }
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple() {
    assert_eq!(part1(&input_generator(exemple_raw_input())), 58);
}

#[allow(unused_variables)]
#[aoc(day25, part2)]

fn part2(input: &Input) -> Output {
    todo!()
}

#[allow(unreachable_code)]
#[test]
fn part2_provided_exemple() {
    assert_eq!(part2(&input_generator(exemple_raw_input())), todo!());
}

#[allow(unreachable_code)]
#[allow(dead_code)]
fn exemple_raw_input() -> &'static str {
    "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
}
