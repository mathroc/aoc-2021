use std::{collections::HashMap, usize::MAX};

type PolymerElement = char;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PolymerElementsPair(PolymerElement, PolymerElement);

#[derive(Debug, PartialEq, Clone)]
struct Polymer {
    pairs: HashMap<PolymerElementsPair, usize>,
    count_start: PolymerElement,
}

impl Polymer {
    pub fn elements_counts(&self) -> HashMap<PolymerElement, usize> {
        self.pairs.iter()
            .fold(HashMap::from([(self.count_start, 1)]), |mut counts, (PolymerElementsPair(_, element), n)| {
                *counts.entry(*element).or_default() += n;

                counts
            },
        )
    }

    pub fn apply_rules(&mut self, rules: &HashMap<PolymerElementsPair, PolymerElement>, times: usize) {
        if times == 0 {
            return;
        }

        let mut pairs: HashMap<PolymerElementsPair, usize> = HashMap::new();

        for (pair, count) in &self.pairs {
            if let Some(element) = rules.get(pair) {
                *pairs.entry(PolymerElementsPair(pair.0, *element)).or_default() += count;
                *pairs.entry(PolymerElementsPair(*element, pair.1)).or_default() += count;
            } else {
                *pairs.entry(*pair).or_default() += count;
            }
        }

        self.pairs = pairs;

        self.apply_rules(rules, times - 1)
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    template: Polymer,
    pair_insertion_rules: HashMap<PolymerElementsPair, PolymerElement>,
}

impl PolymerElementsPair {
    pub fn from_chars(input: &[char]) -> Self {
        Self(input[0], input[1])
    }

    pub fn from_str(input: &str) -> Self {
        Self::from_chars(&input.chars().collect::<Vec<_>>())
    }
}

#[allow(unused_variables)]
#[aoc_generator(day14)]
fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let line = lines.next().unwrap();

    let template = Polymer {
        count_start: line.chars().nth(0).unwrap(),
        pairs: line
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .map(PolymerElementsPair::from_chars)
            .fold(HashMap::new(), |mut polymer, element| {
                *polymer.entry(element).or_default() += 1;
                polymer
            }),
    };
    lines.next();

    Input {
        template,
        pair_insertion_rules: lines.fold(
            HashMap::new(),
            |mut rules, line| {
                let (pair, char) = line.split_once(" -> ").unwrap();

                rules.insert(
                    PolymerElementsPair::from_str(pair),
                    char.chars().next().unwrap(),
                );

                rules
            },
        ),
    }
}

type Output = usize;

#[allow(unused_variables)]
#[aoc(day14, part1)]

fn part1(input: &Input) -> Output {
    let mut polymer = input.template.clone();

    polymer.apply_rules(&input.pair_insertion_rules, 10);

    let (min, max) = polymer.elements_counts()
        .iter()
        .fold((MAX, 0), |(min, max), (_element, count)| {
            (std::cmp::min(min, *count), std::cmp::max(max, *count))
        });

    max - min
}

#[allow(unused_variables)]
#[aoc(day14, part2)]

fn part2(input: &Input) -> Output {
    let mut polymer = input.template.clone();

    polymer.apply_rules(&input.pair_insertion_rules, 40);

    let (min, max) = polymer.elements_counts()
        .iter()
        .fold((MAX, 0), |(min, max), (_element, count)| {
            (std::cmp::min(min, *count), std::cmp::max(max, *count))
        });

    max - min
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), 2188189693529);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), 1588);
    }

    #[test]
    fn polymer_elements_counts() {
        let polymer = Polymer {
            count_start: 'N',
            pairs: HashMap::from([
                (PolymerElementsPair('N', 'N'), 1),
                (PolymerElementsPair('N', 'C'), 1),
                (PolymerElementsPair('C', 'B'), 1),
            ]),
        };

        assert_eq!(polymer.elements_counts(), HashMap::from([
            ('N', 2),
            ('C', 1),
            ('B', 1),
        ]));
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input()), Input {
            template: Polymer {
                count_start: 'N',
                pairs: HashMap::from([
                    (PolymerElementsPair('N', 'N'), 1),
                    (PolymerElementsPair('N', 'C'), 1),
                    (PolymerElementsPair('C', 'B'), 1),
                ]),
            },
            pair_insertion_rules: HashMap::from([
                (PolymerElementsPair('C', 'H'), 'B'),
                (PolymerElementsPair('H', 'H'), 'N'),
                (PolymerElementsPair('C', 'B'), 'H'),
                (PolymerElementsPair('N', 'H'), 'C'),
                (PolymerElementsPair('H', 'B'), 'C'),
                (PolymerElementsPair('H', 'C'), 'B'),
                (PolymerElementsPair('H', 'N'), 'C'),
                (PolymerElementsPair('N', 'N'), 'C'),
                (PolymerElementsPair('B', 'H'), 'H'),
                (PolymerElementsPair('N', 'C'), 'B'),
                (PolymerElementsPair('N', 'B'), 'B'),
                (PolymerElementsPair('B', 'N'), 'B'),
                (PolymerElementsPair('B', 'B'), 'N'),
                (PolymerElementsPair('B', 'C'), 'B'),
                (PolymerElementsPair('C', 'C'), 'N'),
                (PolymerElementsPair('C', 'N'), 'C'),
            ]),
        });
    }
}
