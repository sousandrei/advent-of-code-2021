use std::fs::read;

fn parse_input(data: &[u8]) -> Vec<u8> {
    data.to_owned()
}

pub fn input() -> Vec<u8> {
    let data = read("inputs/day9.txt").unwrap();
    parse_input(&data)
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn count_basin(data: &[u8], row_len: usize, start_index: usize, visited: &[usize]) -> Vec<usize> {
    if data[start_index] == b'9' {
        return visited.to_vec();
    }

    let mut visited = visited.to_owned();
    visited.push(start_index);

    // ======================================
    // left
    let l_index = start_index.checked_sub(1);

    if let Some(i) = l_index {
        if !visited.contains(&i) && start_index % row_len != 0 {
            visited = count_basin(data, row_len, i, &visited);
        }
    }
    // ======================================
    // right
    let r_index = start_index.checked_add(1);

    if let Some(i) = r_index {
        if !visited.contains(&i) && start_index < data.len() && (start_index + 1) % row_len != 0 {
            visited = count_basin(data, row_len, i, &visited);
        }
    }
    // ======================================
    // up
    let u_index = start_index.checked_sub(row_len);

    if let Some(i) = u_index {
        if !visited.contains(&i) && start_index % row_len != 0 {
            visited = count_basin(data, row_len, i, &visited);
        }
    }
    // ======================================
    // down
    let d_index = start_index.checked_add(row_len);

    if let Some(i) = d_index {
        if !visited.contains(&i) && start_index < data.len() && i < data.len() {
            visited = count_basin(data, row_len, i, &visited);
        }
    }
    // ======================================

    visited
}

pub fn part1(data: &[u8]) -> i32 {
    let row_len = data.iter().position(|n| *n == b'\n').unwrap();

    let data: Vec<u8> = data
        .into_iter()
        .filter(|n| **n != b'\n')
        .map(|n| n.to_owned())
        .collect();

    let mut sum: i32 = 0;

    for (index, num) in data.iter().enumerate() {
        let check = |dir: Direction| {
            let other_i = match dir {
                Direction::Left => index.checked_sub(1),
                Direction::Right => index.checked_add(1),
                Direction::Up => index.checked_sub(row_len),
                Direction::Down => index.checked_add(row_len),
            };

            if other_i.is_none() {
                return true;
            }

            let other_i = other_i.unwrap();

            if match dir {
                Direction::Left => other_i == row_len - 1 || index % row_len == 0,
                Direction::Right => {
                    other_i > data.len() - 1 || (index > row_len && (index + 1) % row_len == 0)
                }
                Direction::Up => index < row_len,
                Direction::Down => other_i > data.len() - 1,
            } {
                return true;
            }

            num < &data[other_i]
        };

        if check(Direction::Left)
            && check(Direction::Right)
            && check(Direction::Up)
            && check(Direction::Down)
        {
            sum += (*num as char).to_digit(10).unwrap_or(0) as i32 + 1;
        }
    }

    sum
}

pub fn part2(data: &[u8]) -> i32 {
    let row_len = data.iter().position(|n| *n == b'\n').unwrap();

    let data: Vec<u8> = data
        .into_iter()
        .filter(|n| **n != b'\n')
        .map(|n| n.to_owned())
        .collect();

    let mut basins = vec![];

    for (index, num) in data.iter().enumerate() {
        let check = |dir: Direction| {
            let other_index = match dir {
                Direction::Left => index.checked_sub(1),
                Direction::Right => index.checked_add(1),
                Direction::Up => index.checked_sub(row_len),
                Direction::Down => index.checked_add(row_len),
            };

            if other_index.is_none() {
                return true;
            }

            let other_i = other_index.unwrap();

            if match dir {
                Direction::Left => other_i == row_len - 1 || index % row_len == 0,
                Direction::Right => {
                    other_i > data.len() - 1 || (index > row_len && (index + 1) % row_len == 0)
                }
                Direction::Up => index < row_len,
                Direction::Down => other_i > data.len() - 1,
            } {
                return true;
            }

            num < &data[other_i]
        };

        if check(Direction::Left)
            && check(Direction::Right)
            && check(Direction::Up)
            && check(Direction::Down)
        {
            let visited = count_basin(&data, row_len, index, &vec![]);
            basins.push(visited.len());
        }
    }

    let mut res = basins.to_vec();
    res.sort_unstable_by(|a, b| b.cmp(a));

    res[0..=2].into_iter().fold(1, |acc, n| acc * *n as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<u8> {
        let data = "2199943210
3987894921
9856789892
8767896789
9899965678"
            .as_bytes();
        parse_input(&data)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1134);
    }
}
