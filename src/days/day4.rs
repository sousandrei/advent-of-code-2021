use std::{fs::read_to_string, str::FromStr};

use super::Day;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Board {
    numbers: Vec<Vec<i32>>,
    hits: Vec<Vec<i32>>,
    won: bool,
}

impl Board {
    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;

        for i in 0..self.numbers[0].len() {
            for j in 0..self.numbers.len() {
                if self.hits[i][j] == 0 {
                    sum += self.numbers[i][j];
                }
            }
        }

        sum
    }

    pub fn check_row(&self) -> bool {
        let check_line = |line: Vec<i32>| {
            for &n in line.iter() {
                if n < 1 {
                    return false;
                }
            }

            true
        };

        for col in self.hits.iter() {
            if check_line(col.clone()) {
                return true;
            }
        }

        false
    }
    pub fn check_col(&self) -> bool {
        for i in 0..self.hits[0].len() {
            let mut ones = vec![];

            for j in 0..self.hits.len() {
                ones.push(self.hits[j][i]);
            }

            if ones.iter().find(|&&x| x == 0).is_none() {
                return true;
            }
        }

        false
    }

    pub fn compute_guess(&mut self, guess: i32) {
        for (i, row) in self.numbers.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if *n == guess {
                    self.hits[i][j] = 1;
                }
            }
        }
    }
}

impl FromStr for Board {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<Vec<i32>> = s
            .split('\n')
            .map(|line| {
                line.split(' ')
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)
                    .collect::<Vec<i32>>()
            })
            .collect();

        let hits = vec![vec![0; numbers[0].len()]; numbers.len()];

        let board = Board {
            numbers,
            hits,
            won: false,
        };

        Ok(board)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Game {
    input: Vec<i32>,
    board: Vec<Board>,
}

pub struct Day4;

impl Day4 {
    fn parse_input(data: &str) -> Game {
        let data = data.replace("\n ", "\n").replace("  ", " ");
        let mut data: Vec<&str> = data.split("\n\n").collect();

        let input = data
            .remove(0)
            .split(',')
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect();

        let board = data
            .into_iter()
            .map(Board::from_str)
            .map(Result::unwrap)
            .collect();

        Game { input, board }
    }
}

impl Day for Day4 {
    type In = Game;

    fn input() -> Self::In {
        let data = read_to_string("inputs/day4.txt").unwrap();
        Day4::parse_input(&data)
    }

    fn part1(game: &Self::In) -> i32 {
        let mut game = game.clone();

        for guess in game.input.iter() {
            for b in game.board.iter_mut() {
                b.compute_guess(*guess);

                if b.check_col() || b.check_row() {
                    return b.sum_unmarked() * guess;
                }
            }
        }

        0
    }

    fn part2(game: &Self::In) -> i32 {
        let mut game = game.clone();

        let mut last_guess = 0;
        let mut last_win = None;

        for guess in game.input.iter() {
            for b in game.board.iter_mut() {
                if b.won {
                    continue;
                }

                b.compute_guess(*guess);

                if b.check_col() || b.check_row() {
                    b.won = true;

                    last_win = Some(b.clone());
                    last_guess = *guess;
                }
            }
        }

        last_win.unwrap().sum_unmarked() * last_guess
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Game {
        let data = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        Day4::parse_input(&data)
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day4::part1(&input()), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day4::part2(&input()), 1924);
    }
}
