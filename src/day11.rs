use std::{collections::{HashMap, HashSet}, fmt::Display};
use std::cmp::min;

#[derive(PartialEq, Clone)]
struct Input {
    octopuses: HashMap<Coord, usize>,
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(0..10).map(|x|
            (0..10)
                .map(|y| self.octopuses.get(&Coord(x, y)).unwrap().to_string())
                .collect::<Vec<_>>()
                .join("")
        ).collect::<Vec<_>>().join("\n"))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(usize, usize);

#[allow(unused_variables)]
#[aoc_generator(day11)]
fn input_generator(input: &str) -> Input {
    Input {
        octopuses: input.lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .map(|n| usize::try_from(n).unwrap())
            .enumerate()
            .map(|(n, power_level)| (Coord::from(n), power_level))
            .collect::<HashMap<_, _>>(),
    }
}

type Output = usize;

impl Coord {
    pub fn from(n: usize) -> Self {
        Coord(n / 10, n % 10)
    }

    pub fn neighbors(&self) -> Vec<Coord> {
        let mut neighbors = vec![];

        let xmin = if self.0 == 0 { 0 } else { self.0 - 1};
        let ymin = if self.1 == 0 { 0 } else { self.1 - 1};

        for x in xmin..=min(9, self.0 + 1) {
            for y in ymin..=min(9, self.1 + 1) {
                if x != self.0 || y != self.1 {
                    neighbors.push(Coord(x, y));
                }
            }
        }

        neighbors
    }
}

impl Input {
    pub fn step(&mut self) -> usize {
        let mut flashes: HashSet<Coord> = HashSet::new();

        for n in 0..100 {
            self.inc(Coord::from(n), &mut flashes);
        }

        flashes.len()
    }

    pub fn inc(&mut self, pos: Coord, flashes: &mut HashSet<Coord>) {
        if flashes.contains(&pos) {
            return;
        }

        self.octopuses.entry(pos).and_modify(|power_level| {
            *power_level += 1;
            *power_level %= 10;
        });

        if *self.octopuses.get(&pos).unwrap() == 0 && flashes.insert(pos) {
            for p in pos.neighbors() {
                self.inc(p, flashes)
            }
        }
    }
}

#[allow(unused_variables)]
#[aoc(day11, part1)]

fn part1(input: &Input) -> Output {
    let mut i = input.clone();
    let mut flashes = 0;

    for step in 0..100 {
        flashes += i.step();
    }

    flashes
}

#[allow(unused_variables)]
#[aoc(day11, part2)]

fn part2(input: &Input) -> Output {
    let mut i = input.clone();
    let mut step = 1;

    while i.step() != 100 {
        step += 1;
    }

    step
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), 195);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), 1656);
    }

    #[test]
    fn input_print() {
        assert_eq!(format!("{}", input_generator(exemple_raw_input())), exemple_raw_input());
    }

    #[test]
    fn coord_neigbors() {
        assert_eq!(Coord(1, 1).neighbors(), vec![
            Coord(0, 0),
            Coord(0, 1),
            Coord(0, 2),
            Coord(1, 0),
            Coord(1, 2),
            Coord(2, 0),
            Coord(2, 1),
            Coord(2, 2),
        ]);
        assert_eq!(Coord(0, 0).neighbors(), vec![
            Coord(0, 1),
            Coord(1, 0),
            Coord(1, 1),
        ]);
        assert_eq!(Coord(9, 9).neighbors(), vec![
            Coord(8, 8),
            Coord(8, 9),
            Coord(9, 8),
        ]);
    }
}
