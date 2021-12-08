use std::fs::read_to_string;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse_input(data: &str) -> Vec<i32> {
    data.split(',')
        .map(|x| x.parse::<i32>())
        .map(Result::unwrap)
        .collect()
}

pub fn input() -> Vec<i32> {
    let data = read_to_string("inputs/day7.txt").unwrap();
    parse_input(&data)
}

pub fn part1(crabs: &[i32]) -> i32 {
    let mut crabs = crabs.to_owned();
    crabs.sort_unstable();

    let median = crabs[crabs.len() / 2];

    crabs
        .into_par_iter()
        .map(move |x| x - median)
        .map(|x| x.abs())
        .sum()
}

pub fn part2(crabs: &[i32]) -> i32 {
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();

    (min..=max)
        .into_par_iter()
        .map(|i| {
            crabs
                .iter()
                .map(move |&x| {
                    let n = (x - i).abs();
                    (n * (n + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<i32> {
        let data = "16,1,2,0,4,2,7,1,2,14";
        parse_input(&data)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 168);
    }
}
