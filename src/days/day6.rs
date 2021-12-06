use std::fs::read_to_string;

use super::Day;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Fish(u8);

impl Fish {
    pub fn tick(&mut self) -> bool {
        if self.0 == 0 {
            self.0 = 6;
            return true;
        }

        self.0 -= 1;
        false
    }
}

pub fn rotate_count(ages: &mut [usize; 9], days: usize) {
    for _ in 0..days {
        let births = ages[0];
        ages.rotate_left(1);
        ages[6] += births;
    }
}

pub struct Day6;

impl Day6 {
    fn parse_input(data: &str) -> Vec<Fish> {
        let fishes: Vec<Fish> = data
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .map(Fish)
            .collect();

        fishes
    }
}

impl Day for Day6 {
    type In = Vec<Fish>;
    type P1 = usize;
    type P2 = usize;

    fn input() -> Self::In {
        let data = read_to_string("inputs/day6.txt").unwrap();
        Day6::parse_input(&data)
    }

    fn part1(fishes: &Self::In) -> Self::P1 {
        let mut ages = [0; 9];

        for Fish(f) in fishes {
            ages[*f as usize] += 1;
        }

        rotate_count(&mut ages, 80);

        ages.iter().sum()
    }

    fn part2(fishes: &Self::In) -> Self::P2 {
        let mut ages = [0; 9];

        for Fish(f) in fishes {
            ages[*f as usize] += 1;
        }

        rotate_count(&mut ages, 256);

        ages.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Fish> {
        let data = "3,4,3,1,2";
        Day6::parse_input(&data)
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day6::part1(&input()), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day6::part2(&input()), 26984457539);
    }
}
