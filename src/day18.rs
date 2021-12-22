#[derive(Debug, PartialEq)]
struct Input {
    numbers: Vec<Vec<(u8, u8)>>
}

fn parse_fish(s: &[u8], depth: u8) -> (Vec<(u8, u8)>, usize) {
    match s.get(0).unwrap() {
        b'[' => {
            let (mut left, i) = parse_fish(&s[1..], depth + 1);
            let (right, j) = parse_fish(&s[i + 2..], depth + 1);
            left.extend(right.into_iter());
            (left, i + j + 3)
        },
        c @ b'0'..=b'9' => ([(*c - b'0', depth)].into_iter().collect(), 1),
        _ => panic!(),
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Input {
    Input {
        numbers: input.lines()
            .map(|line| parse_fish(line.as_bytes(), 0).0)
            .collect::<Vec<_>>(),
    }
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple_input() {
    assert_eq!(input_generator(exemple_raw_input()), Input {
        numbers: vec![
            vec![(1,1), (1,1)],
            vec![(2,1), (2,1)],
            vec![(3,1), (3,1)],
            vec![(4,1), (4,1)],
        ],
    });
}

type Output = u32;

#[allow(unused_variables)]
#[aoc(day18, part1)]

fn part1(input: &Input) -> Output {
    magnitude(input.numbers.iter().cloned().reduce(|l, r| add_fish(l, &r)).unwrap())
}

fn add_fish(mut l: Vec<(u8, u8)>, r: &Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    l.extend(r);

    for (_value, depth) in &mut l {
        *depth += 1;
    }

    reduce_fish(reduce_fish(l, true), false)
}

fn reduce_fish(mut v: Vec<(u8, u8)>, first_pass: bool) -> Vec<(u8, u8)> {
    let mut out: Vec<(u8, u8)> = vec![];
    let mut i = 0;
    while i < v.len() {
        let (value, depth) = v[i];
        if depth >= 5 {
            // Explode in-place by undoing our last push to the output, allowing
            // us to split on it in the next iteration if necessary.
            let after_increment = v[i + 1].0;
            if let Some(after) = v.get_mut(i + 2) {
                after.0 += after_increment;
            }
            v[i + 1] = (0, depth - 1);
            if let Some(last_push) = out.pop() {
                v[i] = (last_push.0 + value, last_push.1);
            } else {
                i += 1;
            }
        } else if !first_pass && value >= 10 {
            // Try to split in place if possible by moving i back.
            if i > 0 {
                i -= 1;
                v[i] = (value / 2, depth + 1);
            } else {
                v.insert(0, (value / 2, depth + 1));
            }
            v[i + 1] = (value - value / 2, depth + 1);
        } else {
            out.push((value, depth));
            i += 1;
        }
    }
    out
}

fn magnitude(v: Vec<(u8, u8)>) -> u32 {
    let mut stack = vec![];

    for (value, depth) in v {
        stack.push((value as u32, depth));
        while let Some(&[l, r]) = stack.get(stack.len().saturating_sub(2)..) {
            if l.1 == r.1 {
                stack.pop();
                stack.pop();
                stack.push((3 * l.0 + 2 * r.0, l.1 - 1));
            } else {
                break;
            }
        }
    }

    stack.pop().unwrap().0
}

#[allow(unreachable_code)]
#[test]
fn part1_provided_exemple() {
    assert_eq!(part1(&input_generator(exemple_raw_input())), 445);
}

#[allow(unused_variables)]
#[aoc(day18, part2)]

fn part2(input: &Input) -> Output {
    let mut max = 0;

    for l in input.numbers.clone() {
        for r in input.numbers.clone() {
            max = std::cmp::max(max, magnitude(add_fish(l.clone(), &r)));
            max = std::cmp::max(max, magnitude(add_fish(r.clone(), &l)));
        }
    }

    max
}

#[allow(unreachable_code)]
#[test]
fn part2_provided_exemple() {
    assert_eq!(part2(&input_generator(exemple_raw_input())), 100);
}

#[allow(unreachable_code)]
#[allow(dead_code)]
fn exemple_raw_input() -> &'static str {
    "[1,1]
[2,2]
[3,3]
[4,4]"
}
