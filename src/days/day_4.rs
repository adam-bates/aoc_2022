use crate::{utils, result::*};

// https://adventofcode.com/2022/day/4

const INPUT_FILE: &'static str = "input_4.txt";

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut fully_contained = 0;

    for line in input.lines() {
        let (range_1, range_2) = line.split_once(",").unwrap();

        let (range_1_min, range_1_max) = range_1.split_once("-").unwrap();
        let (range_2_min, range_2_max) = range_2.split_once("-").unwrap();

        let (range_1_min, range_1_max) = (
            range_1_min.parse::<usize>().unwrap(),
            range_1_max.parse::<usize>().unwrap(),
        );

        let (range_2_min, range_2_max) = (
            range_2_min.parse::<usize>().unwrap(),
            range_2_max.parse::<usize>().unwrap(),
        );

        if (
            range_1_min <= range_2_min && range_1_max >= range_2_max
        ) || (
            range_2_min <= range_1_min && range_2_max >= range_1_max
        ) {
            fully_contained += 1;
        }
    }

    return Ok(fully_contained.to_string());
}

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut overlapping = 0;

    for line in input.lines() {
        let (range_1, range_2) = line.split_once(",").unwrap();

        let (range_1_min, range_1_max) = range_1.split_once("-").unwrap();
        let (range_2_min, range_2_max) = range_2.split_once("-").unwrap();

        let (range_1_min, range_1_max) = (
            range_1_min.parse::<usize>().unwrap(),
            range_1_max.parse::<usize>().unwrap(),
        );

        let (range_2_min, range_2_max) = (
            range_2_min.parse::<usize>().unwrap(),
            range_2_max.parse::<usize>().unwrap(),
        );

        if (
            range_1_min <= range_2_min && range_2_min <= range_1_max
        ) || (
            range_1_min <= range_2_max && range_2_max <= range_1_max
        ) || (
            range_2_min <= range_1_min && range_1_min <= range_2_max
        ) || (
            range_2_min <= range_1_max && range_1_max <= range_2_max
        ) {
            overlapping += 1;
        }
    }

    return Ok(overlapping.to_string());
}


