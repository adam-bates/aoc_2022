use clap::*;

/// Advent of Code 2022 solutions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Which day to solve
    #[arg(short, long)]
    pub day: usize,

    /// Which day's part to solve
    #[arg(short, long, default_value_t = 1)]
    pub part: u8, 
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();

    if args.day == 0 {
        args.day = 1;
    }

    if args.part == 0 {
        args.part = 1;
    } else if args.part > 2 {
        args.part = 2;
    }

    return args;
}

