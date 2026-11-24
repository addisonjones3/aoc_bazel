use clap::Parser;
use hack::{Args, run};
use rust_2019::SOLVERS;

fn main() -> anyhow::Result<()> {
    let answer = run(Args::parse(), 2019, SOLVERS)?;
    println!("{:?}", answer);
    Ok(())
}
