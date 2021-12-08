use std::fs::read_to_string;

fn count_digits(n: &str) -> Option<usize> {
    let mut digit: u8 = 0;

    for c in n.chars() {
        let bit = match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => unreachable!("invalid char"),
        };

        digit ^= 1 << bit;
    }

    match digit.count_ones() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

fn parse_input(data: &str) -> Vec<String> {
    data.split('\n').map(|s| s.to_owned()).collect()
}

pub fn input() -> Vec<String> {
    let data = read_to_string("inputs/day8.txt").unwrap();
    parse_input(&data)
}

fn diff_str(a: &str, b: &str) -> String {
    let mut a = a.to_owned();
    for bc in b.chars() {
        a = a.replace(bc, "");
    }

    a
}

fn sort_str(a: &str) -> String {
    let mut v = a.split("").collect::<Vec<&str>>();
    v.sort_unstable();
    v.join("")
}

pub fn part1(lines: &[String]) -> i32 {
    let mut count = 0;
    for line in lines {
        let parts = line.split(" | ");
        let output = parts.last().unwrap().to_owned();

        let code = output.split(' ');

        for c in code {
            if count_digits(c).is_some() {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(data: &[String]) -> i32 {
    let mut sum = 0;

    for line in data {
        let raw_line = line.replace(" | ", " ");
        let raw = raw_line.split(' ').collect::<Vec<&str>>();

        let d1 = raw.iter().find(|d| d.len() == 2).unwrap();
        let d4 = raw.iter().find(|d| d.len() == 4).unwrap();
        let d7 = raw.iter().find(|d| d.len() == 3).unwrap();
        let d8 = raw.iter().find(|d| d.len() == 7).unwrap();

        let d3 = raw.iter().find(|d| diff_str(d, d1).len() == 3).unwrap();
        let d6 = raw
            .iter()
            .find(|d| diff_str(d, d1).len() == 5 && *d != d8)
            .unwrap();

        let d0 = raw
            .iter()
            .find(|d| d.len() == 6 && diff_str(d, d4).len() == 3 && *d != d6)
            .unwrap();
        let d2 = raw
            .iter()
            .find(|d| d.len() == 5 && diff_str(d, d4).len() == 3)
            .unwrap();
        let d5 = raw
            .iter()
            .find(|d| d.len() == 5 && diff_str(d, d4).len() == 2 && *d != d3)
            .unwrap();
        let d9 = raw
            .iter()
            .find(|d| d.len() == 6 && diff_str(d, d4).len() == 2)
            .unwrap();

        let ds = vec![d0, d1, d2, d3, d4, d5, d6, d7, d8, d9];

        let ds = ds.iter().map(|s| sort_str(s));

        let output = line.split(" | ").last().unwrap().split(' ');

        let num = output
            .into_iter()
            .map(|o| {
                let o = sort_str(o);
                let (i, _) = ds.clone().enumerate().find(|(_, d)| d == &o).unwrap();
                i
            })
            .map(|i| i.to_string())
            .collect::<Vec<String>>();

        let num = num.join("").parse::<i32>().unwrap();

        sum += num;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        let data =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        parse_input(&data)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                // &parse_input("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")
                &input()
            ),
            61229
        );
    }
}
