use std::error::Error;

mod one;

fn main() -> Result<(), Box<dyn Error>> {
    one::solve()?;
    Ok(())
}
