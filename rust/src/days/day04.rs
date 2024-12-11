use crate::file_ops;

pub fn solve() {
    let data_str = file_ops::read_day(4);
    let data: Vec<_> = data_str.lines().map(|line| line.as_bytes()).collect();
    println!("part a: {}", part_a(&data));
    println!("part b: {}", part_b(&data));
}

fn part_a(data: &[&[u8]]) -> usize {
    // compiler *should* precompute this
    let directions = (-1_i32..=1)
        .flat_map(|i| (-1_i32..=1).map(move |j| (i, j)).filter(|&t| t != (0, 0)))
        .collect::<Vec<_>>();

    let data_get = |i, j| {
        data.get(i as usize)
            .and_then(|line| line.get(j as usize))
            .copied()
    };

    (0..(data.len() as i32))
        // get valid pairs (i, j) of positions
        .flat_map(|i| (0..(data[0].len() as i32)).map(move |j| (i, j)))
        // pair (i, j) with possible directions you can move
        .flat_map(|(i, j)| directions.iter().map(move |(d1, d2)| (i, j, d1, d2)))
        // check from position (i, j) moving in directioon (d1, d2) writes "XMAS"
        .filter(|(i, j, &d1, &d2)| {
            (0..4)
                .filter_map(|n| data_get(n * d1 + i, n * d2 + j))
                .eq("XMAS".bytes())
        })
        .count()
}

fn part_b(data: &[&[u8]]) -> usize {
    let directions = [-1, 1]
        .iter()
        .flat_map(|i| [-1, 1].iter().map(move |j| (i, j)))
        .collect::<Vec<_>>();

    let data_get = |i, j| {
        data.get(i as usize)
            .and_then(|line| line.get(j as usize))
            .copied()
    };

    (0..(data.len() as i32))
        .flat_map(|i| (0..(data[0].len() as i32)).map(move |j| (i, j)))
        .flat_map(|(i, j)| directions.iter().map(move |(d1, d2)| (i, j, d1, d2)))
        .filter(|(i, j, &d1, &d2)| {
            (-1..=1)
                .filter_map(|k| data_get(i + k * d1, j + k * d1))
                .eq("MAS".bytes())
                && (-1..=1)
                    .filter_map(|k| data_get(i + k * d2, j - k * d2))
                    .eq("MAS".bytes())
        })
        .count()
}
