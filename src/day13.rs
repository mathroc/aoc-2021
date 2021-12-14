use std::{collections::{HashSet, HashMap}, fmt::Display};
use std::cmp::max;

use parse_display::FromStr;

#[derive(PartialEq, Debug, FromStr)]
#[display("{x},{y}")]
struct Dot {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, FromStr, Copy, Clone)]
enum Fold {
    #[display("fold along x={0}")]
    AlongX(usize),
    #[display("fold along y={0}")]
    AlongY(usize),
}

#[derive(Debug, PartialEq, Clone)]
struct Paper {
    dots: HashMap<usize, HashSet<usize>>,
    width: usize,
    height: usize,
}

impl Paper {
    pub fn dots(&self) -> Vec<Dot> {
        self.dots
            .iter()
            .map(|(x, ys)| ys
                .iter()
                .map(|y| Dot { x: *x, y: *y })
            )
            .flatten()
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    paper: Paper,
    folds: Vec<Fold>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(0..=self.height)
            .map(|y| {
                (0..=self.width)
                    .map(|x| self.dots
                        .get(&x)
                        .map(|ys| ys.contains(&y))
                        .unwrap_or(false)
                    )
                    .map(|exists| if exists { "#" } else { "." })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

impl Input {
    pub fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let (dots, width, height) = lines.by_ref()
            .take_while(|&line| !line.is_empty())
            .map(|line| line.parse::<Dot>().unwrap())
            .fold(
                (HashMap::new(), 0, 0),
                |(mut dots, width, height): (HashMap<usize, HashSet<_>>, _, _), dot| {
                    dots.entry(dot.x).or_default().insert(dot.y);

                    (dots, max(width, dot.x), max(height, dot.y))
                },
            );

        let folds = lines
            .map(|str| str.parse::<Fold>().unwrap())
            .collect::<Vec<_>>();

        Self {
            paper: Paper {
                dots,
                width,
                height,
            },
            folds,
        }
    }
}

#[allow(unused_variables)]
#[aoc_generator(day13)]
fn input_generator(input: &str) -> Input {
    Input::from(input)
}

type Output = usize;


impl Paper {
    pub fn fold(&self, instruction: Fold) -> Self {
        let (width, height) = match instruction {
            Fold::AlongX(x) => (x - 1, self.height),
            Fold::AlongY(n) => (self.width, n - 1),
        };

        let mut dots: HashMap<usize, HashSet<usize>> = HashMap::new();

        for (x, ys) in &self.dots {
            for y in ys {
                let (nx, ny) = match instruction {
                    Fold::AlongX(n) => (if x > &n { 2 * n - x } else { *x }, *y),
                    Fold::AlongY(n) => (*x, if y > &n { 2 * n - y } else { *y }),
                };

                dots.entry(nx).or_default().insert(ny);
            }
        }

        Self {
            dots,
            width,
            height,
        }
    }
}

#[allow(unused_variables)]
#[aoc(day13, part1)]
fn part1(input: &Input) -> Output {
    input.paper.fold(input.folds[0]).dots().len()
}

#[allow(unused_variables)]
#[aoc(day13, part2)]

fn part2(input: &Input) -> String {
    format!(
        "{}",
        input.folds
            .iter()
            .fold(
                input.paper.clone(),
                |paper, instruction| paper.fold(*instruction),
            )
    )
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), "#####
#...#
#...#
#...#
#####
.....
.....");
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), 17);
    }

    #[test]
    fn count_dots() {
        assert_eq!(input_generator(exemple_raw_input()).paper.dots().len(), 18);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input()), Input {
            paper: Paper {
                dots: HashMap::from([
                    (0, HashSet::from([3, 13, 14])),
                    (1, HashSet::from([10])),
                    (2, HashSet::from([14])),
                    (3, HashSet::from([0, 4])),
                    (4, HashSet::from([1, 11])),
                    (6, HashSet::from([0, 10, 12])),
                    (8, HashSet::from([4, 10])),
                    (9, HashSet::from([0, 10])),
                    (10, HashSet::from([4, 12])),
                ]),
                width: 10,
                height: 14,
            },
            folds: vec![
                Fold::AlongY(7),
                Fold::AlongX(5),
            ],
        });
    }

    #[test]
    fn provide_exemple_display() {
        assert_eq!(
            format!("{}", input_generator(exemple_raw_input()).paper),
            "...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........"
        )
    }
}
