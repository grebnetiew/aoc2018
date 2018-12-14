use std::io;
use std::io::BufRead;

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut elflist = ElfList::with_length(input + 10);
    //part1
    println!(
        "{:?}",
        elflist.list[input..(input + 10)]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
    );

    //part2
    let input_digits = input
        .to_string()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect::<Vec<u8>>();
    let mut start_from = 0;
    loop {
        elflist.list[start_from..]
            .windows(input_digits.len())
            .enumerate()
            .for_each(|(i, w)| {
                if w.to_vec() == input_digits {
                    println!("Input occurs at {:?}", start_from + i);
                }
            });
        start_from = elflist.list.len() - input_digits.len();
        elflist.extend(1000);
    }
}

struct ElfList {
    list: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl ElfList {
    fn new() -> ElfList {
        ElfList {
            list: vec![3, 7],
            elf1: 0,
            elf2: 1,
        }
    }
    fn with_length(length: usize) -> ElfList {
        let mut result = ElfList::new();
        result.extend(length);
        result
    }
    fn extend(&mut self, by: usize) {
        let target = self.list.len() + by;
        while self.list.len() < target {
            let (score1, score2) = (self.list[self.elf1], self.list[self.elf2]);
            let mut total = score1 + score2;
            if total > 9 {
                self.list.push(total / 10);
                total = total % 10;
            }
            self.list.push(total);
            self.elf1 = (self.elf1 + score1 as usize + 1) % self.list.len();
            self.elf2 = (self.elf2 + score2 as usize + 1) % self.list.len();
        }
    }
}
