extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::io;
use std::io::BufRead;
use std::io::Error;

fn main() {
    let stdin = io::stdin();
    let less_than: Vec<(char, char)> = stdin.lock().lines().map(|l| parse_line(l)).collect();

    let mut chars: Vec<char> = Vec::new();
    for (a, b) in less_than.iter() {
        chars.push(*a);
        chars.push(*b);
    }
    chars.sort();
    chars.dedup();
    chars.sort_by(|a, b| cmp(*a, *b, &less_than));
    println!("{:?}", cmp('C', 'A', &less_than));

    println!("{:?}", chars);
}

fn parse_line(l: Result<String, Error>) -> (char, char) {
    let re = Regex::new(r" ([A-Z]) ").unwrap();

    re.captures_iter(&l.unwrap())
        .take(2)
        .map(|s| s.get(1).unwrap().as_str().chars().next().unwrap())
        .next_tuple()
        .unwrap()
}

fn cmp(a: char, b: char, ordering: &Vec<(char, char)>) -> std::cmp::Ordering {
    match cmp_less(a, b, ordering) {
        Ordering::Equal => cmp_less(b, a, ordering),
        t => t,
    }
}

fn cmp_less(a: char, b: char, ordering: &Vec<(char, char)>) -> std::cmp::Ordering {
    for (x, y) in ordering.iter() {
        if a == *x {
            if b == *y {
                return Ordering::Less;
            }
            if cmp(*y, b, ordering) == Ordering::Less {
                return Ordering::Less;
            }
        }
    }
    Ordering::Equal
}
