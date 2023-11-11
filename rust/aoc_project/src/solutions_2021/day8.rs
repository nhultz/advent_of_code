use crate::{file_input, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alpha1, space0, space1},
    character::{is_alphabetic, is_space},
    combinator::{map, rest},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

pub fn part1() -> Result<String> {
    let input = file_input("data/2021/day8_input.txt")?;
    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2021/day8_input.txt")?;
    solve_part2(input)
}

fn solve_part1<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let num_unique: usize = input
        .map(|line| parse_output(&line))
        .map(|o| o.num_unique_digits())
        .sum();

    Ok(format!("{}", num_unique))
}

fn solve_part2<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    // Target num: 5040
    let all_possible: Vec<Vec<&Segment>> = Segment::VALUES.iter().permutations(7).collect();
    println!("{} possible choices", all_possible.len());

    let entries: Vec<DisplayEntry> = input.map(|line| parse_entry(&line)).collect();

    // let remaining_after_one = all_possible.iter().filter(|choice| {
    //     entries[0].signals[0]
    //         .possible_positions()
    //         .iter()
    //         .all(|(pos, segments)| {
    //             let choice_val = choice[*idx];
    //             if vals.contains(&choice_val) {
    //                 true
    //             } else {
    //                 false
    //             }
    //         })
    // // });
    // println!("{} remaining choices", remaining_after_one.count());

    // be
    // (2, b-1)
    // (5, b-1)
    // (2, e-4)
    // (5, e-4)

    // edb
    //

    // let possible_one = vec![(2, [1, 4]), (5, [1, 4])];

    // let possible_seven = vec![(0, [1, 3, 4]), (2, [1, 3, 4]), (5, [1, 3, 4])];
    // let remaining_after_seven = perms.iter().filter(|choice| {
    //     possible_seven.iter().all(|(idx, vals)| {
    //         let choice_val = choice[*idx];
    //         if vals.contains(&choice_val) {
    //             true
    //         } else {
    //             false
    //         }
    //     })
    // });
    // println!("{} remaining choices", remaining_after_seven.count());

    // //cgeb
    // // a=0
    // // b=1
    // // c=2
    // // d=3
    // // e=4
    // // f=5
    // // g=6

    // let possible_four = vec![
    //     (1, [1, 2, 4, 6]),
    //     (2, [1, 2, 4, 6]),
    //     (3, [1, 2, 4, 6]),
    //     (5, [1, 2, 4, 6]),
    // ];

    // let remaining_after_four = perms.iter().filter(|choice| {
    //     possible_four.iter().all(|(idx, vals)| {
    //         let choice_val = choice[*idx];
    //         if vals.contains(&choice_val) {
    //             true
    //         } else {
    //             false
    //         }
    //     })
    // });
    // println!("{} remaining choices", remaining_after_four.count());

    // input
    //     .map(|line| parse_entry(&line))
    //     // .map(|o| o.to_parsed_digits())
    //     .for_each(|d| println!("{:#?}", d));

    // // let output_sum: usize = input
    // //     .map(|line| parse_output(&line))
    // //     .map(|o| o.to_parsed_digits())
    // //     .flatten()
    // //     .sum();

    // let output_sum = 15;
    // Ok(format!("{}", output_sum))

    Ok(format!("{}", "part2"))
}

fn char_or_space(input: &str) -> IResult<&str, &str> {
    alt((alpha1, space1))(input)
}

fn parse_output(input: &str) -> OutputValue {
    let (_, ((_, _), output_digits)) =
        tuple((many_till(char_or_space, tag("|")), many1(char_or_space)))(input)
            .expect("Got bad input");

    let output_digits: Vec<String> = output_digits
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    OutputValue {
        digits: output_digits,
    }
}

#[derive(Debug)]
struct OutputValue {
    digits: Vec<String>,
}

