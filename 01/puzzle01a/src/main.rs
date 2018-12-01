use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()

fn main() {
    // Get the first line from standard input
    let stdin = io::stdin(); // need to lock the stdin for synced access like "next line"
                             // This needs to be a separate variable so it lives until the .fold at the end

    let total: i32 = stdin
        .lock()
        .lines() // no newline bytes; is an iterator of Result<String> (which is String or Err)
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .sum();

    println!("{}", total);
}
