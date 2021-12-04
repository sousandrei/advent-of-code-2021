use std::fs::read_to_string;

use super::Day;

pub struct Day1;

impl Day for Day1 {
    type In = Vec<i32>;

    fn input() -> Vec<i32> {
        read_to_string("inputs/day1.txt")
            .unwrap()
            .split('\n')
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
    }

    fn part1(integers: &Self::In) -> i32 {
        let mut count = 0;
        let mut last = integers[0];

        for curr in integers {
            if curr > &last {
                count += 1;
            }
            last = *curr;
        }

        count
    }

    fn part2(integers: &Self::In) -> i32 {
        let mut count = 0;
        let mut last = 0;

        let mut integers = integers.iter();

        let mut window: [i32; 3] = [
            *integers.next().unwrap(),
            *integers.next().unwrap(),
            *integers.next().unwrap(),
        ];

        for (i, curr) in integers.enumerate() {
            window[i % 3] = *curr;
            let curr_sum: i32 = window.iter().sum();

            if curr_sum > last {
                count += 1;
            }

            last = curr_sum;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn input() -> Vec<i32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day1::part1(&input()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day1::part2(&input()), 5);
    }
}