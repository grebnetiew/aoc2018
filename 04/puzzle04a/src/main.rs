extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()

fn main() {
    let re = Regex::new(
        r"\[(\d{4}-\d{2}-\d{2}) \d{2}:(\d{2})\] ((wakes up)|(falls asleep)|Guard #(\d+) begins shift)",
    )
    .unwrap();

    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();
    lines.sort();

    let mut sleep_times = HashMap::new();

    let mut lines_iter = lines.iter();
    let mut l = lines_iter.next();
    while l.is_some() {
        // Here, l contains a Guard... regex
        let guard_id = re
            .captures(l.unwrap())
            .unwrap()
            .get(6)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let this_guard_sleep_times = sleep_times.entry(guard_id).or_insert(Vec::new());
        loop {
            l = lines_iter.next();
            if l == None {
                break;
            }
            let caps = re.captures(l.unwrap()).unwrap();
            if caps.get(5) == None {
                // Not a Fall Asleep event
                break;
            }
            let minute_asleep = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let minute_awake = re
                .captures(lines_iter.next().unwrap())
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            for i in minute_asleep..minute_awake {
                this_guard_sleep_times.push(i);
            }
        }
        // l is None or is a new guard event
    }

    // we now have a Vec of sleep times sorted by guard
    let (longest_sleeper, _) = sleep_times
        .iter()
        .max_by_key(|(_guard_id, sleep_vec)| sleep_vec.len())
        .unwrap();

    // find the mode of the minutes vec
    let mut occurrences = HashMap::new();

    for &value in sleep_times.get(longest_sleeper).unwrap() {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let sleeps_at_minute = occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers");

    println!("{:?}", sleeps_at_minute * longest_sleeper);
}
