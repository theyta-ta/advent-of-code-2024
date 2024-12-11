use regex::Regex;

use std::fs;
use std::io::{BufReader, Read /* for `bytes` */};

// use this method instead of file_ops to get rid of making everything oneline in part b
fn read_day() -> String {
    String::from_utf8(
        BufReader::new(fs::File::open("data/data03.txt").unwrap())
            .bytes()
            .map(|x| x.unwrap())
            .filter(|&c| c != b'\n')
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

pub fn solve() {
    let data = read_day();
    println!("Part a: {}", part_a(&data));
    println!("Part b: {}", part_b(&data));
}

fn part_a(data: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(data)
        .map(|cap| {
            let (_, [a, b]) = cap.extract();
            a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
        })
        .sum()
}

fn part_b(data: &str) -> u32 {
    let re = Regex::new(r"(?:^|do\(\))(.*?)(?:don't\(\).*?|$)").unwrap();
    re.captures_iter(data)
        .map(|c| {
            // dont fucking do `part_a(c.extract::<1>().1[0])`.
            // dont you fucking dare.
            let (_, [ret]) = c.extract();
            part_a(ret)
        })
        .sum()
}
