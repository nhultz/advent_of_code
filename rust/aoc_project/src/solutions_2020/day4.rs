use crate::{file_input, Result};
use std::collections::HashMap;

pub fn part1() -> Result<String> {
    let passport_data: Vec<String> = file_input("data/2020/day4_input.txt")?.collect();

    let num_valid = split_by_newline(&passport_data)
        .map(|fields| PassportEntry::from(fields))
        .filter(|p| p.has_required_fields())
        .count();

    Ok(format!("{}", num_valid))
}

pub fn part2() -> Result<String> {
    let passport_data: Vec<String> = file_input("data/2020/day4_input.txt")?.collect();

    let num_valid = split_by_newline(&passport_data)
        .map(|fields| PassportEntry::from(fields))
        .filter(|p| p.has_valid_data())
        .count();

    Ok(format!("{}", num_valid))
}

fn split_by_newline(rows: &Vec<String>) -> impl Iterator<Item = HashMap<&str, &str>> {
    let mut records = vec![];

    let mut fields = HashMap::new();
    for row in rows {
        if row.is_empty() {
            records.push(fields);
            fields = HashMap::new();
        }

        let entries: HashMap<&str, &str> = row
            .split_whitespace()
            .map(|item| item.split(':').collect())
            .map(|item: Vec<&str>| (item[0], item[1]))
            .collect();

        fields.extend(entries);
    }

    if !fields.is_empty() {
        records.push(fields);
    }

    return records.into_iter();
}

#[derive(Debug)]
struct PassportEntry {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl PassportEntry {
    fn from(items: HashMap<&str, &str>) -> Self {
        PassportEntry {
            birth_year: items.get("byr").map(|s| s.to_string()),
            issue_year: items.get("iyr").map(|s| s.to_string()),
            expiration_year: items.get("eyr").map(|s| s.to_string()),
            height: items.get("hgt").map(|s| s.to_string()),
            hair_color: items.get("hcl").map(|s| s.to_string()),
            eye_color: items.get("ecl").map(|s| s.to_string()),
            passport_id: items.get("pid").map(|s| s.to_string()),
            country_id: items.get("cid").map(|s| s.to_string()),
        }
    }

    fn valid_year(&self, value: &Option<String>, start_year: u32, end_year: u32) -> bool {
        value
            .as_ref()
            .map(|n| n.parse::<u32>().ok())
            .flatten()
            .filter(|n| n >= &start_year && n <= &end_year)
            .is_some()
    }

    fn valid_height(&self, value: &Option<String>) -> bool {
        value
            .as_ref()
            .map(|n| {
                let num_part = &n[0..(n.len() - 2)];
                let height = num_part.parse::<u32>().ok();

                match height {
                    Some(x) => (&n[(n.len() - 2)..], x),
                    None => ("xx", 0),
                }
            })
            .filter(|(unit_type, height)| match *unit_type {
                "cm" => height >= &150 && height <= &193,
                "in" => height >= &59 && height <= &76,
                _ => false,
            })
            .is_some()
    }

    fn valid_hair_color(&self, value: &Option<String>) -> bool {
        value
            .as_ref()
            .filter(|s| s.starts_with('#'))
            .filter(|s| s.replace("#", "").len() == 6)
            .filter(|s| {
                let color = s
                    .chars()
                    .filter(|c| match c {
                        'a'..='f' => true,
                        '0'..='9' => true,
                        _ => false,
                    })
                    .collect::<String>();

                color.len() == 6
            })
            .is_some()
    }

    fn valid_eye_color(&self, value: &Option<String>) -> bool {
        value
            .as_ref()
            .filter(|s| match s.as_str() {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false,
            })
            .is_some()
    }

    fn valid_passport(&self, value: &Option<String>) -> bool {
        value
            .as_ref()
            .filter(|s| s.len() == 9)
            .filter(|s| {
                s.chars()
                    .filter(|c| !c.is_numeric())
                    .collect::<String>()
                    .is_empty()
            })
            .is_some()
    }

    fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn has_valid_data(&self) -> bool {
        self.valid_year(&self.birth_year, 1920, 2002)
            && self.valid_year(&self.issue_year, 2010, 2020)
            && self.valid_year(&self.expiration_year, 2020, 2030)
            && self.valid_height(&self.height)
            && self.valid_eye_color(&self.eye_color)
            && self.valid_hair_color(&self.hair_color)
            && self.valid_passport(&self.passport_id)
    }
}
