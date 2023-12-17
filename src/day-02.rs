use std::str::FromStr;

fn main() {
    let input = include_str!("../data/day-02.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| line.parse::<Game>().ok())
        .filter(|game| {
            let (max_r, max_g, max_b) = game.plays.iter().fold((0, 0, 0), |(r, g, b), play| {
                (r.max(play.red), g.max(play.green), b.max(play.blue))
            });
            max_r <= 12 && max_g <= 13 && max_b <= 14
        })
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| line.parse::<Game>().ok())
        .map(|game| {
            let (max_r, max_g, max_b) = game.plays.iter().fold((0, 0, 0), |(r, g, b), play| {
                (r.max(play.red), g.max(play.green), b.max(play.blue))
            });
            max_r * max_g * max_b
        })
        .sum()
}

#[derive(Debug)]
struct Game {
    id: usize,
    plays: Vec<Play>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with("Game") {
            let (game, plays) = s.split_once(":").ok_or("invalid game")?;
            let (_, id) = game.split_once(" ").ok_or("invalid game")?;
            let id = id.parse().map_err(|_| "invalid game id")?;
            let plays: Result<_, _> = plays.split(";").map(|s| s.parse()).collect();
            Ok(Game { id, plays: plays? })
        } else {
            Err("invalid game")
        }
    }
}

#[derive(Debug, Default)]
struct Play {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Play {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        s.split(", ").try_fold(Play::default(), |play, cube| {
            let (count, color) = cube.split_once(" ").ok_or("invalid play")?;
            let count = count.parse().map_err(|_| "invalid cube count")?;
            match color {
                "red" => Ok(Play { red: count, ..play }),
                "green" => Ok(Play {
                    green: count,
                    ..play
                }),
                "blue" => Ok(Play {
                    blue: count,
                    ..play
                }),
                _ => Err("invalid cube color"),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        assert_eq!(part1(input), 8)
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        assert_eq!(part2(input), 2286)
    }
}
