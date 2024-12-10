use crate::file_ops;
use regex::Regex;

pub fn solve() {
    let data = file_ops::read_day(3);
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
    re.captures_iter(&data.lines().collect::<Vec<_>>().join(""))
        .map(|c| c.extract::<1>().1[0])
        .map(part_a)
        .sum()
}
