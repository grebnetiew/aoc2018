extern crate regex;
use regex::Regex;
use std::cmp;
use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()

fn main() {
    let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(Result::ok);

    let (mut width, mut height) = (0, 0);
    let rectangles = lines
        .map(|s| {
            re.captures(&s)
                .unwrap()
                .iter()
                .skip(1) // the entire line is the first match
                .map(|s| s.unwrap().as_str().parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|entry| {
            width = cmp::max(width, entry[0] + entry[2]);
            height = cmp::max(height, entry[1] + entry[3]);
            entry
        })
        .collect::<Vec<_>>();

    let mut fabric: Vec<u8> = vec![0; width * height];
    for v in rectangles {
        for i in v[0]..(v[0] + v[2]) {
            for j in v[1]..(v[1] + v[3]) {
                fabric[width * j + i] = fabric[width * j + i].saturating_add(1);
            }
        }
    }
    println!("{:?}", fabric.into_iter().filter(|&n| n > 1).count());
}
