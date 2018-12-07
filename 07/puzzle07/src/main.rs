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
    let task_order: Vec<(char, char)> = io::stdin().lock().lines().map(|l| parse_line(l)).collect();

    let highest = task_order.iter().fold('A', |m, (a, b)| max(m, max(*a, *b)));
    let mut all: Vec<char> = CharRangeInclusive('A', highest).collect();

    // Part 1. Do the tasks in order
    let mut done = Vec::new();
    while let Some(do_more) = available(&all, &done, &task_order) {
        done.push(do_more);
    }

    println!("{:}", done.iter().collect::<String>());

    // Part 2. Do the tasks in parallel, measure time
    const WORKERS: usize = 5;
    const EXTRA_TIME: usize = 60;

    let mut workers: Vec<Task> = vec![Default::default(); WORKERS];
    let mut done = Vec::new();
    let mut time = 0;

    loop {
        // Step all jobs, and free workers who are done
        for w in workers.iter_mut() {
            if let Some(task) = w.current {
                w.time_left -= 1;
                if w.time_left == 0 {
                    done.push(task);
                    *w = Default::default();
                }
            }
        }
        // Give new jobs to idle workers
        for w in workers.iter_mut().filter(|w| w.current.is_none()) {
            if let Some(next_task) = available(&all, &done, &task_order) {
                *w = Task {
                    current: Some(next_task),
                    time_left: (next_task as usize) - ('A' as usize) + EXTRA_TIME + 1,
                };
                // Remove the task from the pile of stuff to do
                all = all.into_iter().filter(|&t| t != next_task).collect();
            }
        }
        // If everyone is still idle, we're done
        if workers.iter().filter(|w| w.current.is_some()).count() == 0 {
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

#[derive(Clone, Default)]
struct Task {
    current: Option<char>,
    time_left: usize,
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
