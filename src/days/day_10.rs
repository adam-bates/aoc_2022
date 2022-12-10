use crate::{result::*, utils};

// https://adventofcode.com/2022/day/9

const INPUT_FILE: &'static str = "input_9.txt";

enum Instruction {
    AddX(isize),
    Noop,
}

impl Instruction {
    pub fn cycles(&self) -> isize {
        match self {
            Instruction::Noop => return 1,
            Instruction::AddX(_) => return 2,
        }
    }
}

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut instructions = Vec::new();

    for line in input.lines() {
        let instruction = if line == "noop" {
            Instruction::Noop
        } else {
            let value = line[5..].parse::<isize>().unwrap();

            Instruction::AddX(value)
        };

        instructions.push(instruction);
    }

    let mut register = 1;
    let mut cycle = 1;

    let mut signal_strength_sum = 0;

    for instruction in instructions {
        let interest_dist = (cycle + 21) % 40;

        if interest_dist == 1 {
            signal_strength_sum += cycle * register;
        }

        match instruction {
            Instruction::Noop => {
                cycle += 1;
            }
            Instruction::AddX(x) => {
                if interest_dist == 0 {
                    signal_strength_sum += (cycle + 1) * register;
                }

                cycle += 2;
                register += x;
            }
        }
    }

    return Ok(signal_strength_sum.to_string());
}

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut instructions = Vec::new();

    for line in input.lines() {
        let instruction = if line == "noop" {
            Instruction::Noop
        } else {
            let value = line[5..].parse::<isize>().unwrap();

            Instruction::AddX(value)
        };

        instructions.push(instruction);
    }

    let mut crt: [[char; CRT_WIDTH]; CRT_HEIGHT] =
        std::array::from_fn(|_| std::array::from_fn(|_| '-'));

    let mut register: isize = 1;

    let mut row = 0;
    let mut col = 0;

    for instruction in instructions {
        let cycles;
        let register_diff;

        match instruction {
            Instruction::Noop => {
                cycles = 1;
                register_diff = 0;
            }
            Instruction::AddX(x) => {
                cycles = 2;
                register_diff = x;
            }
        }

        for _ in 0..cycles {
            let pixel = if register - 1 <= col && col <= register + 1 { '#' } else { '.' };

            crt[row as usize][col as usize] = pixel;

            col += 1;
            if col >= CRT_WIDTH as isize {
                col = 0;
                row += 1;
            }
        }

        register += register_diff;
    }

    let mut output = String::new();

    for row in crt {
        for pixel in row {
            output.push(pixel);
        }

        output.push('\n');
    }

    return Ok(output);
}
