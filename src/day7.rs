#[derive(Debug, PartialEq)]
struct Input {
    crabs: Vec<isize>
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Input {
    Input {
        crabs: input
            .split(',')
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>(),
    }
}

type Output = isize;

#[allow(unused_variables)]
#[aoc(day7, part1)]

fn part1(input: &Input) -> Output {
    let max = input.crabs.iter().max().unwrap_or(&0);

    (0..=*max)
        .map(|t| input.crabs.iter()
            .map(|n| (n - t).abs())
            .sum::<isize>(),
        )
        .min().unwrap()
}

#[allow(unused_variables)]
#[aoc(day7, part2)]

fn part2(input: &Input) -> Output {
    let max = input.crabs.iter().max().unwrap_or(&0);

    (0..=*max)
        .map(|t| input.crabs.iter()
            .map(|n| {
                let m = (n - t).abs() as f32;

                (( m / 2.0 ) * (m + 1.0)) as isize
            })
            .sum::<isize>(),
        )
        .min().unwrap()
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), 168);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), 37);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input()), Input {
            crabs: vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14],
        });
    }
}
