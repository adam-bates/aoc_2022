use crate::{result::*, utils};

// https://adventofcode.com/2022/day/5

const INPUT_FILE: &'static str = "input_5.txt";

type Crate = char;
type Stack = Vec<Crate>;

struct Instruction {
    pub n: usize,
    pub from: usize,
    pub to: usize,
}

fn parse_file(input: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let mut stacks = Vec::new();
    let mut stack_idx = 0;

    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        while stack_idx >= stacks.len() {
            stacks.push(Stack::new());
        }

        match c {
            ' ' => {
                let c = chars.next().unwrap();

                match c {
                    ' ' => {
                        // shorter stack
                        chars.next();
                    }
                    '1' => {
                        // stack numbers
                        break;
                    }
                    _ => panic!(),
                }
            }
            '[' => {
                let val: Crate = chars.next().unwrap();

                chars.next(); // ]

                stacks[stack_idx].insert(0, val);
            }
            _ => panic!(),
        }

        match chars.next() {
            Some(' ') => {
                stack_idx += 1;
            }
            Some('\n') => {
                stack_idx = 0;
            }
            _ => panic!(),
        }
    }

    // Line with stack numbers
    while let Some(c) = chars.next() {
        match c {
            '\n' => break,
            _ => continue,
        }
    }

    // Empty line
    while let Some(c) = chars.next() {
        match c {
            '\n' => break,
            _ => continue,
        }
    }

    let mut instructions = Vec::new();

    while let Some(_) = chars.next() {
        // 'm'

        // "ove "
        for _ in 0..4 {
            chars.next();
        }

        let n = {
            let mut buf = String::new();
            while let Some(c) = chars.next() {
                match c {
                    ' ' => break,
                    _ => buf.push(c),
                }
            }
            buf.parse::<usize>().unwrap()
        };

        // "from "
        for _ in 0..5 {
            chars.next();
        }

        let from = {
            let mut buf = String::new();
            while let Some(c) = chars.next() {
                match c {
                    ' ' => break,
                    _ => buf.push(c),
                }
            }
            buf.parse::<usize>().unwrap()
        };

        // "to "
        for _ in 0..3 {
            chars.next();
        }

        let to = {
            let mut buf = String::new();
            while let Some(c) = chars.next() {
                match c {
                    '\n' => break,
                    _ => buf.push(c),
                }
            }
            buf.parse::<usize>().unwrap()
        };

        instructions.push(Instruction { n, from, to });
    }

    return (stacks, instructions);
}

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let (mut stacks, instructions) = parse_file(&input);

    for instruction in instructions {
        for _ in 0..instruction.n {
            let moving = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(moving);
        }
    }

    let mut out = String::new();

    for stack in &mut stacks {
        out.push(stack.pop().unwrap());
    }

    return Ok(out);
}

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let (mut stacks, instructions) = parse_file(&input);

    for instruction in instructions {
        let mut moving = Vec::new();

        for _ in 0..instruction.n {
            moving.push(stacks[instruction.from - 1].pop().unwrap());
        }

        while let Some(moving) = moving.pop() {
            stacks[instruction.to - 1].push(moving);
        }
    }

    let mut out = String::new();

    for stack in &mut stacks {
        out.push(stack.pop().unwrap());
    }

    return Ok(out);
}
