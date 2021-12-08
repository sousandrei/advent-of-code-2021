use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn parse_input(data: &str) -> Vec<Instruction> {
    data.split('\n')
        .map(|n| {
            let parts: Vec<&str> = n.split(' ').collect();
            let value = parts[1].parse().unwrap();

            match parts[0] {
                "up" => Instruction::Up(value),
                "down" => Instruction::Down(value),
                "forward" => Instruction::Forward(value),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

pub fn input() -> Vec<Instruction> {
    let data = read_to_string("inputs/day2.txt").unwrap();
    parse_input(&data)
}

pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut depth = 0;
    let mut pos = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Up(value) => depth -= value,
            Instruction::Down(value) => depth += value,
            Instruction::Forward(value) => pos += value,
        };
    }

    depth * pos
}

pub fn part2(instructions: &[Instruction]) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Forward(value) => {
                pos += value;
                depth += value * aim;
            }
            Instruction::Up(value) => aim -= value,
            Instruction::Down(value) => aim += value,
        };
    }

    depth * pos
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Instruction> {
        let data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        parse_input(&data)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 900);
    }
}
