#[derive(Debug, PartialEq)]
struct Input {}

#[allow(unused_variables)]
#[aoc_generator(dayX)]
fn input_generator(input: &str) -> Input {
    todo!()
}

type Output = usize;

#[allow(unused_variables)]
#[aoc(dayX, part1)]

fn part1(input: &Input) -> Output {
    todo!()
}

#[allow(unused_variables)]
#[aoc(dayX, part2)]

fn part2(input: &Input) -> Output {
    todo!()
}

#[allow(unreachable_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn exemple_raw_input() -> &'static str {
        todo!()
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(exemple_raw_input())), todo!());
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(exemple_raw_input())), todo!());
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(exemple_raw_input()), todo!());
    }
}