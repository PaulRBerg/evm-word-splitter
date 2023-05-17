use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let string1 = fs::read_to_string("src/inputs/input0.txt")?;
    let string2 = fs::read_to_string("src/inputs/input1.txt")?;

    if string1 == string2 {
        println!("Strings are equal");
    } else {
        println!("Strings are NOT equal");
    }

    Ok(())
}
