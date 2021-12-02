use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let increase_count: u32 = reader
        .lines()
        .map(|l| l.unwrap().parse::<u32>().expect("parse_error"))
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();

    println!("Increase count: {}", increase_count);

    Ok(())
}
