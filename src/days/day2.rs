use std::fs::read_to_string;

use super::Day;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

pub struct Day2;

impl Day for Day2 {
    type In = Vec<Instruction>;

    fn input() -> Self::In {
        read_to_string("inputs/day2.txt")
            .unwrap()
            .split('\n')
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

    fn part1(instructions: &Self::In) -> i32 {
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

    fn part2(instructions: &Self::In) -> i32 {
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
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn input() -> Vec<Instruction> {
        vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day2::part1(&input()), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day2::part2(&input()), 900);
    }
}
