use std::{str::FromStr, convert::Infallible};
use std::cmp::{min, max};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{start}..{end}")]
struct CuboidAxisRange {
    start: isize,
    end: isize,
}

#[test]
fn test_parse_range() {
    assert_eq!("-5..10".parse::<CuboidAxisRange>(), Ok(CuboidAxisRange {
        start: -5,
        end: 10,
    }))
}

impl CuboidAxisRange {
    pub fn len(&self) -> usize {
        ((self.end - self.start).abs() + 1).try_into().unwrap()
    }
}

#[test]
fn test_range_len() {
    assert_eq!(CuboidAxisRange {
        start: 1,
        end: 1,
    }.len(), 1);

    assert_eq!(CuboidAxisRange {
        start: 2,
        end: 5,
    }.len(), 4);

    assert_eq!(CuboidAxisRange {
        start: 5,
        end: 2,
    }.len(), 4);

    assert_eq!(CuboidAxisRange {
        start: -5,
        end: 10,
    }.len(), 16)
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("x={x},y={y},z={z}")]
struct Cuboid {
    x: CuboidAxisRange,
    y: CuboidAxisRange,
    z: CuboidAxisRange,
}

#[test]
fn test_parse_cuboid() {
    assert_eq!("x=-5..10,y=1..2,z=3..6".parse::<Cuboid>(), Ok(Cuboid {
        x: CuboidAxisRange {
            start: -5,
            end: 10,
        },
        y: CuboidAxisRange {
            start: 1,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 3,
            end: 6,
        },
    }))
}

impl Cuboid {
    pub fn size(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }

    pub fn remove_from(&self, cuboids: Vec<Cuboid>) -> Vec<Cuboid> {
        let mut remaining = vec![];

        for c in cuboids {
            remaining.extend(c.without(self));
        }

        remaining
    }

    pub fn without(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut remaining = vec![];

        let before_x = min(self.x.end, other.x.start - 1);
        let after_x = max(self.x.start, other.x.end + 1);
        let between_x = self.x.end >= other.x.start && self.x.start <= other.x.end;

        // left
        if self.x.start < other.x.start {
            remaining.push(Cuboid {
                x: CuboidAxisRange { start: self.x.start, end: before_x },
                y: self.y,
                z: self.z,
            })
        }

        // right
        if self.x.end > other.x.end {
            remaining.push(Cuboid {
                x: CuboidAxisRange { start: after_x, end: self.x.end },
                y: self.y,
                z: self.z,
            })
        }

        if !between_x {
            return remaining;
        }

        let x_start = max(self.x.start, other.x.start);
        let x_end = min(self.x.end, other.x.end);

        let before_y = min(self.y.end, other.y.start - 1);
        let after_y = max(self.y.start, other.y.end + 1);
        let between_y = self.y.end >= other.y.start && self.y.start <= other.y.end;

        // front
        if self.y.end > other.y.end {
            remaining.push(Cuboid {
                x: CuboidAxisRange { start: x_start, end: x_end },
                y: CuboidAxisRange { start: after_y, end: self.y.end },
                z: self.z,
            })
        }

        // back
        if self.y.start < other.y.start {
            remaining.push(Cuboid {
                x: CuboidAxisRange { start: x_start, end: x_end },
                y: CuboidAxisRange { start: self.y.start, end: before_y },
                z: self.z,
            })
        }

        if !between_y {
            return remaining;
        }

        let y_start = max(self.y.start, other.y.start);
        let y_end = min(self.y.end, other.y.end);

        let before_z = min(self.z.end, other.z.start - 1);
        let after_z = max(self.z.start, other.z.end + 1);

        // top
        if self.z.end > other.z.end {
            remaining.push(Cuboid {
                x: CuboidAxisRange { start: x_start, end: x_end },
                y: CuboidAxisRange { start: y_start, end: y_end },
                z: CuboidAxisRange { start: after_z, end: self.z.end },
            })
        }

        // bottom
        if self.z.start < other.z.start {
            remaining.push(Cuboid {
                x: CuboidAxisRange { start: x_start, end: x_end },
                y: CuboidAxisRange { start: y_start, end: y_end },
                z: CuboidAxisRange { start: self.z.start, end: before_z },
            })
        }

        remaining
    }

