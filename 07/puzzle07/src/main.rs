extern crate itertools;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::io;
use std::io::BufRead;
use std::io::Error;

fn main() {
    let stdin = io::stdin();
    let task_order: Vec<(char, char)> = stdin.lock().lines().map(|l| parse_line(l)).collect();

    let mut all: Vec<char> = CharRangeInclusive(
        'A',
        task_order.iter().fold('A', |m, (a, b)| max(m, max(*a, *b))),
    )
    .collect();

    // Part 1. Do the tasks in order
    let mut done = Vec::new();
    while let Some(do_more) = available(&all, &done, &task_order) {
        done.push(do_more);
    }

    println!("{:}", done.iter().collect::<String>());

    // Part 2. Do the tasks in parallel, measure time
    const WORKERS: usize = 5;
    const EXTRA_TIME: usize = 60;

    let mut workers = vec![(None, 0); WORKERS];
    let mut done = Vec::new();
    let mut time = 0;

    loop {
        for w in 0..WORKERS {
            if let Some(task) = workers[w].0 {
                workers[w].1 -= 1;
                if workers[w].1 == 0 {
                    done.push(task);
                    workers[w].0 = None;
                    workers[w].1 = 0;
                }
            }
        }
        let mut all_idle = true;
        for w in 0..WORKERS {
            if workers[w].0.is_none() {
                if let Some(next) = available(&all, &done, &task_order) {
                    workers[w].0 = Some(next);
                    workers[w].1 = (next as usize) - ('A' as usize) + EXTRA_TIME + 1;
                    all = all.into_iter().filter(|&t| t != next).collect();
                    all_idle = false;
                }
            } else {
                all_idle = false;
            }
        }
        if all_idle {
            break;
        }
        time += 1;
    }

    println!("{:?}", time);
}

fn parse_line(l: Result<String, Error>) -> (char, char) {
    lazy_static! {
        static ref re: regex::Regex = Regex::new(r" ([A-Z]) ").unwrap();
    }
    re.captures_iter(&l.expect("Read error"))
        .take(2)
        .map(|s| s.get(1).unwrap().as_str().chars().next().unwrap())
        .next_tuple()
        .unwrap()
}

fn available(wait: &Vec<char>, done: &Vec<char>, task_order: &Vec<(char, char)>) -> Option<char> {
    let not_yet: Vec<char> = task_order
        .iter()
        .filter(|t| !done.contains(&t.0))
        .map(|t| t.1)
        .collect();

    wait.iter()
        .filter(|c| !done.contains(c) && !not_yet.contains(c))
        .map(|c| *c)
        .min()
}

struct CharRangeInclusive(char, char);
impl Iterator for CharRangeInclusive {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.0 <= self.1 {
            let v = self.0;
            self.0 = (v as u8 + 1) as char;
            Some(v)
        } else {
            None
        }
    }
}
