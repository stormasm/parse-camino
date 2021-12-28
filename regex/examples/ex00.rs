use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("/j/tmp99/parse-camino-data/t3.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let x = line.unwrap();
        let b = regex_t1(&x.clone());
        if b {
            println!("{:?}", x);
        }
    }
    Ok(())
}

fn regex_t1(s1: &str) -> bool {
    //    ^((?:mon|tue|wed|thu|fri|sat|sun)(?:, )?)(?1)*$

    //    let re = Regex::new(r"^(?:Sunday|Monday|Tuesday|Wednesday|Thursday|Friday|Saturday)*$").unwrap();

    let re = Regex::new(r"\W*(Sunday)\W*").unwrap();
    let val = re.is_match(s1);
    //println!("{:?}", val);
    return val;
}
