use clap::Parser;
use hack::{Args, run};
use rust_{{project-name}}::SOLVERS;

fn main() -> anyhow::Result<()> {
    let answer = run(Args::parse(), SOLVERS)?;
    println!("{:?}", answer);
    Ok(())
}
