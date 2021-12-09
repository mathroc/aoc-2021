use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Input {
    heights: Vec<Vec<u32>>,
}

impl Input {
    pub fn at(&self, pos: (usize, usize)) -> u32 {
        self.heights[pos.0][pos.1]
    }

    pub fn min_around(&self, pos: (usize, usize)) -> u32 {
        *self.heights_around(pos)
            .iter()
            .min()
            .unwrap()
    }

    fn heights_around(&self, pos: (usize, usize)) -> Vec<u32> {
        self.around(pos)
            .iter()
            .map(|&pos| self.at(pos))
            .collect::<Vec<_>>()
    }

    fn around(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut around = vec![];
        let (row, column) = pos;

        if row > 0 {
            around.push((row - 1, column));
        }

        let rows = self.heights.len();

        if row < rows - 1 {
            around.push((row + 1, column));
        }

        if column > 0 {
            around.push((row, column - 1));
        }

        let columns = self.heights[0].len();

        if column < columns - 1 {
            around.push((row, column + 1));
        }

        around
    }

    pub fn low_points(&self) -> Vec<(usize, usize)> {
        self.heights
            .iter()
            .enumerate()
            .flat_map(|(row, line)| line.iter()
                .enumerate()
                .filter_map(move |(column, &height)| {
                    let pos = (row, column);

                    if self.min_around(pos) > height {
                        Some(pos)
                    } else {
                        None
                    }
                })
            )
            .collect::<Vec<_>>()
    }

    fn basin(&self, pos: (usize, usize)) -> usize {
        let mut basin: HashSet<(usize, usize)> = HashSet::from([pos]);

        let mut out = basin.clone();
        while !out.is_empty() {
            out = out
                .into_iter()
                .flat_map(|pos| self.around(pos))
                .filter(|&pos| self.at(pos) != 9 && !basin.contains(&pos))
                .collect();

            basin.extend(out.clone());
        }

        basin.len()
    }

    pub fn basins_sizes(&self) -> Vec<usize> {
        self.low_points()
            .iter()
            .map(|&pos| self.basin(pos))
            .collect::<Vec<_>>()
    }
}

#[allow(unused_variables)]
#[aoc_generator(day9)]
fn input_generator(input: &str) -> Input {
    Input {
        heights: input.lines()
            .map(|line| line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
            )
            .collect::<Vec<_>>(),
    }
}

#[allow(unused_variables)]
#[aoc(day9, part1)]

fn part1(input: &Input) -> u32 {
    input.low_points()
        .iter()
        .map(|(row, column)| input.heights[*row][*column] + 1)
        .sum()
}

#[allow(unused_variables)]
#[aoc(day9, part2)]

fn part2(input: &Input) -> usize {
    let mut sizes = input.basins_sizes();

    sizes.sort();

    sizes.iter().rev().take(3).product()
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        "2199943210
3987894921
9856789892
8767896789
9899965678"
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), 1134);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), 15);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input()), Input {
            heights: vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ],
        });
    }
}
