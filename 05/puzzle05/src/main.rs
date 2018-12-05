use std::io; // provides io's stdin()
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let elements: Vec<_> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .next()
        .expect("No input")
        .bytes()
        .collect();

    let lengths: Vec<_> = (0..26)
        .map(|i| react(remove_elem(&elements, 'A' as u8 + i)))
        .collect();

    println!("{:?}", react(elements.clone()));
    println!("{:?}", lengths.iter().min().unwrap());
}

const CAPITAL: u8 = ('a' as u8) - ('A' as u8);

fn react(mut elements: Vec<u8>) -> usize {
    let mut i = 0;
    while i + 1 < elements.len() {
        if elements[i] == elements[i + 1] + CAPITAL || elements[i] + CAPITAL == elements[i + 1] {
            elements.drain(i..(i + 2));
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    elements.len()
}

fn remove_elem(v: &Vec<u8>, elem: u8) -> Vec<u8> {
    v.iter()
        .filter(|&&n| n != elem && n != elem + CAPITAL)
        .map(|&n| n)
        .collect()
}
