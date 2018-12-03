extern crate regex;
use regex::Regex;
use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()

fn main() {
    let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(Result::ok);

    let rectangles = lines
        .map(|s| {
            let caps = re.captures(&s).unwrap();
            caps.iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let mut intersecting_rects = vec![false; rectangles.len()];

    for i in 0..rectangles.len() {
        for j in 0..i {
            if intersect(&rectangles[i], &rectangles[j]) {
                intersecting_rects[i] = true;
                intersecting_rects[j] = true;
            }
        }
    }
    println!(
        "{:?}",
        intersecting_rects
            .iter()
            .enumerate()
            .filter(|&(_, &b)| !b)
            .next()
            .unwrap()
            .0
            + 1 // because line 0 has id 1
    );
}

fn intersect(r: &Vec<usize>, s: &Vec<usize>) -> bool {
    // If one rectangle is aside other
    if r[0] > s[0] + s[2] || s[0] > r[0] + r[2] {
        return false;
    }

    // If one rectangle is above other
    if r[1] > s[1] + s[3] || s[1] > r[1] + r[3] {
        return false;
    }

    true
}
