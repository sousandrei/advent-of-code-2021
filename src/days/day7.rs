use std::fs::read_to_string;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse_input(data: &str) -> Vec<i32> {
    data.split(',')
        .map(|x| x.parse::<i32>())
        .map(Result::unwrap)
        .collect()
}

fn get_cost(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    n + get_cost(n - 1)
}

pub fn input() -> Vec<i32> {
    let data = read_to_string("inputs/day7.txt").unwrap();
    parse_input(&data)
}

pub fn part1(crabs: &Vec<i32>) -> i32 {
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();

    (min..=max)
        .into_par_iter()
        .map(|i| {
            crabs
                .iter()
                // maps crabs
                .map(move |&x| x - i)
                .map(|x| x.abs())
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part2(crabs: &Vec<i32>) -> i32 {
    let max = *crabs.iter().max().unwrap();
    let min = *crabs.iter().min().unwrap();

    (min..=max)
        .into_par_iter()
        .map(|i| {
            crabs
                .iter()
                .map(move |&x| get_cost((x - i).abs()))
                .map(|x: i32| x.abs())
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
