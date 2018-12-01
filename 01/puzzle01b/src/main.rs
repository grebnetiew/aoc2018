use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()
use std::collections::HashSet;

fn main() {
    // Get the first line from standard input
    let stdin = io::stdin(); // need to lock the stdin for synced access like "next line"

    let numbers : Vec<i32> = stdin.lock()
		.lines() // no newline bytes; is an iterator of Result<String> (which is String or Err)
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut seen = HashSet::new();
    let mut current = 0;
    seen.insert(current);

    loop {
    	for n in numbers.iter() { // It doesn't work with just "numbers", as this moves the Vec and the next loop{} can't use it
    		current += n;
    		if seen.contains(&current) {
    			println!("{}",current);
    			return
    		}
    		seen.insert(current);
    	}
    }
}
