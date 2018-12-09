use std::collections::VecDeque;
use std::io;
use std::io::BufRead;

fn main() {
    let numbers: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("Error: No lines")
        .expect("Error: Read error")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("{:?}", marblegame(numbers[0] as usize, numbers[1]));
    println!("{:?}", marblegame(numbers[0] as usize, numbers[1] * 100));
}

fn marblegame(players: usize, last_marble: u32) -> u32 {
    let mut ring: VecDeque<u32> = VecDeque::new();
    ring.push_back(0);
    let mut scores: Vec<u32> = vec![0; players];
    let mut current_player = 0;

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            cycle(&mut ring, -7);
            scores[current_player] += marble + ring[0];
            ring.pop_front();
        } else {
            cycle(&mut ring, 2);
            ring.push_front(marble);
        }
        current_player = (current_player + 1) % players;
    }
    *scores.iter().max().unwrap()
}

fn cycle(vd: &mut VecDeque<u32>, idx: isize) {
    if vd.len() == 0 {
        return;
    }
    for _ in 0..idx {
        let e = vd.pop_front().unwrap();
        vd.push_back(e);
    }
    for _ in 0..-idx {
        let e = vd.pop_back().unwrap();
        vd.push_front(e);
    }
}

#[test]
fn test() {
    assert_eq!(marblegame(9, 25), 32);
    assert_eq!(marblegame(10, 1618), 8317);
    assert_eq!(marblegame(13, 7999), 146373);
    assert_eq!(marblegame(17, 1104), 2764);
    assert_eq!(marblegame(21, 6111), 54718);
    assert_eq!(marblegame(30, 5807), 37305);
}
