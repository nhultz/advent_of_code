use crate::file_input;
use crate::Result;

use anyhow::anyhow;

pub fn part1() -> Result<String> {
    let mut program = intcode_program()?;
    run_program(&mut program, 12, 2);
    Ok(program[0].to_string())
}

pub fn part2() -> Result<String> {
    let orig_program = intcode_program()?;
    let expected_output = 19690720;

    for i in 0..=99 {
        for j in 0..=99 {
            let mut program = orig_program.clone();
            run_program(&mut program, i, j);

            if program[0] == expected_output {
                return Ok(format!("{}", 100 * i + j));
            }
        }
    }

    Err(anyhow!("No answer found"))
}

fn intcode_program() -> Result<Vec<usize>> {
    let input: Vec<usize> = file_input("data/2019/day2_input.txt")?
        .flat_map(|line| {
            line.split(',')
                .map(|val| val.parse::<usize>())
                .flat_map(|val| val)
                .collect::<Vec<usize>>()
        })
        .collect();

    Ok(input)
}

fn run_program(intcode_program: &mut [usize], input1: usize, input2: usize) {
    intcode_program[1] = input1;
    intcode_program[2] = input2;

    let mut idx = 0;

    loop {
        match intcode_program[idx] {
            1 => {
                let val1 = intcode_program[idx + 1];
                let val2 = intcode_program[idx + 2];
                let dest = intcode_program[idx + 3];
                intcode_program[dest] = intcode_program[val1] + intcode_program[val2];
            }
            2 => {
                let val1 = intcode_program[idx + 1];
                let val2 = intcode_program[idx + 2];
                let dest = intcode_program[idx + 3];
                intcode_program[dest] = intcode_program[val1] * intcode_program[val2];
            }
            99 => break,
            _ => break,
        };

        idx += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let mut v = vec![1, 0, 0, 4, 99, 5, 6, 0, 99];
        run_program(&mut v, 1, 1);
        assert_eq!(v[0], 30);
    }
}
