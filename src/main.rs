#![allow(dead_code)]
#![allow(unused_imports)]

mod utils;
mod cli;

pub mod result;
use result::*;

mod days;
use days::*;

type SolutionPart<'a> = &'a dyn Fn() -> Result<String>;
type SolutionDay<'a> = (SolutionPart<'a>, SolutionPart<'a>); 

fn main() -> Result {
    let solutions: Vec<SolutionDay> = vec![
        (&day_1::part_1, &day_1::part_2),
    ];

    let args = cli::parse_args();

    println!("Day {}, Part {}", args.day, args.part);

    let day_parts = solutions[args.day - 1];
    let solution = if args.part == 1 { day_parts.0 } else { day_parts.1 };

    println!("{}", solution()?);

    return Ok(());
}

