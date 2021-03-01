use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use hangrs::raw;
use hangrs::Hangouts;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    println!("Reading...");
    let parsed: raw::Hangouts = serde_json::from_reader(reader)?;
    println!("Finished reading");

    println!("Converting...");
    let hangouts: Hangouts = parsed.try_into()?;

    println!("{:#?}", hangouts);

    Ok(())
}
