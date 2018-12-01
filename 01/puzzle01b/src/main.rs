use std::cell::Cell;
use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()
use std::collections::HashSet;

fn main() {
    // Get the first line from standard input
    let stdin = io::stdin(); // need to lock the stdin for synced access like "next line"

    let mut seen = HashSet::new();
    let current = Cell::new(0);

    let numbers: Vec<i32> = stdin.lock()
		.lines()                                    // is an iterator of Result<String> (which is String or Err)
        .map(|s| s.unwrap().parse().unwrap())       // turn into numbers, i32 inferred from next line
        .collect();                                 // cycling the stdin gives Issues(tm), so we take a breather
    numbers.iter()
        .cycle()                                    // make it wrap around
        .take_while(|_| seen.insert(current.get())) // stop if current is already in seen
        .for_each(|n| {
            current.set(current.get() + n)          // add to the current value
        });

    println!("{:?}", current.get());
}
