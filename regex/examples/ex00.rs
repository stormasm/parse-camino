use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("/j/tmp99/parse-camino-data/t3.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}
