fn main() {
    let input = include_str!("../data/day-01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_line(line: &str) -> Option<usize> {
    let digits: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
    let first = digits.first()?;
    let last = digits.last()?;
    let num = format!("{first}{last}");
    num.parse().ok()
}

fn part1(input: &str) -> usize {
    input.lines().filter_map(parse_line).sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let new_line = line
                .replace("zero", "z0o")
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            parse_line(&new_line)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        assert_eq!(part1(input), 142)
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        assert_eq!(part2(input), 281)
    }
}
