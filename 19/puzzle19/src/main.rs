#![feature(try_trait)] // Please use the nightly build, I want to use the shiny thing
use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() {
    let re_ip = Regex::new(r"^#ip (\d)$").unwrap();
    let re_line = Regex::new(r"^([a-z]+) (\d+) (\d+) (\d+)$").unwrap();

    let stdin = io::stdin();
    let mut input = stdin.lock().lines().filter_map(Result::ok);

    let ip_register: usize = re_ip.captures(&input.next().unwrap()).unwrap()[1]
        .parse()
        .unwrap();
    let program: Vec<Vec<usize>> = input
        .map(|line| {
            let caps = re_line.captures(&line).unwrap();
            vec![
                str_to_opcode(&caps[1]),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            ]
        })
        .collect();

    // Part 1
    let mut computer = Computer::new(ip_register);
    while computer.ip < program.len() {
        computer.execute(&program[computer.ip]);
    }

    println!("The value of register 0 is {:?}", computer.reg.0[0]);

    // Part 2
    let mut computer = Computer::with_values(ip_register, vec![1, 0, 0, 0, 0, 0]);
    for _ in 0..100 {
        computer.execute(&program[computer.ip]);
    }

    // The program sums all divisors of r3. Do it ourselves.
    let real_input = computer.reg.0[3];
    println!("The true input is {:?}", real_input);
    let mut answer = 0;
    for i in 1..=real_input {
        if real_input % i == 0 {
            answer += i;
        }
    }
    println!("The sum of divisors is {:?}", answer);
}

#[derive(Debug)]
struct Computer {
    reg: Registry,
    ip: usize,
    focused: usize,
}

impl Computer {
    fn new(focus: usize) -> Computer {
        Computer {
            reg: Registry::new(),
            ip: 0,
            focused: focus,
        }
    }
    fn with_values(focus: usize, initial: Vec<usize>) -> Computer {
        Computer {
            reg: Registry::with_values(initial),
            ip: 0,
            focused: focus,
        }
    }

    fn execute(&mut self, instr: &Vec<usize>) {
        self.reg.0[self.focused] = self.ip;
        self.reg
            .opcode(instr[0], instr[1], instr[2], instr[3])
            .expect("Invalid instruction");
        self.ip = self.reg.0[self.focused] + 1;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Registry(Vec<usize>);

type OpResult = Result<(), RegistryError>;
impl Registry {
    fn new() -> Registry {
        Registry(vec![0; 6])
    }
    fn with_values(initial: Vec<usize>) -> Registry {
        assert_eq!(6, initial.len());
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

fn str_to_opcode(s: &str) -> usize {
    match s {
        "addr" => 0x0,
        "addi" => 0x1,
        "mulr" => 0x2,
        "muli" => 0x3,
        "banr" => 0x4,
        "bani" => 0x5,
        "borr" => 0x6,
        "bori" => 0x7,
        "setr" => 0x8,
        "seti" => 0x9,
        "gtir" => 0xa,
        "gtri" => 0xb,
        "gtrr" => 0xc,
        "eqir" => 0xd,
        "eqri" => 0xe,
        "eqrr" => 0xf,
        _ => 16,
    }
}
