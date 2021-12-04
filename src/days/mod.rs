pub trait Day {
    type In;

    fn input() -> Self::In;
    fn part1(input: &Self::In) -> i32;
    fn part2(input: &Self::In) -> i32;
}

pub mod day1;
pub mod day2;
pub mod day3;
