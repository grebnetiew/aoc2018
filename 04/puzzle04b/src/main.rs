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
        let this_guard_sleep_times = sleep_times.entry(guard_id).or_insert(HashMap::new());
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
                *this_guard_sleep_times.entry(i).or_insert(0) += 1;
            }
        }
        // l is None or is a new guard event
    }

    let (longest_sleeper, (sleeps_at_minute, _how_often)) = sleep_times
        .iter()
        .filter_map(|(guard_id, this_guard_sleep_times)| {
            Some((
                guard_id,
                this_guard_sleep_times
                    .iter()
                    .max_by_key(|(_minute, &sleep_count)| sleep_count)?,
            ))
        })
        .max_by_key(|(_guard_id, (&_sleeps_at_minute, &how_often))| how_often)
        .expect("B");

    println!("{:?}", sleeps_at_minute * longest_sleeper);
}
