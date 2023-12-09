fn main() {
    let input = include_str!("../data/day-01.txt");
    println!("Part 1: {}", Part1::solve(input));
    println!("Part 2: {}", Part2::solve(input));
}

trait Solver {
    fn solve(input: &str) -> usize {
        input.lines().filter_map(Self::parse_line).sum()
    }

    fn parse_line(line: &str) -> Option<usize> {
        let first = Self::first_digit(line)?;
        let last = Self::last_digit(line)?;
        let num = format!("{first}{last}");
        num.parse().ok()
    }

    fn first_digit(line: &str) -> Option<char>;

    fn last_digit(line: &str) -> Option<char>;
}

struct Part1;

impl Solver for Part1 {
    fn first_digit(line: &str) -> Option<char> {
        line.chars().find(|c| c.is_digit(10))
    }

    fn last_digit(line: &str) -> Option<char> {
        line.chars().rev().find(|c| c.is_digit(10))
    }
}

struct Part2;

impl Solver for Part2 {
    fn first_digit(line: &str) -> Option<char> {
        line.char_indices().find_map(|(i, c)| {
            if c.is_digit(10) {
                Some(c)
            } else {
                match &line[..=i] {
                    s if s.ends_with("zero") => Some('0'),
                    s if s.ends_with("one") => Some('1'),
                    s if s.ends_with("two") => Some('2'),
                    s if s.ends_with("three") => Some('3'),
                    s if s.ends_with("four") => Some('4'),
                    s if s.ends_with("five") => Some('5'),
                    s if s.ends_with("six") => Some('6'),
                    s if s.ends_with("seven") => Some('7'),
                    s if s.ends_with("eight") => Some('8'),
                    s if s.ends_with("nine") => Some('9'),
                    _ => None,
                }
            }
        })
    }

    fn last_digit(line: &str) -> Option<char> {
        line.char_indices().rev().find_map(|(i, c)| {
            if c.is_digit(10) {
                Some(c)
            } else {
                match &line[i..] {
                    s if s.starts_with("zero") => Some('0'),
                    s if s.starts_with("one") => Some('1'),
                    s if s.starts_with("two") => Some('2'),
                    s if s.starts_with("three") => Some('3'),
                    s if s.starts_with("four") => Some('4'),
                    s if s.starts_with("five") => Some('5'),
                    s if s.starts_with("six") => Some('6'),
                    s if s.starts_with("seven") => Some('7'),
                    s if s.starts_with("eight") => Some('8'),
                    s if s.starts_with("nine") => Some('9'),
                    _ => None,
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";
        assert_eq!(Part1::solve(input), 142)
    }

    #[test]
    fn test_part2() {
        let input = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";
        assert_eq!(Part2::solve(input), 281)
    }
}
