use crate::{file_input, Result};

pub fn part1() -> Result<String> {
    let entries = password_entries()?;

    let num_valid_passwords = entries.iter()
        .filter(|p| p.is_valid_part1())
        .count();

    Ok(format!("{}", num_valid_passwords))
}

pub fn part2() -> Result<String> {
    let entries = password_entries()?;

    let num_valid_passwords = entries.iter()
        .filter(|p| p.is_valid_part2())
        .count();

    Ok(format!("{}", num_valid_passwords))
}

fn password_entries() -> Result<Vec<PasswordEntry>> {
    return file_input("data/2020/day2_input.txt")?
        .map(|line| PasswordEntry::from(line))
        .collect();
}

#[derive(Debug)]
struct PasswordEntry {
    min: u32,
    max: u32,
    letter: char,
    password: String
}

impl PasswordEntry {
    // Parse "8-13 v: qgmgcrxvdvkbs" into a PasswordEntry
    fn from(entry_str: String) -> Result<Self> {
        let parts: Vec<&str> = entry_str.split_whitespace().collect();

        let bounds: Vec<&str> = parts[0].split("-").collect();
        let min: u32 = bounds[0].parse()?;
        let max: u32 = bounds[1].parse()?;

        let letter: char = parts[1].replace(":", "").parse()?;

        let password = parts[2].to_string();

        Ok(PasswordEntry{
            min,
            max,
            letter,
            password,
        })
    }

    fn is_valid_part1(&self) -> bool {
        let count = self.password.chars().fold(0, |acc, x| {
            match x {
                x if x == self.letter => acc + 1,
                _ => acc
            }
        });

        count >= self.min && count <= self.max
    }

    fn is_valid_part2(&self) -> bool {
        let position_1 = self.password.chars().nth((self.min - 1) as usize).unwrap();
        let position_2 = self.password.chars().nth((self.max - 1) as usize).unwrap();

        match (position_1, position_2) {
            (x, y) if x == self.letter && y != self.letter => true,
            (x, y) if x != self.letter && y == self.letter => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_parse() -> Result<()> {
        let input = String::from("8-13 v: qgmgcrxvdvkbs");
        let entry = PasswordEntry::from(input)?;

        assert_eq!(entry.min, 8);
        assert_eq!(entry.max, 13);
        assert_eq!(entry.letter, 'v');
        assert_eq!(entry.password, "qgmgcrxvdvkbs");

        Ok(())
    }

    #[test]
    fn test_password_parse_another() -> Result<()> {
        let input = String::from("10-11 m: mmmmmmmmmmc");
        let entry = PasswordEntry::from(input)?;

        assert_eq!(entry.min, 10);
        assert_eq!(entry.max, 11);
        assert_eq!(entry.letter, 'm');
        assert_eq!(entry.password, "mmmmmmmmmmc");

        Ok(())
    }
}
