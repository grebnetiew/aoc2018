extern crate itertools;

use itertools::Itertools;
use std::cmp::max;
use std::io; // provides io's stdin()
use std::io::BufRead;

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
            let distances = points
                .iter()
                .map(|p| manhattan(*p, (x, y)))
                .enumerate()
                .collect::<Vec<_>>();
            // Part a: record the closest point for each coordinate in the area

            let min_dist = distances.iter().min_by_key(|(_, d)| *d).unwrap();

            if distances.iter().filter(|d| d.1 == min_dist.1).count() == 1
            // ignore any coordinates tied for closest point
            {
                if x == 0 || x == width || y == 0 || y == width {
                    disqualify[min_dist.0] = true; // infinite area
                } else {
                    scores[min_dist.0] += 1;
                }
            }

            // Part b: count coordinates with total distance under 10000
            if distances.iter().map(|t| t.1).sum::<usize>() < 10000 {
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
    // I chose unsigned integers for the coordinates, so now I have to deal with
    // overflow on subtraction. This tries p - q and if that overflowed, gives q - p.
    abs_sub(p.0, q.0) + abs_sub(p.1, q.1)
}

fn abs_sub(x: usize, y: usize) -> usize {
    x.checked_sub(y).unwrap_or(y.wrapping_sub(x))
}
