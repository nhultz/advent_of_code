use aoc_derive::{aoc, aoc_input};

use std::collections::HashSet;

#[aoc_input(day3)]
fn parse(input: &str) -> Vec<char> {
    input.lines().map(|l| l.chars()).flatten().collect()
}

#[aoc(day3, part1)]
fn part1(input: &[char]) -> usize {
    let mut houses: HashSet<(i32, i32)> = HashSet::default();

    let mut x = 0;
    let mut y = 0;

    houses.insert((x, y));

    for dir in input {
        match dir {
            '^' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            'v' => y -= 1,
            _ => {}
        }
        houses.insert((x, y));
    }

    houses.len()
}

#[aoc(day3, part2)]
fn part2(input: &[char]) -> usize {
    let mut houses: HashSet<(i32, i32)> = HashSet::default();

    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;

    houses.insert((santa_x, santa_y));
    houses.insert((robo_x, robo_y));

    let mut santa_turn = true;

    for dir in input {
        let (x, y) = match santa_turn {
            true => (&mut santa_x, &mut santa_y),
            false => (&mut robo_x, &mut robo_y),
        };

        match dir {
            '^' => *y += 1,
            '<' => *x -= 1,
            '>' => *x += 1,
            'v' => *y -= 1,
            _ => {}
        }
        houses.insert((*x, *y));
        santa_turn = !santa_turn;
    }

    houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "^v^v^v^v^v";

    #[test]
    fn test_part1() {
        let entries = parse(INPUT);
        assert_eq!(part1(&entries), 2);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT);
        assert_eq!(part2(&entries), 11);
    }
}
