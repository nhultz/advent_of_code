use crate::{file_input, Result};
use nom::{bytes::complete::tag, character::complete::u32, sequence::tuple, IResult};
use std::collections::HashMap;

pub fn part1() -> Result<String> {
    let input = file_input("data/2021/day5_input.txt")?;
    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2021/day5_input.txt")?;
    solve_part2(input)
}

fn solve_part1<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let mut board: HashMap<Point, u64> = HashMap::default();
    let parsed_points = parse_points(input)?
        .into_iter()
        // Only horizontal or vertical lines
        .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y);

    for (first_point, second_point) in parsed_points {
        let line = first_point.to(&second_point);
        for point in line {
            let counter = board.entry(point).or_insert(0);
            *counter += 1;
        }
    }

    let count_overlapping = board.iter().filter(|(_, count)| **count > 1).count();
    Ok(format!("{}", count_overlapping))
}

fn solve_part2<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let mut board: HashMap<Point, u64> = HashMap::default();
    let parsed_points = parse_points(input);

    for (first_point, second_point) in parsed_points? {
        let line = first_point.to(&second_point);
        for point in line {
            let counter = board.entry(point).or_insert(0);
            *counter += 1;
        }
    }

    let count_overlapping = board.iter().filter(|(_, count)| **count > 1).count();
    Ok(format!("{}", count_overlapping))
}

fn parse_points<T>(input: T) -> Result<Vec<(Point, Point)>>
where
    T: Iterator<Item = String>,
{
    input.map(|line| input_line(&line)).collect()
}

fn point_coords(input: &str) -> IResult<&str, Point> {
    let (i, (x, _, y)) = tuple((u32, tag(","), u32))(input)?;
    Ok((i, Point::new(x, y)))
}

fn input_line(input: &str) -> Result<(Point, Point)> {
    let (_, (p1, _, p2)) = tuple((point_coords, tag(" -> "), point_coords))(input).unwrap();
    Ok((p1, p2))
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }

    fn to(&self, other: &Point) -> PointIter {
        PointIter::new(self.clone(), other.clone())
    }
}

struct PointIter {
    start: Point,
    end: Point,
    done: bool,
}

impl PointIter {
    fn new(start: Point, end: Point) -> Self {
        PointIter {
            start,
            end,
            done: false,
        }
    }
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let cur_point = self.start;

        if cur_point == self.end {
            self.done = true;
            return Some(cur_point);
        }

        if self.start.x < self.end.x {
            self.start.x += 1;
        } else if self.start.x > self.end.x {
            self.start.x -= 1;
        };

        if self.start.y < self.end.y {
            self.start.y += 1;
        } else if self.start.y > self.end.y {
            self.start.y -= 1;
        };

        Some(cur_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_points() {
        assert_eq!(point_tuple("35,336"), Ok(("", Point { x: 35, y: 336 })));
    }

    #[test]
    fn test_forward_line() -> Result<()> {
        let start_point = Point::new(0, 9);
        let end_point = Point::new(5, 9);

        let line: Vec<Point> = start_point.to(&end_point).collect();

        let expected = vec![
            Point::new(0, 9),
            Point::new(1, 9),
            Point::new(2, 9),
            Point::new(3, 9),
            Point::new(4, 9),
            Point::new(5, 9),
        ];

        assert_eq!(line, expected);

        Ok(())
    }

    #[test]
    fn test_backward_line() -> Result<()> {
        let start_point = Point::new(9, 4);
        let end_point = Point::new(3, 4);

        let line: Vec<Point> = start_point.to(&end_point).collect();

        let expected = vec![
            Point::new(9, 4),
            Point::new(8, 4),
            Point::new(7, 4),
            Point::new(6, 4),
            Point::new(5, 4),
            Point::new(4, 4),
            Point::new(3, 4),
        ];

        assert_eq!(line, expected);

        Ok(())
    }

    // #[test]
    // fn test_diag_line() -> Result<()> {
    //     Ok(())
    // }

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let mut i = input.iter().map(|line| line.to_string());

        let answer = solve_part1(&mut i)?;
        assert_eq!(answer, "5");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let mut i = input.iter().map(|line| line.to_string());

        let answer = solve_part2(&mut i)?;
        assert_eq!(answer, "12");

        Ok(())
    }
}
