use crate::file_ops;
use std::collections::BTreeMap;

type BTM = BTreeMap<u32, Vec<u32>>;

struct BitSet {
    bits: u128,
}

impl BitSet {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn get(&self, idx: u32) -> bool {
        self.bits & (1 << idx) != 0
    }

    // returns previous bool
    fn insert(&mut self, idx: u32) -> bool {
        let ret = self.get(idx);
        self.bits |= 1 << idx;
        ret
    }

    // returns previous bool
    fn _remove(&mut self, idx: u32) -> bool {
        let ret = self.get(idx);
        self.bits &= !(1 << idx);
        ret
    }
}

fn parse_data(data: &str) -> (BTM, Vec<Vec<u32>>) {
    let (rules, pages) = data
        .split_once("\n\n")
        .expect("file should be in two blocks separated by \"\\n\\n\" (a blank line)");
    (
        rules
            .lines()
            .map(|l| {
                let (a, b) = l.split_once('|').unwrap();
                (
                    a.parse().unwrap(),
                    b.parse().expect("pages should be in the form \"a|b\""),
                )
            })
            .fold(BTreeMap::new(), |mut acc, (l, r)| {
                acc.entry(l).or_insert(Vec::new()).push(r);
                acc
            }),
        pages
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|n| n.parse().expect("pages should be in the form \"a,b,c\""))
                    .collect()
            })
            .collect(),
    )
}

pub fn solve() {
    let (ord_rules, pages) = parse_data(&file_ops::read_day(5));
    println!("part a: {}", part_a(&ord_rules, &pages));
    println!("part b: {}", part_b(&ord_rules, &pages));
}

fn part_a(rules: &BTM, pages_list: &Vec<Vec<u32>>) -> u32 {
    pages_list
        .iter()
        .filter(|&pages| valid_topo_sort(rules, pages))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn valid_topo_sort(rules: &BTM, pages: &Vec<u32>) -> bool {
    let mut seen = BitSet::new();
    pages.iter().all(|page| {
        let _ = seen.insert(*page);
        rules
            .get(page)
            .map(|befores| !befores.iter().any(|&b| seen.get(b)))
            .unwrap_or(true)
    })
}

use std::cmp::Ordering;
fn part_b(rules: &BTM, pages_list: &Vec<Vec<u32>>) -> u32 {
    pages_list
        .into_iter()
        .filter(|&pages| !valid_topo_sort(rules, pages))
        .map(|pages| {
            let mut pages = pages.clone();
            pages.sort_unstable_by(|a, b| {
                if a == b {
                    Ordering::Equal
                } else if rules.get(b).map(|l| l.contains(a)).unwrap_or(true) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            pages[pages.len() / 2]
        })
        .sum()
}
