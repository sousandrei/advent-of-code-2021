use std::fs::read;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Counter {
    ones: usize,
    zeros: usize,
}

fn count_bits(counter: &mut Vec<Counter>, number: &[u8]) {
    for (i, bit) in number.iter().enumerate() {
        if i >= counter.len() {
            counter.push(Counter { ones: 0, zeros: 0 });
        }

        match bit {
            0 => counter[i].zeros += 1,
            1 => counter[i].ones += 1,
            _ => panic!("Invalid character"),
        }
    }
}

fn calculate_dynamic(
    numbers: Vec<Vec<u8>>,
    logic: fn(&Vec<Counter>, usize, &&Vec<u8>) -> bool,
) -> i32 {
    let mut numbers = numbers.to_vec();

    for i in 0..numbers[0].len() {
        let mut counter: Vec<Counter> = vec![];

        numbers
            .iter()
            .for_each(|number| count_bits(&mut counter, number));

        numbers = numbers
            .iter()
            .filter(|n| logic(&counter, i, n))
            .cloned()
            .collect::<Vec<Vec<u8>>>();

        if numbers.len() == 1 {
            break;
        }
    }

    numbers
        .last()
        .unwrap()
        .iter()
        .fold(0, |acc, i| acc << 1 ^ *i as i32)
}

fn parse_input(data: &Vec<u8>) -> Vec<Vec<u8>> {
    let bytes: Vec<u8> = data
        .iter()
        .map(|b| match b {
            b'0' => 0,
            b'1' => 1,
            b'\n' => 2,
            _ => panic!("Invalid input"),
        })
        .collect();

    bytes.split(|b| *b == 2).map(|arr| arr.to_vec()).collect()
}

pub fn input() -> Vec<Vec<u8>> {
    let data = read("inputs/day.txt").unwrap();
    parse_input(&data)
}

pub fn part1(numbers: &Vec<Vec<u8>>) -> i32 {
    let mut counter: Vec<Counter> = vec![];

    numbers
        .iter()
        .for_each(|number| count_bits(&mut counter, number));

    let gamma = counter
        .iter()
        .fold(0, |acc, count| acc << 1 ^ (count.ones > count.zeros) as i32);

    let epsilon = counter
        .iter()
        .fold(0, |acc, count| acc << 1 ^ (count.ones < count.zeros) as i32);

    gamma * epsilon
}

pub fn part2(numbers: &Vec<Vec<u8>>) -> i32 {
    let oxigen = calculate_dynamic(numbers.clone(), |counter, i, &number| {
        if counter[i].ones >= counter[i].zeros {
            number[i] == 1
        } else {
            number[i] == 0
        }
    });

    let co2 = calculate_dynamic(numbers.clone(), |counter, i, &number| {
        if counter[i].ones >= counter[i].zeros {
            number[i] == 0
        } else {
            number[i] == 1
        }
    });

    oxigen * co2
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn input() -> Vec<Vec<u8>> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(|n| {
            n.chars()
                .map(|c| c.to_digit(2).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 230);
    }
}
