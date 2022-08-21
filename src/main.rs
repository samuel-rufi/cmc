use std::fs;
use crate::entry::FullEntry;

mod entry;

fn main() {
    let data = fs::read_to_string("input.json").expect("Unable to read file");
    let full: FullEntry = serde_json::from_str(&data).unwrap();


    // 1. Open CMC page
    // 2, Open Twitter
    // 3. Check followers account
    // 4. Check friends (logged in).
}