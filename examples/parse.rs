use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use hangrs::raw::Hangouts;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    let parsed = Hangouts::from_reader(reader)?;

    println!("{:#?}", parsed);

    Ok(())
}
