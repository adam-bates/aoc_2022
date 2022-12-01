use crate::{utils, result::*};

// https://adventofcode.com/2022/day/1

const INPUT_FILE: &'static str = "input_1.txt";

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut max = 0;
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current > max {
                max = current;
            }

            current = 0;
        } else {
            let calories: i32 = line.parse::<i32>()?;

            current += calories;
        }
    }

    return Ok(max.to_string());
}

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut max = [0, 0, 0];
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current > max[2] {
                max[0] = max[1];
                max[1] = max[2];
                max[2] = current;
            } else if current > max[1] {
                max[0] = max[1];
                max[1] = current;
            } else if current > max[0] {
                max[0] = current;
            }

            current = 0;
        } else {
            let calories: i32 = line.parse::<i32>()?;

            current += calories;
        }
    }

    let max: i32 = max.iter().sum();

    return Ok(max.to_string());
}

