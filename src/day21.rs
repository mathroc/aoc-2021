use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Game {
    first_player_is_next: bool,
    players: (Player, Player)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Player {
    position: usize,
    score: usize,
}

#[allow(unused_variables)]
#[aoc_generator(day21)]
fn input_generator(input: &str) -> Game {
    let players = input.lines()
        .map(|line| line.split_once(": ").map(|(_, n)| Player {
            position: n.parse::<usize>().unwrap() - 1,
            score: 0,
        }).unwrap())
        .collect::<Vec<_>>();
        
    Game {
        first_player_is_next: true,
        players: (players[0].clone(), players[1].clone()),
    }
}

#[test]
fn part1_provided_exemple_input() {
    assert_eq!(input_generator(exemple_raw_input()), Game {
        first_player_is_next: true,
        players: (
            Player {
                position: 3,
                score: 0,
            },
            Player {
                position: 7,
                score: 0,
            },
        ),
    });
}

impl Game {
    pub fn after_next_turn(&self, dice: &mut impl Dice) -> Self {
        let mut game_after_next_turn = self.clone();

        if self.first_player_is_next {
            &mut game_after_next_turn.players.0
        } else {
            &mut game_after_next_turn.players.1
        }.take_turn(dice);

        game_after_next_turn.first_player_is_next = !game_after_next_turn.first_player_is_next;

        game_after_next_turn
    }

    pub fn higher_score(&self) -> usize {
        std::cmp::max(self.players.0.score, self.players.1.score)
    }

    pub fn smallest_score(&self) -> usize {
        std::cmp::min(self.players.0.score, self.players.1.score)
    }
}

trait Dice {
    fn next(&mut self) -> usize;
}

#[derive(Debug, PartialEq)]
struct DeterministicDice {
    n: usize,
    max: usize,
}

impl DeterministicDice {
    pub fn new(max: usize, start: usize) -> Self {
        Self {
            n: max + start - 2,
            max,
        }
    }
}

impl Dice for DeterministicDice {
    fn next(&mut self) -> usize {
        self.n += 1;
        self.n %= self.max;

        self.n + 1
    }
}

#[test]
fn test_deterministic_dice() {
    let mut dice = DeterministicDice::new(6, 1);

    assert_eq!(dice.next(), 1);
    assert_eq!(dice.next(), 2);
    assert_eq!(dice.next(), 3);

    let mut dice = DeterministicDice::new(6, 5);

    assert_eq!(dice.next(), 5);
    assert_eq!(dice.next(), 6);
    assert_eq!(dice.next(), 1);
}

impl Player {
    pub fn take_turn(&mut self, dice: &mut impl Dice) {
        self.position += dice.next();
        self.position += dice.next();
        self.position += dice.next();

        self.position %= 10;

        self.score += self.position + 1;
    }
}

type Output = usize;

#[allow(unused_variables)]
#[aoc(day21, part1)]

fn part1(game: &Game) -> Output {
    let mut dice = DeterministicDice::new(100, 1);
    let mut game = game.clone();
    let mut turns = 0;

    while game.higher_score() < 1_000 {
        game = game.after_next_turn(&mut dice);
        turns += 1;
    }

    game.smallest_score() * turns * 3
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple() {
    assert_eq!(part1(&input_generator(exemple_raw_input())), 739785);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct DiracDiceTimeline {
    next: usize,
    rolls: [usize; 3],
}

impl DiracDiceTimeline {
    pub fn new(rolls: [usize; 3]) -> Self {
        Self {
            next: 0,
            rolls,
        }
    }
}

impl Dice for DiracDiceTimeline {
    fn next(&mut self) -> usize {
        let roll = self.rolls[self.next];
        
        self.next += 1;
        self.next %= self.rolls.len();

        roll
    }
}

struct Universes {
    cache: HashMap<Game, (usize, usize)>,
    dices: Vec<(DiracDiceTimeline, usize)>,
}

impl Universes {
    pub fn new() -> Self {
        let mut dices = HashMap::new();

        for roll_1 in 1..=3 {
            for roll_2 in 1..=3 {
                for roll_3 in 1..=3 {
                    let mut rolls = [roll_1, roll_2, roll_3];
                    
                    rolls.sort_unstable();

                    *dices.entry(DiracDiceTimeline::new(rolls))
                        .or_default() += 1;
                }
            }
        }

        Self { cache: HashMap::new(), dices: dices.iter().map(|(&dice, &n)| (dice, n)).collect::<Vec<_>>() }
    }

    pub fn scores(&mut self, game: &Game) -> (usize, usize) {
        if let Some(res) = self.cache.get(game) {
            return *res;
        }
        
        let new_scores = self.compute_scores(game);

        self.cache.insert(*game, new_scores);

        new_scores
    }

    fn compute_scores(&mut self, game: &Game) -> (usize, usize) {
        if game.players.0.score >= 21 {
            return (1, 0);
        }

        if game.players.1.score >= 21 {
            return (0, 1);
        }

        let (mut acc1, mut acc2) = (0, 0);

        for (dice, n) in self.dices.clone().iter_mut() {
            let next_game = &game.after_next_turn(dice);
            let (s1, s2) = self.scores(next_game);

            acc1 += s1 * *n;
            acc2 += s2 * *n;
        }

        (acc1, acc2)
    }
}

#[test]
fn test_compute_scores() {
    let mut u = Universes::new();

    assert_eq!(u.compute_scores(&Game {
        first_player_is_next: true,
        players: (
            Player {
                position: 9,
                score: 21,
            },
            Player {
                position: 9,
                score: 20,
            },
        )
    }), (1, 0));

    let mut u = Universes::new();

    assert_eq!(u.compute_scores(&Game {
        first_player_is_next: true,
        players: (
            Player {
                position: 9,
                score: 20,
            },
            Player {
                position: 9,
                score: 20,
            },
        )
    }), (27, 0));

    let mut u = Universes::new();

    assert_eq!(u.compute_scores(&Game {
        first_player_is_next: true,
        players: (
            Player {
                position: 9,
                score: 19,
            },
            Player {
                position: 9,
                score: 20,
            },
        )
    }), (27, 0));

    let mut u = Universes::new();

    assert_eq!(u.compute_scores(&Game {
        first_player_is_next: true,
        players: (
            Player {
                position: 9,
                score: 17,
            },
            Player {
                position: 9,
                score: 20,
            },
        )
    }), (26, 27));

    let mut u = Universes::new();

    assert_eq!(u.compute_scores(&Game {
        first_player_is_next: true,
        players: (
            Player {
                position: 9,
                score: 17,
            },
            Player {
                position: 9,
                score: 17,
            },
        )
    }), (53, 26));
}

#[allow(unused_variables)]
#[aoc(day21, part2)]

fn part2(game: &Game) -> Output {
    let mut universes = Universes::new();

    let (p1, p2) = universes.scores(&game);

    std::cmp::max(p1, p2)
}

#[allow(unreachable_code)]
#[test]
fn part2_provided_exemple() {
    assert_eq!(part2(&input_generator(exemple_raw_input())), 444356092776315);
}

#[allow(unreachable_code)]
#[allow(dead_code)]
fn exemple_raw_input() -> &'static str {
    "Player 1 starting position: 4
Player 2 starting position: 8"
}
