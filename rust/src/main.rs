pub mod days;
pub(crate) mod file_ops;
use days::{day03, day04, day05 /*day06*/, day11, day13};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert!(args.len() == 2);
    let day: i32 = args[1].parse().unwrap();
    match day {
        0..=2 => unimplemented!(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        //6 => day06::solve(),
        7..=10 | 12 => unimplemented!(),
        11 => day11::solve(),
        13 => day13::solve(),
        14..=15 => unimplemented!(),
        _ => panic!("not an advent day"),
    }
}
