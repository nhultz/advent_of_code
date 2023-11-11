use aoc_derive::{aoc, aoc_input};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space0},
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

#[aoc_input(day8)]
fn parse(input: &str) -> Result<Vec<DisplayEntry>, String> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day8, part1)]
fn part1(input: &[DisplayEntry]) -> usize {
    input
        .iter()
        .map(|entry| {
            entry
                .outputs
                .iter()
                .filter(|o| match o.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &[DisplayEntry]) -> u64 {
    input.iter().map(|entry| entry.decode()).sum()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Segment {
    val: String,
}

impl Segment {
    fn new(s: &str) -> Self {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));

        Segment {
            val: String::from_iter(chars),
        }
    }

    fn len(&self) -> usize {
        self.val.len()
    }

    fn contains(&self, other: &Segment) -> bool {
        other.val.chars().all(|c| self.val.contains(c))
    }

    fn num_overlapping(&self, other: &Segment) -> u32 {
        other
            .val
            .chars()
            .map(|c| match self.val.contains(c) {
                true => 1,
                false => 0,
            })
            .sum()
    }
}

#[derive(Debug)]
pub struct DisplayEntry {
    signals: Vec<Segment>,
    outputs: Vec<Segment>,
}

impl DisplayEntry {
    fn new(signals: Vec<&str>, outputs: Vec<&str>) -> Self {
        let signals = signals.into_iter().map(|s| Segment::new(s)).collect();
        let outputs = outputs.into_iter().map(|s| Segment::new(s)).collect();

        DisplayEntry { signals, outputs }
    }

    fn decode(&self) -> u64 {
        let one = self.find_one();
        let four = self.find_four();
        let seven = self.find_seven();
        let eight = self.find_eight();
        let three = self.find_three(one);
        let nine = self.find_nine(four);
        let two = self.find_two(three, four);
        let five = self.find_five(three, four);
        let zero = self.find_zero(one, nine);
        let six = self.find_six(one, nine);

        let mut m: HashMap<&Segment, char> = HashMap::new();
        m.insert(zero, '0');
        m.insert(one, '1');
        m.insert(two, '2');
        m.insert(three, '3');
        m.insert(four, '4');
        m.insert(five, '5');
        m.insert(six, '6');
        m.insert(seven, '7');
        m.insert(eight, '8');
        m.insert(nine, '9');

        self.outputs
            .iter()
            .map(|o| m.get(o).unwrap())
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn filter_by_count(&self, count: usize) -> impl Iterator<Item = &Segment> {
        self.signals.iter().filter(move |s| s.len() == count)
    }

    fn find_zero(&self, one: &Segment, nine: &Segment) -> &Segment {
        self.filter_by_count(6)
            .filter(|s| {
                if *s == nine {
                    return false;
                }

                s.contains(one)
            })
            .next()
            .unwrap()
    }

    fn find_one(&self) -> &Segment {
        self.filter_by_count(2).next().unwrap()
    }

    fn find_two(&self, three: &Segment, four: &Segment) -> &Segment {
        self.filter_by_count(5)
            .filter(|s| {
                if *s == three {
                    return false;
                }

                s.num_overlapping(four) == 2
            })
            .next()
            .unwrap()
    }

    fn find_three(&self, one: &Segment) -> &Segment {
        self.filter_by_count(5)
            .filter(|s| s.contains(one))
            .next()
            .unwrap()
    }

    fn find_four(&self) -> &Segment {
        self.filter_by_count(4).next().unwrap()
    }

    fn find_five(&self, three: &Segment, four: &Segment) -> &Segment {
        self.filter_by_count(5)
            .filter(|s| {
                if *s == three {
                    return false;
                }

                s.num_overlapping(four) == 3
            })
            .next()
            .unwrap()
    }

    fn find_six(&self, one: &Segment, nine: &Segment) -> &Segment {
        self.filter_by_count(6)
            .filter(|s| {
                if *s == nine {
                    return false;
                }

                s.num_overlapping(one) == 1
            })
            .next()
            .unwrap()
    }

    fn find_seven(&self) -> &Segment {
        self.filter_by_count(3).next().unwrap()
    }

    fn find_eight(&self) -> &Segment {
        self.filter_by_count(7).next().unwrap()
    }

    fn find_nine(&self, four: &Segment) -> &Segment {
        self.filter_by_count(6)
            .filter(|s| s.contains(four))
            .next()
            .unwrap()
    }
}

impl FromStr for DisplayEntry {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let mut parser = tuple((many1(entry_pattern), tag("|"), many1(entry_pattern)));
        let (_, (signals, _, outputs)) =
            parser(val).map_err(|e| format!("unable to parse line: {}", e))?;

        Ok(DisplayEntry::new(signals, outputs))
    }
}

fn entry_pattern(input: &str) -> IResult<&str, &str> {
    delimited(space0, alpha1, space0)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
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

    #[test]
    fn test_part1() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part1(&entries), 26);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part2(&entries), 61229);
    }
}
