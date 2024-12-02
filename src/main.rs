#![allow(dead_code)]

use std::error::Error;

mod one;
mod two;

fn main() -> Result<(), Box<dyn Error>> {
    two::solve()?;
    Ok(())
}
