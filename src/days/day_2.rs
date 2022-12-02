use crate::{utils, result::*};

// https://adventofcode.com/2022/day/2

const INPUT_FILE: &'static str = "input_2.txt";

enum Move {
    Rock,
    Paper,
    Scissors,
}

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut total_points = 0;

    for round_input in input.lines() {
        let (lhs, rhs) = round_input.split_once(" ").unwrap();

        let lhs = match lhs {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!(),
        };

        let (_rhs, points) = match rhs {
            "X" => (Move::Rock, 1 + match lhs {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissors => 6,
            }),
            "Y" => (Move::Paper, 2 + match lhs {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissors => 0,
            }),
            "Z" => (Move::Scissors, 3 + match lhs {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissors => 3,
            }),
            _ => panic!(),
        };

        total_points += points;
    }

    return Ok(total_points.to_string());
}

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut total_points = 0;

    for round_input in input.lines() {
        let (lhs, rhs) = round_input.split_once(" ").unwrap();

        let lhs = match lhs {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!(),
        };

        let points = match rhs {
            "X" => match lhs {
                Move::Rock => 3 + 0,
                Move::Paper => 1 + 0,
                Move::Scissors => 2 + 0,
            },
            "Y" =>  match lhs {
                Move::Rock => 1 + 3,
                Move::Paper => 2 + 3,
                Move::Scissors => 3 + 3,
            },
            "Z" => match lhs {
                Move::Rock => 2 + 6,
                Move::Paper => 3 + 6,
                Move::Scissors => 1 + 6,
            },
            _ => panic!(),
        };

        total_points += points;
    }

    return Ok(total_points.to_string());
}

