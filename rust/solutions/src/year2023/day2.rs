use aoc_derive::{aoc, aoc_input};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use std::str::FromStr;

#[aoc_input(day2)]
fn parse(input: &str) -> Result<Vec<GameRecord>, String> {
    input.lines().map(|l| l.trim().parse()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[GameRecord]) -> u32 {
    input
        .into_iter()
        .filter(|r| r.is_possible())
        .map(|r| r.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[GameRecord]) -> u32 {
    input.into_iter().map(|r| r.power()).sum()
}

#[derive(Debug)]
pub struct GameRecord {
    id: u32,
    games: Vec<Game>,
}

impl GameRecord {
    fn is_possible(&self) -> bool {
        self.games.iter().all(|g| {
            (g.red.is_some_and(|n| n <= 12) || g.red.is_none())
                && (g.green.is_some_and(|n| n <= 13) || g.green.is_none())
                && (g.blue.is_some_and(|n| n <= 14) || g.blue.is_none())
        })
    }

    fn power(&self) -> u32 {
        let min_red = self.games.iter().filter_map(|g| g.red).max().unwrap_or(0);
        let min_green = self.games.iter().filter_map(|g| g.green).max().unwrap_or(0);
        let min_blue = self.games.iter().filter_map(|g| g.blue).max().unwrap_or(0);

        min_red * min_green * min_blue
    }
}

#[derive(Debug)]
struct Game {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Game {
    fn new() -> Self {
        Game {
            red: None,
            green: None,
            blue: None,
        }
    }
    fn add_color(&mut self, c: Color) {
        match c {
            Color::Red(n) => self.red = Some(n),
            Color::Green(n) => self.green = Some(n),
            Color::Blue(n) => self.blue = Some(n),
        }
    }
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

// Game 1: 6 green, 3 blue; 3 red, 1 green; 4 green, 3 red, 5 blue
impl FromStr for GameRecord {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let mut parser = tuple((game_id, separated_list1(tag(";"), game)));
        let (_, (id, games)) = parser(val).map_err(|e| format!("unable to parse line: {}", e))?;

        Ok(GameRecord { id, games })
    }
}

fn game_id(input: &str) -> IResult<&str, u32> {
    terminated(preceded(tag("Game "), u32), tag(":"))(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    map(
        separated_list1(tag(","), alt((red, green, blue))),
        |colors| {
            let mut g = Game::new();
            for c in colors {
                g.add_color(c);
            }
            g
        },
    )(input)
}

fn red(input: &str) -> IResult<&str, Color> {
    map(
        terminated(delimited(multispace0, u32, multispace0), tag("red")),
        |n| Color::Red(n),
    )(input)
}

fn green(input: &str) -> IResult<&str, Color> {
    map(
        terminated(delimited(multispace0, u32, multispace0), tag("green")),
        |n| Color::Green(n),
    )(input)
}

fn blue(input: &str) -> IResult<&str, Color> {
    map(
        terminated(delimited(multispace0, u32, multispace0), tag("blue")),
        |n| Color::Blue(n),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part1(&entries), 8);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part2(&entries), 2286);
    }
}
