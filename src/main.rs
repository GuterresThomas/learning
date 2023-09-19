use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("Hello.txt")?;
    Ok(())
}
