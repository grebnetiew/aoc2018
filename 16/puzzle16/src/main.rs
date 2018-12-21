#![feature(try_trait)] // Please use the nightly build, I want to use the shiny thing
use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() {
    let re_befo = Regex::new(r"^Before: \[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
    let re_line = Regex::new(r"^(\d+) (\d+) (\d+) (\d+)$").unwrap();
    let re_aftr = Regex::new(r"^After:  \[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();

    let stdin = io::stdin();
    let mut input = stdin.lock().lines().filter_map(Result::ok);
    let mut testcases: Vec<Vec<usize>> = Vec::new();

    loop {
        let line = input.next().expect("Input ended after test cases");
        let caps = re_befo.captures(&line);
        if caps.is_none() {
            break;
        }

        let before_numbers = caps
            .unwrap()
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse().unwrap())
            .collect::<Vec<usize>>();

        let line = input.next().expect("Input ended during test case");
        let testing_numbers = re_line
            .captures(&line)
            .expect("Expected Sample line, got something else")
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let line = input.next().expect("Input ended during test case");
        let after_numbers = re_aftr
            .captures(&line)
            .expect("Expected After line, got something else")
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse().unwrap())
            .collect::<Vec<usize>>();

        input.next(); //empty line

        testcases.push(
            before_numbers
                .into_iter()
                .chain(testing_numbers.into_iter())
                .chain(after_numbers.into_iter())
                .collect(),
        );
    }

    // Phew! Test cases parsed. Now to find out which opcode is which
    let opcode_map = find_opcode_map(&testcases);

    // Now we execute the program!
    let mut reg = Registry::new();

    for program_line in input.filter(|line| re_line.is_match(&line)) {
        let captures = re_line.captures(&program_line).unwrap();
        reg.opcode(
            opcode_map[captures[1].parse::<usize>().unwrap()],
            captures[2].parse().unwrap(),
            captures[3].parse().unwrap(),
            captures[4].parse().unwrap(),
        )
        .expect("Program contained invalid statement");
    }

    println!("Register 0 has value {:?}", reg.0[0]);
}

fn find_opcode_map(testcases: &Vec<Vec<usize>>) -> Vec<usize> {
    // output[i] gives index of opcode in Registry::opcode for input i
    let mut possible_matches = vec![vec![true; 16]; 16];
    // p_m[i][C] to be true if input i could be opcode C

    // For part a, we track how many samples behave like 3 or more opcodes
    let mut behave_like_three_or_more = 0;

    for testcase in testcases.iter() {
        let after = Registry::with_values(testcase[8..12].to_vec());
        let (i, a, b, c) = (testcase[4], testcase[5], testcase[6], testcase[7]);

        let mut behaves_like_c = 0;
        for code in 0..16 {
            let mut before = Registry::with_values(testcase[0..4].to_vec());
            let result = before.opcode(code, a, b, c);
            if result.is_err() || before != after {
                possible_matches[i][code] = false;
            } else {
                behaves_like_c += 1;
            }
        }
        if behaves_like_c > 2 {
            behave_like_three_or_more += 1;
        }
    }

    println!(
        "{:?} samples out of {:?} behave like three or more opcodes",
        behave_like_three_or_more,
        testcases.len()
    );

    // Now to solve the sudoku
    let mut opcode_map = vec![16; 16];

    let mut sums_i: Vec<usize> = (0..16)
        .map(|i| (0..16).map(|c| possible_matches[i][c] as usize).sum())
        .collect();
    let mut sums_c: Vec<usize> = (0..16)
        .map(|c| (0..16).map(|i| possible_matches[i][c] as usize).sum())
        .collect();

    let mut found_match = true;
    while found_match {
        found_match = false;
        // if i behaves only like c, they must correspond
        for i in 0..16 {
            // For each input code i we check if only one c corresponds
            if sums_i[i] == 1 {
                found_match = true;
                // If so, we still have to find out which c that was
                let mut correct_c = 0;
                for c in 0..16 {
                    if possible_matches[i][c] {
                        correct_c = c;
                        break;
                    }
                }
                // Save the result in our Map of Truth
                opcode_map[i] = correct_c;
                // Now, all matches (i, *) and (*, c) become impossible.
                // Don't forget to update the sum columns if you erase a possible match
                for n in 0..16 {
                    if possible_matches[i][n] {
                        sums_i[i] -= 1;
                        sums_c[n] -= 1;
                    }
                    possible_matches[i][n] = false;
                    if possible_matches[n][correct_c] {
                        sums_i[n] -= 1;
                        sums_c[correct_c] -= 1;
                    }
                    possible_matches[n][correct_c] = false;
                }
            }
        }
    }

    opcode_map
}

#[derive(Debug, Eq, PartialEq)]
struct Registry(Vec<usize>);

type OpResult = Result<(), RegistryError>;
impl Registry {
    fn new() -> Registry {
        Registry(vec![0; 4])
    }
    fn with_values(initial: Vec<usize>) -> Registry {
        assert_eq!(4, initial.len());
        Registry(initial)
    }

    fn addr(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)? + *self.0.get(b)?)
    }
    fn addi(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)? + b)
    }

    fn mulr(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)? * *self.0.get(b)?)
    }
    fn muli(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)? * b)
    }

    fn banr(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)? & *self.0.get(b)?)
    }
    fn bani(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)? & b)
    }

    fn borr(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)? | *self.0.get(b)?)
    }
    fn bori(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)? | b)
    }

    fn setr(&mut self, a: usize, _b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = *self.0.get(a)?)
    }
    fn seti(&mut self, a: usize, _b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = a)
    }

    fn gtir(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = (a > *self.0.get(b)?) as usize)
    }
    fn gtri(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = (*self.0.get(a)? > b) as usize)
    }
    fn gtrr(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = (*self.0.get(a)? > *self.0.get(b)?) as usize)
    }

    fn eqir(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = (a == *self.0.get(b)?) as usize)
    }
    fn eqri(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = (*self.0.get(a)? == b) as usize)
    }
    fn eqrr(&mut self, a: usize, b: usize, c: usize) -> OpResult {
        Ok(self.0[c] = (*self.0.get(a)? == *self.0.get(b)?) as usize)
    }

    fn opcode(&mut self, oc: usize, a: usize, b: usize, c: usize) -> OpResult {
        match oc {
            0x0 => self.addr(a, b, c),
            0x1 => self.addi(a, b, c),
            0x2 => self.mulr(a, b, c),
            0x3 => self.muli(a, b, c),
            0x4 => self.banr(a, b, c),
            0x5 => self.bani(a, b, c),
            0x6 => self.borr(a, b, c),
            0x7 => self.bori(a, b, c),
            0x8 => self.setr(a, b, c),
            0x9 => self.seti(a, b, c),
            0xa => self.gtir(a, b, c),
            0xb => self.gtri(a, b, c),
            0xc => self.gtrr(a, b, c),
            0xd => self.eqir(a, b, c),
            0xe => self.eqri(a, b, c),
            0xf => self.eqrr(a, b, c),
            _ => Err(RegistryError::InvalidOpcode),
        }
    }
}

#[derive(Debug)]
enum RegistryError {
    RegisterIndexOutOfBounds,
    InvalidOpcode,
}
impl From<std::option::NoneError> for RegistryError {
    fn from(_: std::option::NoneError) -> Self {
        RegistryError::RegisterIndexOutOfBounds
    }
}
