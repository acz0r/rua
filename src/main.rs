use anyhow::Context;
use clap::Parser;
use std::{fs::File, path::PathBuf};
use thiserror::Error;

mod iife;
#[cfg(test)]
mod tests;
mod transpiler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    src: PathBuf,
    #[arg(short, long)]
    out: PathBuf,
}

fn main() -> Result<(), RuaError> {
    let args = Args::parse();

    let src_file = File::options()
        .read(true)
        .open(args.src)
        .context("Failed to open source file")?;

    // TODO: transpile

    let out_file = File::options()
        .write(true)
        .open(args.out)
        .context("Failed to open out file")?;

    // TODO: write transpiled content to out file

    Ok(())
}

#[derive(Error, Debug)]
enum RuaError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
