use std::collections::HashMap;
use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()

fn main() {
    // Get the first line from standard input
    let stdin = io::stdin(); // need to lock the stdin for synced access like "next line"
                             // This needs to be a separate variable so it lives until the .fold at the end

    let totals: (isize, isize) = stdin
        .lock()
        .lines() // no newline bytes; is an iterator of Result<String> (which is String or Err)
        .map(|s| count(&s.unwrap()))
        .fold((0, 0), |(t0, t1), (b0, b1)| {
            (t0 + b0 as isize, t1 + b1 as isize)
        });
    println!("{:?}", totals.0 * totals.1);
}

fn count(s: &str) -> (bool, bool) {
    let mut h = HashMap::new();
    for c in s.chars() {
        h.insert(c, h.get(&c).unwrap_or(&0) + 1);
    }
    h.values()
        .map(|v| (*v == 2, *v == 3))
        .fold((false, false), |x, y| (x.0 || y.0, x.1 || y.1))
}
