use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Result;

#[derive(Debug, Clone)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    // Parse "#1 @ 185,501: 17x15" into a Claim
    fn from(claim_str: String) -> Result<Self> {
        let parts: Vec<&str> = claim_str.split_whitespace().collect();

        let id: u32 = parts[0].replace("#", "").parse()?;

        let position_parts: Vec<&str> = parts[2].split(",").collect();
        let left: u32 = position_parts[0].parse()?;
        let top: u32 = position_parts[1].replace(":", "").parse()?;

        let size_parts: Vec<&str> = parts[3].split("x").collect();
        let width: u32 = size_parts[0].parse()?;
        let height: u32 = size_parts[1].parse()?;

        Ok(Claim {
            id,
            left,
            top,
            width,
            height,
        })
    }

    fn points(&self) -> HashSet<(u32, u32)> {
        let mut all_points = HashSet::new();

        for x in 0..self.width {
            for y in 0..self.height {
                all_points.insert((self.left + x, self.top + y));
            }
        }
        return all_points;
    }
}

pub fn part1() -> Result<String> {
    let mut fabric_claims = HashMap::new();

    for claim in claims()? {
        for point in claim.points() {
            fabric_claims
                .entry(point)
                .or_insert_with(Vec::new)
                .push(claim.id);
        }
    }

    let answer = fabric_claims
        .values()
        .filter(|claims| claims.len() > 1)
        .count();

    Ok(answer.to_string())
}

pub fn part2() -> Result<String> {
    let all_claims = claims()?;

    let mut fabric_claims = HashMap::new();
    for claim in &all_claims {
        for point in claim.points() {
            fabric_claims
                .entry(point)
                .or_insert_with(Vec::new)
                .push(claim.id);
        }
    }

    let mut overlapping_claims = HashSet::new();
    for (_, ids) in &fabric_claims {
        if ids.len() > 1 {
            for id in ids {
                overlapping_claims.insert(id);
            }
        }
    }

    let leftover: Vec<_> = all_claims
        .iter()
        .filter(|c| !overlapping_claims.contains(&c.id))
        .collect();

    let claim_id = &leftover[0].id;
    Ok(claim_id.to_string())
}

fn claims() -> Result<Vec<Claim>> {
    let file = File::open("data/day3_input.txt")?;
    let reader = BufReader::new(file);

    return reader
        .lines()
        .flat_map(|line| line)
        .map(|line| Claim::from(line))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        let claim = Claim {
            id: 1,
            left: 1,
            top: 3,
            height: 4,
            width: 4,
        };

        assert_eq!(
            claim.points(),
            [
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (3, 3),
                (3, 4),
                (3, 5),
                (3, 6),
                (4, 3),
                (4, 4),
                (4, 5),
                (4, 6)
            ]
            .iter()
            .cloned()
            .collect()
        );
    }
}
