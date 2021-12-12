use std::collections::{HashMap,HashSet};

#[derive(Debug, PartialEq)]
struct Input {
    map: HashMap<String, HashSet<String>>
}

impl Input {
    fn add_link(&mut self, p1: &str, p2: &str) {
        if p1 != "end" && p2 != "start" {
            self.map.entry(p1.to_string()).or_insert(HashSet::new()).insert(p2.to_string());
        }
    }

    fn new(input: &str) -> Self {
        let mut map = Self { map: HashMap::new() };

        for line in input.lines() {
            let (p1, p2) = line.split_once('-').unwrap();
            map.add_link(p1, p2);
            map.add_link(p2, p1);
        }

        map
    }

    pub fn paths(&self, small_twice: bool) -> Vec<Vec<String>> {
        self._paths(vec!["start".to_string()], HashSet::new(), small_twice)
    }

    fn _paths(&self, current: Vec<String>, blacklist: HashSet<String>, small_twice: bool) -> Vec<Vec<String>> {
        let last = current.last().unwrap();

        if last == "end" {
            return vec![current];
        }

        let b = if !small_twice {
            blacklist.clone()
        } else {
            HashSet::new()
        };

        self.map.get(last).unwrap()
            .difference(&b)
            .flat_map(|p| {
                let mut path = current.clone();
                path.push(p.clone());

                let mut b = blacklist.clone();

                let mut s = small_twice;

                if p.chars().next().unwrap().is_lowercase() {
                    if b.contains(p) {
                        s = false;
                    } else {
                        b.insert(p.clone());
                    }
                }

                self._paths(path, b, s)
            })
            .collect::<Vec<_>>()
    }
}

#[allow(unused_variables)]
#[aoc_generator(day12)]
fn input_generator(input: &str) -> Input {
    Input::new(input)
}

type Output = usize;

#[allow(unused_variables)]
#[aoc(day12, part1)]

fn part1(input: &Input) -> Output {
    input.paths(false).len()
}

#[allow(unused_variables)]
#[aoc(day12, part2)]

fn part2(input: &Input) -> Output {
    input.paths(true).len()
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input(n: usize) -> &'static str {
        match n {
            1 => "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
            2 => "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
            3 => "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
            _ => panic!(),
        }
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input(1))), 36);
        assert_eq!(part2(&input_generator(exemple_raw_input(2))), 103);
        assert_eq!(part2(&input_generator(exemple_raw_input(3))), 3509);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input(1))), 10);
        assert_eq!(part1(&input_generator(exemple_raw_input(2))), 19);
        assert_eq!(part1(&input_generator(exemple_raw_input(3))), 226);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input(1)), Input {
            map: HashMap::from([
                ("start".to_string(), HashSet::from(["A".to_string(), "b".to_string()])),
                ("A".to_string(), HashSet::from(["c".to_string(), "b".to_string(), "end".to_string()])),
                ("b".to_string(), HashSet::from(["A".to_string(), "d".to_string(), "end".to_string()])),
                ("c".to_string(), HashSet::from(["A".to_string()])),
                ("d".to_string(), HashSet::from(["b".to_string()])),
            ]),
        });
    }
}
