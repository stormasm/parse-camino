use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("./../../parse-camino-data/t1.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let x = line.unwrap();
        let b = regex_month_and_year(&x.clone());
        if b {
            println!("date: {:?}", x);
        } else {
            //println!("entry: {:?}", x);
        }
    }
    Ok(())
}

/*
This matches on a day name followed by a space followed by a two digit number
Key point being is that if there is a day name somewhere else in the text it
will not match on that particular day name unless by happenstance it has
a space followed by a two digit number after it and if that case ever happens
I can change that two digit number and write it out instead.
So we should never get a match inside the text entry.
*/

fn regex_month_and_year(s1: &str) -> bool {
    let re =
        Regex::new(r"(January|February|March|April|May|June|July|August|September|October|November|December)\s\d{4}").unwrap();
    re.is_match(s1)
}
