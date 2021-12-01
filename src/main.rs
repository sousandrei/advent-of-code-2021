macro_rules! run_day {
    ($day:path) => {
        use $day::*;

        let i = input();
        println!(
            "{}: part1 = {}, part2 = {}",
            stringify!($day),
            part1(i.clone()),
            part2(i)
        );
    };
}

fn main() {
    use aoc2021::*;

    run_day!(day01);
}
