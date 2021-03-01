use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use hangouts_rs::raw;
use hangouts_rs::Hangouts;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    println!("Reading...");
    let parsed: raw::Hangouts = serde_json::from_reader(reader)?;
    println!("Finished reading");

    println!("Converting...");
    let mut hangouts: Hangouts = parsed.try_into()?;

    println!("Sorting...");
    hangouts
        .conversations
        .iter_mut()
        .for_each(|conv| conv.events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)));

    let last = hangouts.conversations.last().expect("No conversations");

    for message in last
        .events
        .iter()
        .filter_map(|event| event.data.as_chat_message())
    {
        println!("{}", message.contents_as_str());
    }

    Ok(())
}
