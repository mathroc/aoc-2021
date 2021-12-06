use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct LanternFishes {
    fishes: Vec<isize>,
}

#[derive(Debug, Default)]
struct Calculator {
    cache: HashMap::<isize, usize>,
}

impl Calculator {
    pub fn count(&mut self, fish: isize, days: isize) -> usize {
        let key = fish - days;
        match self.cache.get(&key) {
            Some(&count) => count,
            None => {
                let mut count = 1;
                let mut d = fish;
                while d < days {
                    d += 7;
                    count += self.count(d + 2, days);
                }
                self.cache.insert(key, count);
                count
            }
        }
    }
}

impl LanternFishes {
    pub fn count(&self, days: usize) -> usize {
        let mut calculator = Calculator::default();

        self.fishes.iter()
            .map(|fish| calculator.count(*fish, days as isize))
            .sum()
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> LanternFishes {
    LanternFishes {
        fishes: input.split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>(),
    }
}

#[aoc(day6, part1)]

fn part1(fishes: &LanternFishes) -> usize {
    fishes.count(80)
}

#[aoc(day6, part2)]

fn part2(fishes: &LanternFishes) -> usize {
    fishes.count(256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE_RAW_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(EXEMPLE_RAW_INPUT)), 26984457539);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(EXEMPLE_RAW_INPUT)), 5934);
    }

    #[test]
    fn calculator() {
        let mut calculator = Calculator::default();
        assert_eq!(calculator.count(0, 0), 1);
        assert_eq!(calculator.count(0, 1), 2);
        assert_eq!(calculator.count(0, 2), 2);
        assert_eq!(calculator.count(0, 3), 2);
        assert_eq!(calculator.count(0, 4), 2);
        assert_eq!(calculator.count(0, 5), 2);
        assert_eq!(calculator.count(0, 6), 2);
        assert_eq!(calculator.count(0, 7), 2);
        assert_eq!(calculator.count(0, 8), 3);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(EXEMPLE_RAW_INPUT), LanternFishes { fishes: vec![3,4,3,1,2] });
    }
}
