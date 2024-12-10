pub mod days;
pub(crate) mod file_ops;
use days::{day03, day04};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert!(args.len() == 2);
    let day: i32 = args[1].parse().unwrap();
    match day {
        0..=2 => unimplemented!(),
        3 => day03::solve(),
        4 => day04::solve(),
        5..=25 => unimplemented!(),
        _ => panic!("not an advent day"),
    }
}
