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

fn main() {
    let lines = lines_from_file("/home/dgrant/git/advent_of_code_2021/day2_part1/data.txt");
    let mut position = 0;
    let mut  depth = 0;
    for line in lines.iter() {
        let commands: Vec<&str> = line.split(" ").collect();
        let command = commands[0];
        let value: u32 = commands[1].parse().unwrap();
        if command == "forward" {
            position += value;
        } else if command == "up" {
            depth -= value;
        } else if command == "down" {
            depth += value;
        }
    }
    println!("depth: {:?}", depth);
    println!("position: {:?}", position);
    println!("depth*position: {:?}", depth*position);
}
