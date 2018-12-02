use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()

fn main() {
    // Get the first line from standard input
    let stdin = io::stdin(); // need to lock the stdin for synced access like "next line"

    let boxids: Vec<String> = stdin
        .lock()
        .lines() // no newline bytes; is an iterator of Result<String> (which is String or Err)
        .map(|s| s.unwrap())
        .collect();

    for (i, s) in boxids.iter().enumerate() {
        for t in &boxids[0..i] {
            let (removed, commons) = find_common_characters(s, t);
            if removed == 1 {
                println!("{:?}", commons);
                return;
            }
        }
    }
}

fn find_common_characters(s: &String, t: &String) -> (isize, String) {
    let mut removed = 0;
    let mut commons = String::new();
    assert_eq!(s.len(), t.len());
    for (c, d) in s.chars().zip(t.chars()) {
        if c != d {
            removed += 1;
        } else {
            commons.push(c);
        }
    }
    (removed, commons)
}
