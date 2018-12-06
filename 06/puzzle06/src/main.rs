use std::cmp::max;
use std::io; // provides io's stdin()
use std::io::BufRead;
extern crate itertools;
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let points: Vec<(usize, usize)> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            s.split(", ")
                .map(|s| s.parse().unwrap())
                .next_tuple()
                .expect("Lines must contain two numbers")
        })
        .collect();

    let (width, height) = points.iter().fold((0, 0), |total, pt| {
        (max(total.0, pt.0 + 1), max(total.1, pt.1 + 1))
    });
    // We add 2, to ensure the maximum coordinate fits inside the grid

    let mut scores = vec![0; points.len()];
    let mut disqualify = vec![false; points.len()];

    let mut region_size = 0;

    for y in 0..height {
        for x in 0..width {
            // Part a: record the closest point for each coordinate in the area
            let min_dist = points
                .iter()
                .map(|p| manhattan(*p, (x, y)))
                .enumerate()
                .min_by_key(|(_, d)| *d)
                .unwrap();

            if points
                .iter()
                .filter(|p| manhattan(**p, (x, y)) == min_dist.1)
                .count()
                == 1
            // ignore any coordinates tied for closest point
            {
                if x == 0 || x == width || y == 0 || y == width {
                    disqualify[min_dist.0] = true; // infinite area
                } else {
                    scores[min_dist.0] += 1;
                }
            }

            // Part b: count coordinates with total distance under 10000
            if points
                .iter()
                .map(|p| manhattan(*p, (x, y)))
                .fold(0, |m, n| n.saturating_add(m))
                < 10000
            {
                region_size += 1;
            }
        }
    }

    let winner = scores
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !disqualify[*i])
        .max_by_key(|(_, s)| *s);
    println!("{:?}", winner);

    println!("{:?}", region_size);
}

fn manhattan(p: (usize, usize), q: (usize, usize)) -> usize {
    let mut d = 0;
    if p.0 > q.0 {
        d += p.0 - q.0;
    } else {
        d += q.0 - p.0
    }
    if p.1 > q.1 {
        d += p.1 - q.1;
    } else {
        d += q.1 - p.1
    }
    d
}
