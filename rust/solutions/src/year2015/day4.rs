use aoc_derive::{aoc, aoc_input};

use md5::{Digest, Md5};

#[aoc_input(day4)]
fn parse(input: &str) -> String {
    input.trim().to_string()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    let mut n = 0;
    let mut keep_going = true;

    while keep_going {
        n += 1;
        let hash = Md5::digest(format!("{}{}", input, n));
        if hash[0] == 0 && hash[1] == 0 && hash[2] <= 15 {
            keep_going = false;
        }
    }

    n
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u64 {
    let mut n = 0;
    let mut keep_going = true;

    while keep_going {
        n += 1;
        let hash = Md5::digest(format!("{}{}", input, n));
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            keep_going = false;
        }
    }

    n
}
