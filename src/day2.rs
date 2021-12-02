use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum Command {
    #[display("forward {0}")]
    Forward(u32),
    #[display("down {0}")]
    Down(u32),
    #[display("up {0}")]
    Up(u32),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]

pub fn part1(commands: &[Command]) -> u32 {
    let (x, z, _) = commands.iter().fold((0, 0, 0), |(x, y, a), command| match command {
        Command::Forward(dx) => (x + dx, y + (a * dx), a),
        Command::Down(da) => (x, y, a + da),
        Command::Up(da) => (x, y, a - da),
    });

    x * z
}

#[aoc(day2, part2)]

pub fn part2(commands: &[Command]) -> u32 {
    let (x, z) = commands.iter().fold((0, 0), |(x, y), command| match command {
        Command::Forward(dx) => (x + dx, y),
        Command::Down(dy) => (x, y + dy),
        Command::Up(dy) => (x, y - dy),
    });

    x * z
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE_RAW_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";
    const EXEMPLE_PARSED_INPUT: &[Command] = &[
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn part2_provided_exemple() {
        assert_eq!(part2(EXEMPLE_PARSED_INPUT), 150);
    }

    #[test]
    fn part1_provided_exemple() {
        assert_eq!(part1(EXEMPLE_PARSED_INPUT), 900);
    }

    #[test]
    fn part1_provided_exemple_input() {
        assert_eq!(input_generator(EXEMPLE_RAW_INPUT), EXEMPLE_PARSED_INPUT);
    }
}
