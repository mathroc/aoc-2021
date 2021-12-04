use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    pub numbers: BTreeMap<usize, (usize, usize)>,
    pub numbers_remaining_in_rows: Vec<usize>,
    pub numbers_remaining_in_columns: Vec<usize>,
}

impl Board {
    pub fn check(&mut self, n: &usize) -> bool {
        match &self.numbers.remove(n) {
            None => false,
            Some((row, column)) => {
                self.numbers_remaining_in_rows[*row] -= 1;
                self.numbers_remaining_in_columns[*column] -= 1;

                self.numbers_remaining_in_rows[*row] == 0 || self.numbers_remaining_in_columns[*column] == 0
            }
        }
    }

    pub fn unmarked_numbers_sum(&self) -> usize {
        self.numbers.keys().sum()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Input {
    pub numbers: Vec<usize>,
    pub boards: Vec<Board>,
}

impl Input {
    pub fn part1(&mut self) -> usize {
        for n in &self.numbers {
            for board in &mut self.boards {
                if board.check(n) {
                    return n * board.unmarked_numbers_sum();
                }
            }
        }
        0
    }

    pub fn part2(&mut self) -> usize {
        for n in &self.numbers {
            let mut i = 0;

            while i < self.boards.len() {
                if self.boards[i].check(n) {
                    if self.boards.len() == 1 {
                        return n * self.boards[i].unmarked_numbers_sum();
                    }

                    self.boards.remove(i);
                } else {
                    i += 1;
                }
            }
        }
        0
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let numbers = lines.next().unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards = vec![];

    while lines.next().is_some() {
        let mut board_numbers: BTreeMap<usize, (usize, usize)> = BTreeMap::new();

        for row in 0..5 {
            for (column, s) in lines.next().unwrap().split_whitespace().enumerate() {
                board_numbers.insert(s.parse().unwrap(), (row, column));
            }
        }

        boards.push(Board {
            numbers: board_numbers,
            numbers_remaining_in_columns: vec![5; 5],
            numbers_remaining_in_rows: vec![5; 5],
        });
    }

    Input {
        numbers,
        boards,
    }
}

#[aoc(day4, part1)]

pub fn part1(input: &Input) -> usize {
    input.clone().part1()
}

#[aoc(day4, part2)]

pub fn part2(input: &Input) -> usize {
    input.clone().part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE_RAW_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(EXEMPLE_RAW_INPUT)), 1924);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(EXEMPLE_RAW_INPUT)), 4512);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(EXEMPLE_RAW_INPUT), Input {
            numbers: vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
            boards: vec![Board {
                numbers: BTreeMap::from([
                    (22, (0, 0)),
                    (13, (0, 1)),
                    (17, (0, 2)),
                    (11, (0, 3)),
                    (0, (0, 4)),
                    (8, (1, 0)),
                    (2, (1, 1)),
                    (23, (1, 2)),
                    (4, (1, 3)),
                    (24, (1, 4)),
                    (21, (2, 0)),
                    (9, (2, 1)),
                    (14, (2, 2)),
                    (16, (2, 3)),
                    (7, (2, 4)),
                    (6, (3, 0)),
                    (10, (3, 1)),
                    (3, (3, 2)),
                    (18, (3, 3)),
                    (5, (3, 4)),
                    (1, (4, 0)),
                    (12, (4, 1)),
                    (20, (4, 2)),
                    (15, (4, 3)),
                    (19, (4, 4)),
                ]),
                numbers_remaining_in_columns: vec![5; 5],
                numbers_remaining_in_rows: vec![5; 5],
            }, Board {
                numbers: BTreeMap::from([
                    (3, (0, 0)),
                    (15, (0, 1)),
                    (0, (0, 2)),
                    (2, (0, 3)),
                    (22, (0, 4)),
                    (9, (1, 0)),
                    (18, (1, 1)),
                    (13, (1, 2)),
                    (17, (1, 3)),
                    (5, (1, 4)),
                    (19, (2, 0)),
                    (8, (2, 1)),
                    (7, (2, 2)),
                    (25, (2, 3)),
                    (23, (2, 4)),
                    (20, (3, 0)),
                    (11, (3, 1)),
                    (10, (3, 2)),
                    (24, (3, 3)),
                    (4, (3, 4)),
                    (14, (4, 0)),
                    (21, (4, 1)),
                    (16, (4, 2)),
                    (12, (4, 3)),
                    (6, (4, 4)),
                ]),
                numbers_remaining_in_columns: vec![5; 5],
                numbers_remaining_in_rows: vec![5; 5],
            }, Board {
                numbers: BTreeMap::from([
                    (14, (0, 0)),
                    (21, (0, 1)),
                    (17, (0, 2)),
                    (24, (0, 3)),
                    (4, (0, 4)),
                    (10, (1, 0)),
                    (16, (1, 1)),
                    (15, (1, 2)),
                    (9, (1, 3)),
                    (19, (1, 4)),
                    (18, (2, 0)),
                    (8, (2, 1)),
                    (23, (2, 2)),
                    (26, (2, 3)),
                    (20, (2, 4)),
                    (22, (3, 0)),
                    (11, (3, 1)),
                    (13, (3, 2)),
                    (6, (3, 3)),
                    (5, (3, 4)),
                    (2, (4, 0)),
                    (0, (4, 1)),
                    (12, (4, 2)),
                    (3, (4, 3)),
                    (7, (4, 4)),
                ]),
                numbers_remaining_in_columns: vec![5; 5],
                numbers_remaining_in_rows: vec![5; 5],
            }],
        });
    }
}