    pub fn filter(&self, l: isize, h: isize) -> Option<Self> {
        if self.x.start > h 
        || self.x.end < l
        || self.y.start > h 
        || self.y.end < l
        || self.z.start > h 
        || self.z.end < l {
            return None;
        }

        Some(Self {
            x: CuboidAxisRange { start: max(self.x.start, l), end: min(self.x.end, h) },
            y: CuboidAxisRange { start: max(self.y.start, l), end: min(self.y.end, h) },
            z: CuboidAxisRange { start: max(self.z.start, l), end: min(self.z.end, h) },
        })
    }
}

#[test]
fn test_cuboid_without() {
    assert_eq!(Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }.without(&Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }), vec![]);

    assert_eq!(Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }.without(&Cuboid {
        x: CuboidAxisRange {
            start: 1,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }), vec![Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 0,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }]);

    assert_eq!(Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }.without(&Cuboid {
        x: CuboidAxisRange {
            start: 1,
            end: 1,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }), vec![Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 0,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }, Cuboid {
        x: CuboidAxisRange {
            start: 2,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }]);

    assert_eq!(Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }.without(&Cuboid {
        x: CuboidAxisRange {
            start: 1,
            end: 1,
        },
        y: CuboidAxisRange {
            start: 1,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }), vec![Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 0,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }, Cuboid {
        x: CuboidAxisRange {
            start: 2,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }, Cuboid {
        x: CuboidAxisRange {
            start: 1,
            end: 1,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 0,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }]);

    assert_eq!(Cuboid {
        x: CuboidAxisRange {
            start: 1,
            end: 3,
        },
        y: CuboidAxisRange {
            start: 4,
            end: 4,
        },
        z: CuboidAxisRange {
            start: 2,
            end: 4,
        },
    }.without(&Cuboid {
        x: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 0,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 0,
            end: 2,
        },
    }), vec![Cuboid {
        x: CuboidAxisRange {
            start: 3,
            end: 3,
        },
        y: CuboidAxisRange {
            start: 4,
            end: 4,
        },
        z: CuboidAxisRange {
            start: 2,
            end: 4,
        },
    }, Cuboid {
        x: CuboidAxisRange {
            start: 1,
            end: 2,
        },
        y: CuboidAxisRange {
            start: 4,
            end: 4,
        },
        z: CuboidAxisRange {
            start: 2,
            end: 4,
        },
    }]);
}

