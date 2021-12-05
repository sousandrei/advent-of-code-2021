use std::{collections::HashMap, fs::read_to_string, str::FromStr};

use super::Day;

#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Line {
    from: Point,
    to: Point,
}

impl Line {
    pub fn count_points(&self, points: &mut HashMap<Point, usize>, diagonal: bool) {
        let max_x = self.from.x.max(self.to.x);
        let max_y = self.from.y.max(self.to.y);

        let min_x = self.from.x.min(self.to.x);
        let min_y = self.from.y.min(self.to.y);

        if !diagonal && !self.is_straight() {
            return;
        }

        if diagonal {
            let len = (max_x - min_x).max(max_y - min_y);

            for i in 0..len + 1 {
                let x = match (self.from.x, self.to.x) {
                    (f, t) if f > t => f - i,
                    (f, t) if f < t => f + i,
                    _ => self.from.x,
                };

                let y = match (self.from.y, self.to.y) {
                    (f, t) if f > t => f - i,
                    (f, t) if f < t => f + i,
                    _ => self.from.y,
                };

                let p = Point { x, y };
                *points.entry(p).or_insert(0) += 1;
            }

            return;
        }

        if self.from.x == self.to.x {
            for y in min_y..=max_y {
                let p = Point { x: self.from.x, y };
                *points.entry(p).or_insert(0) += 1;
            }
        }

        if self.from.y == self.to.y {
            for x in min_x..=max_x {
                let p = Point { x, y: self.from.y };
                *points.entry(p).or_insert(0) += 1;
            }
        }
    }

    pub fn is_straight(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ").map(|p| {
            let mut xy = p.split(',');

            Point {
                x: xy.next().unwrap().parse().unwrap(),
                y: xy.next().unwrap().parse().unwrap(),
            }
        });

        let line = Line {
            from: points.next().unwrap(),
            to: points.next().unwrap(),
        };

        Ok(line)
    }
}

pub struct Day5;

impl Day5 {
    fn parse_input(data: &str) -> Vec<Line> {
        data.split('\n')
            .map(Line::from_str)
            .map(Result::unwrap)
            .collect()
    }
}

impl Day for Day5 {
    type In = Vec<Line>;

    fn input() -> Self::In {
        let data = read_to_string("inputs/day5.txt").unwrap();
        Day5::parse_input(&data)
    }

    fn part1(data: &Self::In) -> i32 {
        let mut counter: HashMap<Point, usize> = HashMap::new();

        for line in data.iter() {
            line.count_points(&mut counter, false)
        }

        let overlaps = &counter.into_iter().filter(|(_, n)| *n > 1).count();
        *overlaps as i32
    }

    fn part2(data: &Self::In) -> i32 {
        let mut counter: HashMap<Point, usize> = HashMap::new();

        for line in data.iter() {
            line.count_points(&mut counter, true)
        }

        let overlaps = &counter.into_iter().filter(|(_, n)| *n > 1).count();
        *overlaps as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Line> {
        let data = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        Day5::parse_input(&data)
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day5::part1(&input()), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day5::part2(&input()), 12);
    }
}
