
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(measurements: &[u32]) -> u32 {
    measurements
        .iter()
        .fold((0, &u32::MAX), |(total, last), current| (total + if current > last { 1 } else { 0 }, current)).0
}

#[aoc(day1, part2)]
pub fn part2(measurements: &[u32]) -> u32 {
    part1(&measurements
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<_>>()
        [..]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7);
    }

    #[test]
    fn part1_samples() {
        assert_eq!(part1(&[1]), 0);
        assert_eq!(part1(&[1, 0]), 0);
        assert_eq!(part1(&[1, 0, 2]), 1);
    }

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 5);
    }

    #[test]
    fn part2_samples() {
        assert_eq!(part2(&[1]), 0);
        assert_eq!(part2(&[1, 0]), 0);
        assert_eq!(part2(&[1, 0, 2, 1]), 0);
        assert_eq!(part2(&[1, 0, 2, 2]), 1);
    }
}
