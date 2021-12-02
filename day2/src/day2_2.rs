use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut x: i32 = 0;
    let mut z: i32 = 0;
    let mut aim: i32 = 0;
    for line in reader.lines() {
        let parsed = line.unwrap();
        let v: Vec<&str> = parsed.split(" ").collect();
        let dir = v[0];
        let amt: i32 = v[1].parse().unwrap();
        match dir {
            "forward" => {
                x += amt;
                z += aim * amt;
            }
            "down" => {
                aim += amt;
            }
            "up" => {
                aim -= amt;
            }
            &_ => {
                continue;
            }
        }
    }

    println!("x {x} z {z} product {p}", x = x, z = z, p = x * z);

    Ok(())
}
