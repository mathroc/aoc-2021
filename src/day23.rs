use std::{str::FromStr, convert::Infallible, collections::{VecDeque, BinaryHeap, HashMap}, hash::Hash};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    rooms: [VecDeque<u32>; 4],
    hallway: [u32; 11],
    cost: u32,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            rooms: [
                VecDeque::from([]),
                VecDeque::from([]),
                VecDeque::from([]),
                VecDeque::from([]),
            ],
            hallway: [0; 11],
            cost: 0,
        }
    }
}

impl FromStr for Input {
    type Err = Infallible;

    // already count the cost of getting the amphipods out and back in 
    // so that we can just move them around in the hallway.
    // then we just have to move each amphipod in front of their rooms
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        
        lines.next();
        lines.next();
        
        let positions = lines
            .flat_map(|line| 
                line
                    .split(&['#', ' '][..])
                    .filter_map(|part| part.chars().next())
                    .map(|c| 10_u32.pow(c as u32 - 'A' as u32))
                    .enumerate()
                    .collect::<Vec<_>>(),
            )
            .collect::<Vec<_>>();
        
        Ok(Input::from_vec(positions))
    }
}

impl Hash for Input {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    {
        self.rooms.hash(state);
        self.hallway.hash(state);
    }
}

#[allow(unused_variables)]
#[aoc_generator(day23)]
fn input_generator(input: &str) -> Input {
    input.parse::<Input>().unwrap()
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple_input() {
    assert_eq!(input_generator(exemple_raw_input()), Input {
        rooms: [
            VecDeque::from([1, 10]),
            VecDeque::from([1000, 100]),
            VecDeque::from([100, 10]),
            VecDeque::from([1, 1000]),
        ],
        hallway: [0; 11],
        cost: 
            // cost of getting all amphipods out
            10 + 2 + 100 + 1000 * 2 + 10 + 100*2 + 1000 + 2
            // cost of getting all amphipods in
             + 3*1111,
    });

    assert_eq!(input_generator(exemple_raw_input()).part2(), Input {
        rooms: [
            VecDeque::from([1, 1000, 1000, 10]),
            VecDeque::from([1000, 10, 100, 100]),
            VecDeque::from([100, 1, 10, 10]),
            VecDeque::from([1, 100, 1, 1000]),
        ],
        hallway: [0; 11],
        cost: 22193,
    });
}

type Output = u32;

#[allow(unused_variables)]
#[aoc(day23, part1)]

fn part1(input: &Input) -> Output {
    input.best_solution().cost
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&(self.cost))

    }
}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Input {
    pub fn from_vec(positions: Vec<(usize, u32)>) -> Self {
        let mut input = Self::default();
        for (queue, value) in positions {
            input.push(queue, value);
        }   
        
        for n in 1..=input.rooms[2].len() {
            input.cost += n as u32 * 1111;
        }

        input
    }
    
    fn push(&mut self, room: usize, value: u32) {
        self.rooms[room].push_front(value);

        self.cost += self.rooms[room].len() as u32 * value;
    }

    fn part2(&self) -> Self {
        let positions = 
            self.rooms.iter().map(|v| v[1]).enumerate()
            .chain([1000, 100, 10, 1].iter().cloned().enumerate())
            .chain([1000, 10, 1, 100].iter().cloned().enumerate())
            .chain(self.rooms.iter().map(|v| v[0]).enumerate())
            .collect::<Vec<_>>();

        Self::from_vec(positions)
    }

    fn solved(&self) -> bool {
        self.rooms.iter().all(|slot| slot.is_empty()) && self.hallway.iter().all(|&v| v == 0)
    }

    fn move_cost(&self, start: usize) -> u32 {
        match start {
            2|4|6|8 => {
                let room = (start / 2) - 1;

                *self.rooms[room].iter().rev().next().unwrap_or(&0)
            }
            _ => self.hallway[start],
        }
    }

    fn move_options(&self, start: usize, move_cost: u32) -> Vec<(usize, u32)> {
        let mut options = vec![];

        if move_cost != 0 {
            let left = (0..start).rev().take_while(|&end| self.hallway[end] == 0);
            let right = (start+1..11).take_while(|&end| self.hallway[end] == 0);
    
            for end in left.chain(right) {
                match end {
                    2|4|6|8 => {
                        let room = (end / 2) - 1;
    
                        if !self.rooms[room].is_empty() || 10_u32.pow(room as u32) != move_cost {
                            continue;
                        }
    
                        return vec![(end, move_cost)];
                    }, 
                    _ => match start {
                        2|4|6|8 => options.push((end, move_cost)),
                        _ => {},
                    },
                }
            }
        }

        options
    }

    fn moves(&self, blacklist: &mut HashMap<Self, ()>) -> BinaryHeap<Self> {
        let mut moves = BinaryHeap::new();
        
        for start in 0..11 {
            let move_cost = self.move_cost(start);

            for (end, move_cost) in self.move_options(start, move_cost) {
                blacklist.entry(self.move_to(start, end, move_cost))
                    .or_insert_with_key(|key| {
                        moves.push(key.clone());
                    });
            }
        }

        moves
    }

    fn move_to(&self, start: usize, end: usize, move_cost: u32) -> Self {
        let moves = (std::cmp::max(start, end) - std::cmp::min(start, end)) as u32;
        assert_ne!(moves, 0);
        assert_ne!(move_cost, 0);
        let mut rooms = self.rooms.clone();
        let mut hallway = self.hallway;

        match start {
            2|4|6|8 => {
                rooms[(start / 2) - 1].pop_back();
            },
            _ => {
                hallway[start] = 0;
            },
        };

        match end {
            2|4|6|8 => {},
            _ => hallway[end] = move_cost,
        }

        Self {
            hallway,
            rooms,
            cost: self.cost + move_cost * moves,
        }
    }

    fn best_solution(&self) -> Self {
        let mut blacklist: HashMap<Self, ()> = HashMap::new();
        let mut moves = BinaryHeap::from([self.clone()]);
        
        let mut best_solution = Self::default();
        best_solution.cost = u32::MAX;

        while let Some(input) = moves.pop() {
            if input.cost > best_solution.cost {
                continue;
            }

            if input.solved() {
                best_solution = input;

                continue;
            }

            moves.append(&mut input.moves(&mut blacklist));
        }

        best_solution
    }
}

