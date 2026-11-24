use clap::Parser;
use hack::{Args, run};
use rust_2023::SOLVERS;

fn main() -> anyhow::Result<()> {
    let answer = run(Args::parse(), SOLVERS)?;
    println!("{:?}", answer);
    Ok(())
}
