use aoc2021::days::*;

macro_rules! run_day {
    ($d:expr) => {
        paste::expr! {
            solve::<[<day $d>]::[<Day $d>]>($d);
        }
    };
}

fn main() {
    run_day!(1);
    run_day!(2);
    run_day!(3);
    run_day!(4);
    run_day!(5);
}

fn solve<S>(day_number: i32)
where
    S: Day,
{
    let i = S::input();

    println!(
        "day {}: part1 = {}, part2 = {}",
        day_number,
        S::part1(&i),
        S::part2(&i)
    );
}
