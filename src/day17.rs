use std::{ops::RangeInclusive, convert::Infallible};

#[derive(Debug, PartialEq, parse_display::FromStr)]
#[display("target area: {target_area}")]
struct Input {
    target_area: Area
}

#[derive(Debug, PartialEq, parse_display::FromStr)]
#[display("x={x_range}, y={y_range}")]
struct Area {
    x_range: AxisRange,
    y_range: AxisRange,
}

#[derive(Debug, PartialEq)]
struct AxisRange {
    range: RangeInclusive<i32>,
}

impl std::str::FromStr for AxisRange {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(input.split_once("..")
            .map(|(start, end)| AxisRange {
                range: RangeInclusive::new(
                    start.parse().unwrap(),
                    end.parse().unwrap(),
                ),
            })
            .unwrap(),
        )
    }
}

#[allow(unused_variables)]
#[aoc_generator(day17)]
fn input_generator(input: &str) -> Input {
    input.parse::<Input>().unwrap()
}

#[test]
fn part1_provided_exemple_input() {
    assert_eq!(input_generator(exemple_raw_input()), Input {
        target_area: Area {
            x_range: AxisRange {
                range: 20..=30,
            },
            y_range: AxisRange {
                range: -10..=-5,
            },
        }
    });
}

type Output = i32;

#[allow(unused_variables)]
#[aoc(day17, part1)]

fn part1(input: &Input) -> Output {
    let y_range = &input.target_area.y_range.range;
    let y_limit = *y_range.start();

    let mut best_vy = 0;

    for initial_vy in (best_vy+1)..=-y_limit {
        let mut vy = initial_vy;
        let mut y = 0;

        loop {
            y += vy;

            if y_range.contains(&y) {
                best_vy = initial_vy;
                break;
            }

            if y < y_limit {
                break;
            }

            vy -= 1;
        }
    }

    best_vy * (best_vy + 1) / 2
}

#[test]
fn part1_provided_exemple() {
    assert_eq!(part1(&input_generator(exemple_raw_input())), 45);
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[aoc(day17, part2)]

fn part2(input: &Input) -> Output {
    let x_range = &input.target_area.x_range.range;
    let y_range = &input.target_area.y_range.range;

    let x_min = *x_range.start();
    let mut vx_min = 1;

    'outer: loop {
        let mut vx = vx_min;
        let mut x = 0;

        loop {
            x += vx;

            if x >= x_min {
                break 'outer;
            }

            if vx == 1 {
                break;
            }

            vx -= 1;
        }

        vx_min += 1;
    }

    let x_limit = *x_range.end();
    let y_limit = *y_range.start();

    let mut count = 0;

    for initial_vx in vx_min..=x_limit {
        for initial_vy in y_limit..=-y_limit {
            let mut vx = initial_vx;
            let mut vy = initial_vy;
            let mut x = 0;
            let mut y = 0;

            loop {
                x += vx;
                y += vy;

                if x_range.contains(&x) && y_range.contains(&y) {
                    count += 1;
                    break;
                }

                if x > x_limit || y < y_limit {
                    break;
                }

                vx -= match vx.cmp(&0) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                };

                vy -= 1;
            }
        }
    }

    count
}

#[allow(unreachable_code)]
#[test]
fn part2_provided_exemple() {
    assert_eq!(part2(&input_generator(exemple_raw_input())), 112);
}

#[allow(dead_code)]
fn exemple_raw_input() -> &'static str {
    "target area: x=20..30, y=-10..-5"
}
