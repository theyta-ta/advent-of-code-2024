use crate::file_ops;
use regex::Regex;

pub fn solve() {
    let data_str = file_ops::read_day::<13>();
    let data: Vec<_> = data_str.split("\n\n").map(parse_block).collect();
    println!("part a: {}", part_a(&data));
    // takes <40us on my machine :D
    println!("part b: {}", part_b(&data));
}

type Trip = [[i128; 2]; 3];

fn part_a(cranes: &Vec<Trip>) -> u128 {
    cranes
        .into_iter()
        .filter_map(|[a, b, p]| num_tokens(*a, *b, *p))
        .filter(|(a, b)| *a >= 0 && *b >= 0)
        .map(|(a, b)| (3 * a + b) as u128)
        .sum()
}

fn part_b(cranes: &Vec<Trip>) -> u128 {
    cranes
        .into_iter()
        .filter_map(|[a, b, p]| num_tokens(*a, *b, p.map(|n| n + PART_B_DELTA)))
        .filter(|(a, b)| *a >= 0 && *b >= 0)
        .map(|(a, b)| (3 * a + b) as u128)
        .sum()
}

static PART_B_DELTA: i128 = 10000000000000;

/*
 * so we have the following pair of equations
 * a * A[x] + b * B[x] = P[x]
 * a * A[y] + b * B[y] = P[y]
 * where A, B, P are the movement/pos of buttons A, B and the prize P.
 *
 * now, rearranging the first eqn we get
 * a = (P[x] - b * B[x]) / A[x]
 *
 * plugging this into the second we get
 * A[y] * (P[x] - b * B[x]) / A[x] + b * B[y[ = P[y]
 *
 * again, rearranging gives us
 * b = (A[x] * P[y] - P[x] * A[y]) / (A[x] * B[y] - B[x] * A[y])
 * and
 * b = (P[x] * B[y] - P[y] * B[x]) / (A[x] * B[y] - B[x] * A[y])
 *
 * now, if you know any lin alg, you should recognise these as determinants
 * so if we have the following matrices
 * M = | A[x]  B[x] |    M_a = | P[x]  B[x] |    M_b = | A[x]  P[x] |
 *     | A[y]  B[y] |    M_a = | P[y]  B[y] |    M_b = | A[y]  P[y] |
 *
 * then we have
 * a = det(M_a) / det(M)
 * b = det(M_b) / det(M)
 *
 * because we want integer solutions,
 * we need det(M_a) and det(M_b) to be multiples of det(M).
 *
 * also, this function does not check if we press the button a negative amount.
 * it leads to cleaner code imo if we check that upsream.
 *
 * note: there are cases where there are solutions even if det(M) = 0,
 * however there are no matrices like this in the input data,
 * so we can (and will) ignore this edge case.
 */

// so we can follow the variables described in the above comment.
#[allow(non_snake_case)]
fn num_tokens(a: [i128; 2], b: [i128; 2], p: [i128; 2]) -> Option<(i128, i128)> {
    let det_M = a[0] * b[1] - a[1] * b[0];
    let det_Ma = p[0] * b[1] - p[1] * b[0];
    let det_Mb = a[0] * p[1] - a[1] * p[0];

    if det_M == 0 || det_Ma % det_M != 0 || det_Mb % det_M != 0 {
        None
    } else {
        Some((det_Ma / det_M, det_Mb / det_M))
    }
}

fn parse_block(lines: &str) -> Trip {
    let [a, b, prize]: &[&str] = &lines.lines().collect::<Vec<_>>()[..] else {
        panic!("malformed data that is not 3 lines");
    };
    let butt_re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let [a_dists, b_dists]: [_; 2] = [a, b].map(|s| {
        butt_re
            .captures(s)
            .expect("button line should be in the form of the regex.")
            .extract::<2>()
            .1
            .map(|n| n.parse().unwrap())
    });

    let prize_loc = prize_re
        .captures(prize)
        .expect("prize line should be the form of the regex")
        .extract::<2>()
        .1
        .map(|n| n.parse().unwrap());

    [a_dists, b_dists, prize_loc]
}
