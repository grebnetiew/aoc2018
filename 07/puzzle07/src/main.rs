extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::io;
use std::io::BufRead;
use std::io::Error;

fn main() {
    let stdin = io::stdin();
    let mut less_than: Vec<(char, char)> = stdin.lock().lines().map(|l| parse_line(l)).collect();
    less_than.sort_by_key(|(_, k)| *k);

    let mut all: Vec<char> = Vec::new();
    for (a, b) in less_than.iter() {
        all.push(*a);
        all.push(*b);
    }
    all.sort();
    all.dedup();
    let mut done = Vec::new();
    println!("a{:?}", all);

    while let Some(do_more) = available(&all, &done, &less_than) {
        println!("d {:?}", done.iter().collect::<String>());
        println!("n {:?}", do_more);
        done.push(do_more);
    }

    println!("{:?}", done.iter().collect::<String>());
}

fn parse_line(l: Result<String, Error>) -> (char, char) {
    let re = Regex::new(r" ([A-Z]) ").unwrap();

    re.captures_iter(&l.unwrap())
        .take(2)
        .map(|s| s.get(1).unwrap().as_str().chars().next().unwrap())
        .next_tuple()
        .unwrap()
}

fn available(all: &Vec<char>, done: &Vec<char>, less_than: &Vec<(char, char)>) -> Option<char> {
    let mut not_yet: Vec<char> = Vec::new();
    for (before, after) in less_than.iter() {
        if !done.contains(before) {
            not_yet.push(*after);
        }
    }
    all.iter()
        .filter(|c| !done.contains(c) && !not_yet.contains(c))
        .map(|c| *c)
        .min()
}
