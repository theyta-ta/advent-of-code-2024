use crate::file_ops;
use std::collections::BTreeMap;

pub fn solve() {
    let data: Vec<_> = file_ops::read_day::<11>()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    println!("part a: {}", part_a(&data));
    // takes about 30ms on my machine :D
    println!("part b: {}", part_b(&data));
}

fn soln(n: usize, data: &[u64]) -> usize {
    fn recur(n: usize, cache: &mut BTreeMap<(usize, u64), usize>, val: u64) -> usize {
        if let Some(&v) = cache.get(&(n, val)) {
            return v;
        }

        let ret = if n == 0 {
            1
        } else if val == 0 {
            recur(n - 1, cache, 1)
        } else {
            let digits = val.ilog10() + 1;
            if digits % 2 == 0 {
                let temp = 10u64.pow(digits / 2);
                recur(n - 1, cache, val / temp) + recur(n - 1, cache, val % temp)
            } else {
                recur(n - 1, cache, val * 2024)
            }
        };

        cache.insert((n, val), ret);
        ret
    }
    let mut cache = BTreeMap::new();
    data.iter().map(|&v| recur(n, &mut cache, v)).sum()
}

fn part_a(data: &[u64]) -> usize {
    soln(25, &data)
}
fn part_b(data: &[u64]) -> usize {
    soln(75, &data)
}
