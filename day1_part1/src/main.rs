use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn lines_from_file_as_int(lines_str: Vec<String>) -> Vec<u32> {
    let mut lines_int: Vec<u32> = Vec::new();

    for line in lines_str {
        let line_as_int = line.parse().unwrap();
        lines_int.push(line_as_int);
    }
    lines_int
}

fn main() {
    let values = lines_from_file_as_int(lines_from_file(
        "/home/dgrant/git/advent_of_code_2021/day1_part1/data.txt",
    ));

    let mut prev = values[0];
    let mut count = 0;
    for value in values {
        if value > prev {
            count += 1;
        }
        prev = value;
    }
    println!("{:?}", count);
}
