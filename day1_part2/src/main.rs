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
        "/home/dgrant/git/advent_of_code_2021/day1_part2/data.txt",
    ));

    let window_size = 3;
    let mut prev = values[..window_size].iter().sum();
    let mut count = 0;
    let mut i = window_size;
    let mut curr: u32;
    while i < values.len() {
        // Incremental moving average, add the next one, remove the earliest one
        curr = prev + values[i] - values[i - window_size];
        if curr > prev {
            count += 1;
        }
        prev = curr;
        i += 1;
    }
    println!("{:?}", count);
}