impl OutputValue {
    fn num_unique_digits(&self) -> usize {
        self.digits
            .iter()
            .map(|o| o.chars().count())
            .filter(|char_count| match char_count {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count()
    }

    fn to_parsed_digits(&self) -> Vec<usize> {
        self.digits
            .iter()
            .map(|digit_str| {
                let mut bits: u8 = 0b0000_0000;

                digit_str.chars().for_each(|c| match c {
                    'a' => bits |= 0b1000_0000,
                    'b' => bits |= 0b0100_0000,
                    'c' => bits |= 0b0010_0000,
                    'd' => bits |= 0b0001_0000,
                    'e' => bits |= 0b0000_1000,
                    'f' => bits |= 0b0000_0100,
                    'g' => bits |= 0b0000_0010,
                    _ => {}
                });

                match bits {
                    0b1100_0000 => 1,
                    _ => 0,
                }
            })
            .collect()
    }
}

fn pattern(input: &str) -> IResult<&str, SignalPattern> {
    map(delimited(space0, alpha1, space0), |s: &str| {
        SignalPattern::new(s.to_string())
    })(input)
}

fn parse_entry(input: &str) -> DisplayEntry {
    let mut parser = tuple((many1(pattern), tag("|"), many1(pattern)));
    let (_, (signals, _, outputs)) = parser(input).expect("Got bad input");

    DisplayEntry::new(signals, outputs)
}

#[derive(Debug)]
struct DisplayEntry {
    signals: Vec<SignalPattern>,
    outputs: Vec<SignalPattern>,
}

impl DisplayEntry {
    fn new(signals: Vec<SignalPattern>, outputs: Vec<SignalPattern>) -> Self {
        let mut signals = signals.clone();
        signals.sort_by(|a, b| a.pattern.len().cmp(&b.pattern.len()));

        DisplayEntry { signals, outputs }
    }
}

#[derive(Debug, Clone)]
struct SignalPattern {
    pattern: Vec<Segment>,
}

impl SignalPattern {
    fn new(pattern: String) -> Self {
        let segments = pattern.chars().map(|c| Segment::from_char(c)).collect();
        SignalPattern { pattern: segments }
    }

    fn possible_positions(&self) -> Vec<(DisplayPosition, Vec<Segment>)> {
        let segments = self.pattern.clone();

        self.number_from_len()
            .into_iter()
            .flatten()
            .zip(std::iter::repeat(segments))
            .collect()
    }

    fn number_from_len(&self) -> Vec<Vec<DisplayPosition>> {
        let mut possible_numbers = vec![];

        match self.pattern.len() {
            // #1
            2 => possible_numbers.push(vec![
                DisplayPosition::TopRight,
                DisplayPosition::BottomRight,
            ]),
            //# 7
            3 => possible_numbers.push(vec![
                DisplayPosition::Top,
                DisplayPosition::TopRight,
                DisplayPosition::BottomRight,
            ]),
            // #4
            4 => possible_numbers.push(vec![
                DisplayPosition::TopLeft,
                DisplayPosition::Middle,
                DisplayPosition::TopRight,
                DisplayPosition::BottomRight,
            ]),
            5 => {
                // #2
                possible_numbers.push(vec![
                    DisplayPosition::Top,
                    DisplayPosition::TopRight,
                    DisplayPosition::Middle,
                    DisplayPosition::BottomLeft,
                    DisplayPosition::Bottom,
                ]);
                // #3
                possible_numbers.push(vec![
                    DisplayPosition::Top,
                    DisplayPosition::TopRight,
                    DisplayPosition::Middle,
                    DisplayPosition::BottomRight,
                    DisplayPosition::Bottom,
                ]);
                // #5
                possible_numbers.push(vec![
                    DisplayPosition::Top,
                    DisplayPosition::TopLeft,
                    DisplayPosition::Middle,
                    DisplayPosition::BottomRight,
                    DisplayPosition::Bottom,
                ]);
            }
            6 => {
                // #0
                possible_numbers.push(vec![
                    DisplayPosition::Top,
                    DisplayPosition::TopLeft,
                    DisplayPosition::TopRight,
                    DisplayPosition::BottomLeft,
                    DisplayPosition::BottomRight,
                    DisplayPosition::Bottom,
                ]);
                // #6
                possible_numbers.push(vec![
                    DisplayPosition::Top,
                    DisplayPosition::TopLeft,
                    DisplayPosition::Middle,
                    DisplayPosition::BottomLeft,
                    DisplayPosition::BottomRight,
                    DisplayPosition::Bottom,
                ]);
                // #9
                possible_numbers.push(vec![
                    DisplayPosition::Top,
                    DisplayPosition::TopLeft,
                    DisplayPosition::TopRight,
                    DisplayPosition::Middle,
                    DisplayPosition::BottomLeft,
                    DisplayPosition::Bottom,
                ]);
            }
            // #8
            7 => possible_numbers.push(vec![
                DisplayPosition::Top,
                DisplayPosition::TopLeft,
                DisplayPosition::TopRight,
                DisplayPosition::Middle,
                DisplayPosition::BottomLeft,
                DisplayPosition::BottomRight,
                DisplayPosition::Bottom,
            ]),
            _ => (),
        };

        possible_numbers
    }
}

#[derive(Debug)]
enum DisplayPosition {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

#[derive(Debug, Clone)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    Unknown,
}

impl Segment {
    const VALUES: [Self; 7] = [
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
    ];

    fn from_char(c: char) -> Segment {
        match c {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => Segment::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ];
        let answer = solve_part1(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "26");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ];
        let answer = solve_part2(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "61229");

        Ok(())
    }
}
