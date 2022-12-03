#![allow(dead_code)]
#![allow(unused_imports)]

mod cli;
mod utils;

pub mod result;
use result::*;

mod days;
use days::*;

type SolutionPart<'a> = &'a dyn Fn() -> Result<String>;
type SolutionDay<'a> = (SolutionPart<'a>, SolutionPart<'a>);

fn main() -> Result {
    let solutions: Vec<SolutionDay> = vec![
        (&day_1::part_1, &day_1::part_2), // DAY 1
        (&day_2::part_1, &day_2::part_2), // DAY 2
        (&day_3::part_1, &day_3::part_2), // DAY 3
        // (&day_4::part_1, &day_4::part_2), // DAY 4
        // (&day_5::part_1, &day_5::part_2), // DAY 5
        // (&day_6::part_1, &day_6::part_2), // DAY 6
        // (&day_7::part_1, &day_7::part_2), // DAY 7
        // (&day_8::part_1, &day_8::part_2), // DAY 8
        // (&day_9::part_1, &day_9::part_2), // DAY 9
        // (&day_10::part_1, &day_10::part_2), // DAY 10
        // (&day_11::part_1, &day_11::part_2), // DAY 11
        // (&day_12::part_1, &day_12::part_2), // DAY 12
        // (&day_13::part_1, &day_13::part_2), // DAY 13
        // (&day_14::part_1, &day_14::part_2), // DAY 14
        // (&day_15::part_1, &day_15::part_2), // DAY 15
        // (&day_16::part_1, &day_16::part_2), // DAY 16
        // (&day_17::part_1, &day_17::part_2), // DAY 17
        // (&day_18::part_1, &day_18::part_2), // DAY 18
        // (&day_19::part_1, &day_19::part_2), // DAY 19
        // (&day_20::part_1, &day_20::part_2), // DAY 20
        // (&day_21::part_1, &day_21::part_2), // DAY 21
        // (&day_22::part_1, &day_22::part_2), // DAY 22
        // (&day_23::part_1, &day_23::part_2), // DAY 23
        // (&day_24::part_1, &day_24::part_2), // DAY 24
        // (&day_25::part_1, &day_25::part_2), // DAY 25
    ];

    let mut args = cli::parse_args();

    if args.day == -1 {
        args.day = solutions.len() as i8;
    }
    let day_parts = solutions[args.day as usize - 1];

    if args.part == -1 {
        display_part(args.day, 1, &day_parts)?;
        println!();
        display_part(args.day, 2, &day_parts)?;
    } else {
        display_part(args.day, args.part, &day_parts)?;
    }

    return Ok(());
}

fn display_part(day: i8, part: i8, day_parts: &SolutionDay) -> Result {
    println!("Day {day}, Part {part}");

    let solution = if part == 1 {
        day_parts.0
    } else {
        day_parts.1
    };

    println!("{}", solution()?);

    return Ok(());
}