#[test]
fn test_cuboid_size() {
    assert_eq!(Cuboid {
        x: CuboidAxisRange {
            start: 10,
            end: 12,
        },
        y: CuboidAxisRange {
            start: 10,
            end: 12,
        },
        z: CuboidAxisRange {
            start: 10,
            end: 12,
        },
    }.size(), 3 * 3 * 3);
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum RebootStep {
    #[display("on {0}")]
    On(Cuboid),
    #[display("off {0}")]
    Off(Cuboid),
}

impl RebootStep {
    pub fn filter(&self, min: isize, max: isize) -> Option<Self> {
        match self {
            RebootStep::On(c) => c.filter(min, max).map(|c| RebootStep::On(c)),
            RebootStep::Off(c) => c.filter(min, max).map(|c| RebootStep::Off(c)),
        }
    }
}
#[test]
fn test_parse_reboot_step() {
    assert_eq!("on x=-5..10,y=1..2,z=3..6".parse::<RebootStep>(), Ok(RebootStep::On(Cuboid {
        x: CuboidAxisRange {
            start: -5,
            end: 10,
        },
        y: CuboidAxisRange {
            start: 1,
            end: 2,
        },
        z: CuboidAxisRange {
            start: 3,
            end: 6,
        },
    })))
}

#[derive(Debug, PartialEq)]
struct Input {
    steps: Vec<RebootStep>,
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            steps: s.lines()
                .map(|line| line.parse::<RebootStep>().unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

#[allow(unused_variables)]
#[aoc_generator(day22)]
fn input_generator(input: &str) -> Input {
    input.parse::<Input>().unwrap()
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple_input() {
    assert_eq!(input_generator(exemple_raw_input()), Input {
        steps: vec![
            RebootStep::On(Cuboid {
                x: CuboidAxisRange {
                    start: 10,
                    end: 12,
                },
                y: CuboidAxisRange {
                    start: 10,
                    end: 12,
                },
                z: CuboidAxisRange {
                    start: 10,
                    end: 12,
                },
            }),
            RebootStep::On(Cuboid {
                x: CuboidAxisRange {
                    start: 11,
                    end: 13,
                },
                y: CuboidAxisRange {
                    start: 11,
                    end: 13,
                },
                z: CuboidAxisRange {
                    start: 11,
                    end: 13,
                },
            }),
            RebootStep::Off(Cuboid {
                x: CuboidAxisRange {
                    start: 9,
                    end: 11,
                },
                y: CuboidAxisRange {
                    start: 9,
                    end: 11,
                },
                z: CuboidAxisRange {
                    start: 9,
                    end: 11,
                },
            }),
            RebootStep::On(Cuboid {
                x: CuboidAxisRange {
                    start: 10,
                    end: 10,
                },
                y: CuboidAxisRange {
                    start: 10,
                    end: 10,
                },
                z: CuboidAxisRange {
                    start: 10,
                    end: 10,
                },
            }),
        ],
    })
}

#[derive(PartialEq, Debug)]
struct Reactor {
    cuboids_on: Vec<Cuboid>,
}

impl Default for Reactor {
    fn default() -> Self {
        Self { cuboids_on: Default::default() }
    }
}

impl Reactor {
    pub fn count_cubes_on(&self) -> usize {
        self.cuboids_on.iter()
            .map(|c| c.size())
            .sum()
    }

    pub fn initialize(&mut self, steps: &[RebootStep]) {
        for step in steps {
            step.filter(-50, 50).map(|step| self.apply(&step));
        }
    }

    pub fn reboot(&mut self, steps: &[RebootStep]) {
        for step in steps {
            self.apply(step);
        }
    }

    pub fn apply(&mut self, step: &RebootStep) {
        match step {
            RebootStep::On(c) => self.on(c),
            RebootStep::Off(c) => self.off(c),
        }
    }

    fn on(&mut self, on: &Cuboid) {
        let mut on = vec![*on];

        for c in &self.cuboids_on {
            on = c.remove_from(on);
        }

        self.cuboids_on.extend(on);
    }

    fn off(&mut self, off: &Cuboid) {
        self.cuboids_on = self.cuboids_on.iter()
            .map(|c| c.without(off))
            .flatten()
            .collect::<Vec<_>>()
    }
}
type Output = usize;

#[allow(unused_variables)]
#[aoc(day22, part1)]

fn part1(input: &Input) -> Output {
    let mut reactor = Reactor::default();

    reactor.initialize(&input.steps);

    reactor.count_cubes_on()
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple() {
    assert_eq!(part1(&input_generator(exemple_raw_input())), 39);
    assert_eq!(part1(&input_generator(larger_exemple_raw_input())), 590784);
}

#[allow(unused_variables)]
#[aoc(day22, part2)]

fn part2(input: &Input) -> Output {
    let mut reactor = Reactor::default();

    reactor.reboot(&input.steps);

    reactor.count_cubes_on()
}

#[allow(unreachable_code)]
#[test]
fn part2_provided_exemple() {
    assert_eq!(part2(&input_generator(exemple_raw_input())), 39);
    assert_eq!(part2(&input_generator(larger_exemple_raw_input())), 39769202357779);
}

#[allow(unreachable_code)]
#[allow(dead_code)]
fn exemple_raw_input() -> &'static str {
    "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"
}

#[allow(unreachable_code)]
#[allow(dead_code)]
fn larger_exemple_raw_input() -> &'static str {
    "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"
}
