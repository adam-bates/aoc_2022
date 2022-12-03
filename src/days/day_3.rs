use std::collections::HashSet;

use crate::{utils, result::*};

// https://adventofcode.com/2022/day/3

const INPUT_FILE: &'static str = "input_3.txt";

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut priority_sum = 0;

    for line in input.lines() {
        let compartment_len = line.len() / 2;

        let seen: HashSet<char> = line[0..compartment_len].chars().collect();

        let mut dupe = None;

        for c in line[compartment_len..].chars() {
            if seen.contains(&c) {
                dupe = Some(c);
            }
        }

        if let Some(dupe) = dupe {
            const UPPER_OFFSET: usize = 'A' as usize - 1;
            const LOWER_OFFSET: usize = 'a' as usize - 1;

            priority_sum += if dupe.is_lowercase() {
                dupe as usize - LOWER_OFFSET
            } else {
                dupe as usize + 26 - UPPER_OFFSET
            };
        }
    }

    return Ok(priority_sum.to_string());
}

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut priority_sum = 0;

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let seen: HashSet<char> = line.chars().collect();

        let mut dupes = HashSet::new();

        for c in lines.next().unwrap().chars() {
            if seen.contains(&c) {
                dupes.insert(c);
            }
        }

        let mut dupe = None;

        for c in lines.next().unwrap().chars() {
            if dupes.contains(&c) {
                dupe = Some(c);
            }
        }

        if let Some(dupe) = dupe {
            const UPPER_OFFSET: usize = 'A' as usize - 1;
            const LOWER_OFFSET: usize = 'a' as usize - 1;

            priority_sum += if dupe.is_lowercase() {
                dupe as usize - LOWER_OFFSET
            } else {
                dupe as usize + 26 - UPPER_OFFSET
            };
        }
    }

    return Ok(priority_sum.to_string());
}

