pub fn input() -> String {
    String::from_utf8(include_bytes!("input.txt").to_vec()).unwrap()
}

pub fn part1(_a: String) -> i8 {
    1
}

pub fn part2(_a: String) -> i8 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1);
    }
}
