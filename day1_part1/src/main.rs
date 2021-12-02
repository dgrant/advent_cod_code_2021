use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn lines_from_file_as_int(filename: impl AsRef<Path>) -> impl Iterator<Item=u32> {
    BufReader::new(File::open(filename).expect("no such file")).lines()
        .map(|l| l.expect("Could not parse line").parse().unwrap())
}

fn main() {
    let mut values = lines_from_file_as_int(
        "/home/dgrant/git/advent_of_code_2021/day1_part1/data.txt",
    );

    let mut prev = values.next().expect("Oops, you have no values");
    let mut count = 0;
    for value in values {
        if value > prev {
            count += 1;
        }
        prev = value;
    }
    println!("{:?}", count);
}
