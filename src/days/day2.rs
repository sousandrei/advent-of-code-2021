use std::fs::read_to_string;

use super::Day;

pub struct Day2;

impl Day for Day2 {
    type In = Vec<i32>;

    fn input() -> Self::In {
        read_to_string("inputs/day2.txt")
            .unwrap()
            .split('\n')
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
    }

    fn part1(_integers: &Self::In) -> i32 {
        0
    }

    fn part2(_integers: &Self::In) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn input() -> Vec<i32> {
        vec![
            1,
            // forward 5
            // down 5
            // forward 8
            // up 3
            // down 8
            // forward 2
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day2::part1(&input()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day2::part2(&input()), 5);
    }
}