#[test]
fn test_solved() {
    assert_eq!(Input {
        ..Default::default()
    }.solved(), true);

    assert_eq!(Input {
        hallway: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ..Default::default()
    }.solved(), false);

    assert_eq!(Input {
        rooms: [
            VecDeque::from([10]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
        ],
        ..Default::default()
    }.solved(), false);
}

#[test]
fn test_input_moves() {
    assert_eq!(Input {
        ..Default::default()
    }.moves(&mut HashMap::new()).iter().cloned().collect::<Vec<Input>>(), vec![]);

    assert_eq!(Input {
        hallway: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        rooms: [
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
        ],
        ..Default::default()
    }.moves(&mut HashMap::new()).len(), 1);

    assert_eq!(Input {
        hallway: [1, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0],
        rooms: [
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
        ],
        ..Default::default()
    }.moves(&mut HashMap::new()).len(), 2);

    assert_eq!(Input {
        hallway: [0, 0, 0, 100, 0, 1, 0, 0, 0, 0, 0],
        rooms: [
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
        ],
        ..Default::default()
    }.moves(&mut HashMap::new()).len(), 0);

    assert_eq!(Input {
        rooms: [
            VecDeque::from([10]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
        ],
        ..Default::default()
    }.moves(&mut HashMap::new()).len(), 1);

    assert_eq!(Input {
        rooms: [
            VecDeque::from([10, 100]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
        ],
        ..Default::default()
    }.moves(&mut HashMap::new()).pop().unwrap().cost, 400);
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple() {
    assert_eq!(part1(&Input {
        hallway: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        rooms: [
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
        ],
        ..Default::default()
    }), 7);

    assert_eq!(part1(&Input {
        hallway: [0, 0, 0, 0, 0, 1000, 0, 1000, 0, 1, 0],
        rooms: [
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
        ],
        ..Default::default()
    }), 4007);

    assert_eq!(part1(&Input {
        hallway: [0, 0, 0, 0, 0, 1000, 0, 0, 0, 0, 0],
        rooms: [
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([]),
            VecDeque::from([1, 1000]),
        ],
        ..Default::default()
    }), 5008);

    assert_eq!(part1(&input_generator(exemple_raw_input())), 12521);
}

#[allow(unused_variables)]
#[aoc(day23, part2)]

fn part2(input: &Input) -> Output {
    input.part2().best_solution().cost
}

#[allow(unreachable_code)]
#[test]
fn part2_provided_exemple() {
    assert_eq!(part2(&input_generator(exemple_raw_input())), 44169);
}

#[allow(unreachable_code)]
#[allow(dead_code)]
fn exemple_raw_input() -> &'static str {
    "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
}
