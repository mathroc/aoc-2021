use std::{collections::{HashMap, BinaryHeap}, cmp::Ordering};

#[derive(Debug, PartialEq)]
struct Input {
    risks: Vec<u32>,
    width: usize,
    height: usize,
    unfolded: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct PathRisk {
    path_end: usize,
    risk: u32,
}

impl PartialOrd for PathRisk {
    fn lt(&self, other: &Self) -> bool {
        !self.risk.lt(&other.risk)
    }

    fn le(&self, other: &Self) -> bool {
        !self.risk.le(&other.risk)
    }

    fn gt(&self, other: &Self) -> bool {
        !self.risk.gt(&other.risk)
    }

    fn ge(&self, other: &Self) -> bool {
        !self.risk.ge(&other.risk)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.risk.partial_cmp(&other.risk).map(Ordering::reverse)
    }
}

impl Ord for PathRisk {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk.cmp(&other.risk).reverse()
    }
}

#[allow(unused_variables)]
#[aoc_generator(day15)]
fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    Input {
        risks: input.lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect::<Vec<_>>(),
        width: input.lines().next().unwrap().len(),
        height: input.lines().collect::<Vec<_>>().len(),
        unfolded: 1,
    }
}

impl Input {
    pub fn unfolded_width(&self) -> usize {
        self.unfolded * self.width
    }

    pub fn unfolded_height(&self) -> usize {
        self.unfolded * self.height
    }

    pub fn end(&self) -> usize {
        self.unfolded_height() * self.unfolded_width() - 1
    }

    pub fn at(&self, pos: usize) -> u32 {
        let pos = self.pos(pos);
        let (x, xf) = (pos.0 % self.width, pos.0 / self.width);
        let (y, yf) = (pos.1 % self.height, pos.1 / self.height);

        let v = self.risks[x + y * self.width] + u32::try_from(xf).unwrap() + u32::try_from(yf).unwrap();

        if v > 9 {
            v % 10 + 1
        } else {
            v
        }
    }

    pub fn shortest_path_length(&self) -> u32 {
        let mut shortest_paths: HashMap<usize, u32> = HashMap::from([(0, 0)]);

        let end = self.end();
        let mut buffer = BinaryHeap::from([PathRisk {
            path_end: 0,
            risk: 0,
        }]);

        while let Some(p) = buffer.pop() {
            let i = p.path_end;
            let w = *shortest_paths.get(&i).unwrap();

            for neighbor in self.neighbor_of(i) {
                let risk = self.at(neighbor);

                let e = shortest_paths
                    .entry(neighbor)
                    .or_insert(u32::MAX);

                if *e > w + risk {
                    buffer.push(PathRisk {
                        path_end: neighbor,
                        risk: w + risk,
                    });
                    *e = w + risk;
                }
            }
        }

        *shortest_paths.get(&end).unwrap()
    }

    fn pos(&self, i: usize) -> (usize, usize) {
        (i % self.unfolded_width(), i / self.unfolded_width())
    }

    fn neighbor_of(&self, i: usize) -> Vec<usize> {
        let (x, y) = self.pos(i);
        let mut neighbors = vec![];

        if x > 0 {
            neighbors.push(self.unfolded_width() * y + x - 1);
        }

        if x < self.unfolded_width() - 1 {
            neighbors.push(self.unfolded_width() * y + x + 1);
        }

        if y > 0 {
            neighbors.push(self.unfolded_width() * y + x - self.unfolded_width());
        }

        if y < self.unfolded_height() - 1 {
            neighbors.push(self.unfolded_width() * y + x + self.unfolded_width());
        }

        neighbors
    }

    fn unfold(&self, times: usize) -> Self {
        Self {
            risks: self.risks.clone(),
            width: self.width,
            height: self.height,
            unfolded: self.unfolded * times,
        }
    }
}

type Output = u32;

#[allow(unused_variables)]
#[aoc(day15, part1)]

fn part1(input: &Input) -> Output {
    input.shortest_path_length()
}

#[allow(unused_variables)]
#[aoc(day15, part2)]

fn part2(input: &Input) -> Output {
    input.unfold(5).shortest_path_length()
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), 315);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), 40);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input()), Input {
            risks: vec![
                1, 1, 6, 3, 7, 5, 1, 7, 4, 2,
                1, 3, 8, 1, 3, 7, 3, 6, 7, 2,
                2, 1, 3, 6, 5, 1, 1, 3, 2, 8,
                3, 6, 9, 4, 9, 3, 1, 5, 6, 9,
                7, 4, 6, 3, 4, 1, 7, 1, 1, 1,
                1, 3, 1, 9, 1, 2, 8, 1, 3, 7,
                1, 3, 5, 9, 9, 1, 2, 4, 2, 1,
                3, 1, 2, 5, 4, 2, 1, 6, 3, 9,
                1, 2, 9, 3, 1, 3, 8, 5, 2, 1,
                2, 3, 1, 1, 9, 4, 4, 5, 8, 1,
            ],
            width: 10,
            height: 10,
            unfolded: 1,
        });
    }
}
