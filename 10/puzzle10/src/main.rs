use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

#[macro_use]
extern crate text_io;

fn main() {
    let re = Regex::new(r"-?\d+").unwrap();
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let mut data: Vec<Vec<i32>> = f
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            re.captures_iter(&s)
                .map(|m| m.get(0).unwrap().as_str().parse().unwrap())
                .collect()
        })
        .collect();

    let mut time = 0;
    for _ in 0..1000 {
        display(&data);
        print!(
            "Time is {:}. Advance how many seconds forward or backward? ",
            time
        );
        let _ = io::stdout().flush();
        let i: i32;
        scan!("{}", i);
        time += i;
        advance(&mut data, i);
    }
}

fn advance(point_data: &mut Vec<Vec<i32>>, factor: i32) {
    for v in point_data.iter_mut() {
        v[0] += factor * v[2];
        v[1] += factor * v[3];
    }
}

const SCREENW: usize = 62;
const SCREENH: usize = 10;

fn display(point_data: &Vec<Vec<i32>>) {
    let mut screen = vec![' '; SCREENW * SCREENH];
    let bounds = point_data.iter().fold(
        (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
        |(a, b, c, d), v| (min(a, v[0]), min(b, v[1]), max(c, v[0]), max(d, v[1])),
    );
    println!(
        "Display is LT {:?}, RB {:?}, that is {:} x {:}",
        (bounds.0, bounds.1),
        (bounds.2, bounds.3),
        bounds.2 - bounds.0 + 1,
        bounds.3 - bounds.1 + 1
    );

    for v in point_data.iter() {
        let x = rescale(v[0], bounds.0, bounds.2, 0, SCREENW as i32 - 1);
        let y = rescale(v[1], bounds.1, bounds.3, 0, SCREENH as i32 - 1);
        screen[(y as usize * SCREENW + x as usize)] = '*';
    }

    for y in 0..SCREENH {
        for x in 0..SCREENW {
            print!("{:}", screen[y * SCREENW + x]);
        }
        println!("");
    }
}

fn rescale(x: i32, froma: i32, fromb: i32, toa: i32, tob: i32) -> i32 {
    (x - froma) * (tob - toa) / (fromb - froma) + toa
}
