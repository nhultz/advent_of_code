use crate::{file_input, Result};

pub fn part1() -> Result<String> {
    let entries: Vec<String> = file_input("data/2020/day3_input.txt")?.collect();

    let num_trees = count_trees(&entries, 3, 1);

    Ok(format!("{}", num_trees))
}

pub fn part2() -> Result<String> {
    let entries: Vec<String> = file_input("data/2020/day3_input.txt")?.collect();

    // Possible slopes: (right, down)
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let answer: u64 = slopes
        .iter()
        .map(|slope| count_trees(&entries, slope.0, slope.1))
        .product();

    Ok(format!("{}", answer))
}

fn count_trees(map: &Vec<String>, right: usize, down: usize) -> u64 {
    let mut num_trees = 0;

    let mut cur_x = 0;
    let mut cur_y = 0;

    while cur_y < map.len() {
        if (map[cur_y].chars().nth(cur_x)) == Some('#') {
            num_trees += 1;
        }

        cur_x = (cur_x + right) % map[cur_y].len();
        cur_y = cur_y + down
    }

    return num_trees;
}
