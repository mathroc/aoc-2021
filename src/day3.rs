pub struct DiagnosticReport {
    pub data: Vec<Vec<bool>>
}

impl DiagnosticReport {
    pub fn from(input: &str) -> Self {
        let data = input
            .lines()
            .map(|l| l.chars().map(|c| c == '1').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { data }
    }

    pub fn part1(&self) -> (u32, u32) {
        self.data.iter()
            .fold(vec![], |mut res, line| {
                res.resize(line.len(), (0, 0));

                res.iter()
                    .zip(line.iter())
                    .map(|((gamma, epsilon), bit)| {
                        let d = if *bit { 1 } else { -1 };
                        (*gamma + d, *epsilon + d)
                    })
                    .collect()
            })
            .iter()
            .fold((0, 0), |(gamma, epsilon), (g, e)| (
                gamma * 2 + if *g > 0 { 1 } else { 0 },
                epsilon * 2 + if *e < 0 { 1 } else { 0 }
            ))
    }

    fn rating(&self, report: &Vec<&Vec<bool>>, i: usize, o2: bool) -> u32 {
        if report.len() == 1 {
            return report[0].iter().fold(0, |n, bit| n * 2 + if *bit { 1 } else { 0 });
        }

        let (ones, zeroes) = report.iter().fold((0, 0), |(ones, zeroes), line| if line[i] {
            (ones + 1, zeroes)
        } else {
            (ones, zeroes + 1)
        });

        let w = o2 == (ones >= zeroes);

        self.rating(&report.iter().filter_map(|line| if line[i] == w { Some(*line) } else { None }).collect(), i + 1, o2)
    }

    pub fn oxygen_generator_rating(&self) -> u32 {
        self.rating(&self.data.iter().collect(), 0, true)
    }

    pub fn carbon_dioxyde_scrubber_rating(&self) -> u32 {
        self.rating(&self.data.iter().collect(), 0, false)
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> DiagnosticReport {
    DiagnosticReport::from(input)
}

#[aoc(day3, part1)]
pub fn part1(report: &DiagnosticReport) -> u32 {
    let (gamma, epsilon) = report.part1();

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(report: &DiagnosticReport) -> u32 {
    report.oxygen_generator_rating() * report.carbon_dioxyde_scrubber_rating()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE_RAW_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    const EXEMPLE_PARSED_INPUT: &[[bool;5]] = &[
        [false, false, true, false, false],
        [true, true, true, true, false],
        [true, false, true, true, false],
        [true, false, true, true, true],
        [true, false, true, false, true],
        [false, true, true, true, true],
        [false, false, true, true, true],
        [true, true, true, false, false],
        [true, false, false, false, false],
        [true, true, false, false, true],
        [false, false, false, true, false],
        [false, true, false, true, false],
    ];

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(&input_generator(EXEMPLE_RAW_INPUT)), 230);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(&input_generator(EXEMPLE_RAW_INPUT)), 198);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(EXEMPLE_RAW_INPUT).data, EXEMPLE_PARSED_INPUT);
    }
}
