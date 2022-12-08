use crate::{result::*, utils};

use std::collections::{HashMap, HashSet, VecDeque};

// https://adventofcode.com/2022/day/6

const INPUT_FILE: &'static str = "input_6.txt";

fn get_char_count_until_unique(input: &str, n_unique: usize) -> usize {
    let mut chars = input.chars();

    let mut queue = VecDeque::with_capacity(n_unique);
    let mut char_count = HashMap::with_capacity(n_unique);
    let mut duplicates = HashSet::new();

    let mut count = 1;

    for _ in 0..n_unique - 1 {
        match chars.next() {
            Some(c) => {
                queue.push_back(c);

                if let Some(count) = char_count.get_mut(&c) {
                    *count += 1;
                    duplicates.insert(c);
                } else {
                    char_count.insert(c, 1);
                }
            }
            None => return count,
        }

        count += 1;
    }

    while let Some(c) = chars.next() {
        queue.push_back(c);

        if let Some(count) = char_count.get_mut(&c) {
            *count += 1;

            if *count > 1 {
                duplicates.insert(c);
            }
        } else {
            char_count.insert(c, 1);
        }

        if duplicates.is_empty() {
            return count;
        }

        count += 1;

        let prev = queue.pop_front().unwrap();

        let prev_count = char_count.get_mut(&prev).unwrap();

        *prev_count -= 1;

        if *prev_count == 1 {
            duplicates.remove(&prev);
        }
    }

    return count;
}

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let count = get_char_count_until_unique(&input, 4);

    return Ok(count.to_string());
}

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let count = get_char_count_until_unique(&input, 14);

    return Ok(count.to_string());
}
