use anyhow::Error;
use clap::Parser;
use rust_2023::{Args, get_solver_fn};
use std::fs;

use runfiles::Runfiles;

fn main() -> anyhow::Result<(), Error> {
    let r = Runfiles::create().unwrap();

    let args = Args::parse();
    let _day = args.day;
    let file_str = match args.sample {
        Some(n) => format!("sample{:?}.txt", n),
        _ => String::from("input.txt"),
    };

    let _solver_fn = get_solver_fn(args)?;

    eprintln!("day: {_day}, file_str: {file_str}");

    let path = runfiles::rlocation!(r, format!("aoc_files/2023/day{_day}/{file_str}",))
        .expect("couldn't open runfiles file");
    let input = fs::read_to_string(path).unwrap();

    let answer = (_solver_fn)(&input).unwrap();
    println!("{:?}", answer);
    Ok(())
}
