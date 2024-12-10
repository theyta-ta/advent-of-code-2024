use std::fs;

pub fn read_day(day: u32) -> String {
    let file_name = format!("data/data{day:0>2}.txt");
    // if we dont have the file, not much we can do but `panic` and crash and burn
    fs::read_to_string(file_name).expect("file should be valid utf-8")
}
