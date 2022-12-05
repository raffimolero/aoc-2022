use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn input_from_day(day: u32) -> impl Iterator<Item = String> {
    BufReader::new(File::open(format!("assets/day{day}.txt")).expect("That file didn't exist."))
        .lines()
        .map(|l| l.unwrap())
}
