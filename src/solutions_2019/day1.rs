use crate::file_input;
use crate::Result;

pub fn part1() -> Result<String> {
    let answer: i64 = file_input("data/2019/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line)
        .map(|mass| calculate_fuel(mass.clone()))
        .sum();

    Ok(answer.to_string())
}

pub fn part2() -> Result<String> {
    let answer: i64 = file_input("data/2019/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line)
        .map(|mass| calculate_fuel_recur(mass.clone()))
        .sum();

    Ok(answer.to_string())
}

fn calculate_fuel(mass: i64) -> i64 {
    /*
    Rules:
    Fuel required to launch a given module is based on its mass.
    Specifically, to find the fuel required for a module, take its mass,
    divide by three, round down, and subtract 2.

    For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    For a mass of 1969, the fuel required is 654.
    For a mass of 100756, the fuel required is 33583.
    */
    (mass as f64 / 3.0_f64).floor() as i64 - 2
}

fn calculate_fuel_recur(mass: i64) -> i64 {
    let fuel = calculate_fuel(mass);

    match fuel {
        f if f > 0 => f + calculate_fuel_recur(f),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_calculate_fuel_recur() {
        assert_eq!(calculate_fuel_recur(14), 2);
        assert_eq!(calculate_fuel_recur(1969), 966);
        assert_eq!(calculate_fuel_recur(100756), 50346);
    }
}
