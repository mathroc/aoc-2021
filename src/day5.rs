use std::cmp::Ordering;
use std::collections::HashMap;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug)]
#[display("{x},{y}")]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{start} -> {end}")]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    pub fn dx(&self) -> i32 {
        match self.start.x.cmp(&self.end.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    pub fn dy(&self) -> i32 {
        match self.start.y.cmp(&self.end.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }
}

impl IntoIterator for &Line {
    type Item = Coord;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let dx = self.dx();
        let dy = self.dy();

        let (mut x, mut y) = (self.start.x, self.start.y);

        let mut points = vec![Coord { x, y }];

        while (x, y) != (self.end.x, self.end.y) {
            x = ((x as i32) + dx) as usize;
            y = ((y as i32) + dy) as usize;

            points.push(Coord { x, y });
        }

        points.into_iter()
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Line> {
    input.lines()
        .map(|line| line.parse::<Line>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day5, part1)]

fn part1(lines: &[Line]) -> usize {
    lines.iter()
        .filter(|Line {
            start: Coord { x: x1, y: y1 },
            end: Coord { x: x2, y: y2 },
        }| x1 == x2 || y1 == y2)
        .fold(HashMap::new(), |mut counts, line| {
            for point in line.into_iter() {
                counts.entry(point)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            counts
        })
        .iter()
        .filter(|(_, &count)| count > 1)
        .count()

}

#[aoc(day5, part2)]

fn part2(lines: &[Line]) -> usize {
    lines.iter()
        .fold(HashMap::new(), |mut counts, line| {
            for point in line.into_iter() {
                counts.entry(point)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            counts
        })
        .iter()
        .filter(|(_, &count)| count > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE_RAW_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(EXEMPLE_RAW_INPUT)), 12);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(EXEMPLE_RAW_INPUT)), 5);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(EXEMPLE_RAW_INPUT), vec![
            Line { start: Coord { x: 0, y: 9 }, end: Coord { x: 5, y: 9 }},
            Line { start: Coord { x: 8, y: 0 }, end: Coord { x: 0, y: 8 }},
            Line { start: Coord { x: 9, y: 4 }, end: Coord { x: 3, y: 4 }},
            Line { start: Coord { x: 2, y: 2 }, end: Coord { x: 2, y: 1 }},
            Line { start: Coord { x: 7, y: 0 }, end: Coord { x: 7, y: 4 }},
            Line { start: Coord { x: 6, y: 4 }, end: Coord { x: 2, y: 0 }},
            Line { start: Coord { x: 0, y: 9 }, end: Coord { x: 2, y: 9 }},
            Line { start: Coord { x: 3, y: 4 }, end: Coord { x: 1, y: 4 }},
            Line { start: Coord { x: 0, y: 0 }, end: Coord { x: 8, y: 8 }},
            Line { start: Coord { x: 5, y: 5 }, end: Coord { x: 8, y: 2 }},
        ]);
    }
}
