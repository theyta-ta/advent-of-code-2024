use crate::file_ops;

pub fn solve() {
    let data_str = file_ops::read_day(4);
    let data: Vec<_> = data_str.lines().map(|line| line.as_bytes()).collect();
    println!("part a: {}", part_a(&data));
    println!("part b: {}", part_b(&data));
}

fn indexes<'a>(data: &'a Vec<&'a [u8]>) -> impl Iterator<Item = (i32, i32)> + 'a {
    (0..(data.len() as i32))
        // get valid pairs (i, j) of positions
        .flat_map(|i| (0..(data[0].len() as i32)).map(move |j| (i, j)))
}

fn get(data: &Vec<&[u8]>, i: i32, j: i32) -> Option<u8> {
    data.get(i as usize)
        .and_then(|l| l.get(j as usize))
        .copied()
}

fn part_a(data: &Vec<&[u8]>) -> usize {
    // compiler *should* precompute this
    let directions = (-1_i32..=1)
        .flat_map(|i| (-1_i32..=1).map(move |j| (i, j)).filter(|&t| t != (0, 0)))
        .collect::<Vec<_>>();

    indexes(data)
        // pair (i, j) with possible directions you can move
        .flat_map(|(i, j)| directions.iter().map(move |(d1, d2)| (i, j, d1, d2)))
        // check from position (i, j) moving in directioon (d1, d2) writes "XMAS"
        .filter(|(i, j, &d1, &d2)| {
            (0..4)
                .filter_map(|n| get(data, n * d1 + i, n * d2 + j))
                .collect::<Vec<u8>>()
                == "XMAS".as_bytes()
        })
        .count()
}

fn part_b(data: &Vec<&[u8]>) -> usize {
    let directions = [-1, 1]
        .into_iter()
        .flat_map(|i| [-1, 1].into_iter().map(move |j| (i, j)))
        .collect::<Vec<_>>();

    indexes(data)
        // pair (i, j) with possible directions you can move
        .flat_map(|(i, j)| directions.iter().map(move |(d1, d2)| (i, j, d1, d2)))
        // check from position (i, j) moving in directioon (d1, d2) writes "XMAS"
        .filter(|(i, j, &d1, &d2)| {
            // im sure theres a better way of doing this.
            // but it would likely involve straying too far from the left margin
            (-1..=1)
                .filter_map(|k| get(data, i + k * d1, j + k * d1))
                .collect::<Vec<_>>()
                == "MAS".as_bytes()
                && (-1..=1)
                    .filter_map(|k| get(data, i + k * d2, j - k * d2))
                    .collect::<Vec<_>>()
                    == "MAS".as_bytes()
        })
        .count()
}
