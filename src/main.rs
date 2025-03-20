use std::error::Error;

mod args;
mod color;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args();
    println!("{:?}", args);

    Ok(())
}
