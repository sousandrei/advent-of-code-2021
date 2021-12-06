use std::fmt::Display;

pub trait Day {
    type In;
    type P1: Display;
    type P2: Display;

    fn input() -> Self::In;
    fn part1(input: &Self::In) -> Self::P1;
    fn part2(input: &Self::In) -> Self::P2;
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
