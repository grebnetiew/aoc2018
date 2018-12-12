use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().filter_map(Result::ok).collect();

    let mut current_state = HashSet::new();
    for (i, c) in input[0]
        .chars()
        .filter(|&c| c == '#' || c == '.')
        .enumerate()
    {
        match c {
            '#' => current_state.insert(i as i64),
            _ => false,
        };
    }

    let mut rules = vec![false; 2usize.pow(5)];
    let re = Regex::new(r"([#.])([#.])([#.])([#.])([#.]) => ([#.])").unwrap();
    for caps in input.iter().skip(1).filter_map(|s| re.captures(s)) {
        let conf: usize = (0..5)
            .map(|i| {
                ((caps.get(i + 1).unwrap().as_str().chars().next().unwrap() == '#') as usize)
                    << (4 - i)
            })
            .sum();
        rules[conf] = caps.get(6).unwrap().as_str().chars().next().unwrap() == '#';
    }

    // Part 1
    for _ in 0..20 {
        current_state = iteration(&current_state, &rules);
    }
    println!("{:?}", current_state.iter().sum::<i64>());

    // Part 2. Wait for the game of pots to stabilize
    for _ in 20..500 {
        current_state = iteration(&current_state, &rules);
    }
    // The answer is just the sum, so if there are any traveling bits
    // ..###...
    // ...###..
    // this will lead to a linear change in the sum of pot numbers.
    // Find out the rate of change, and extrapolate.
    let sum1 = current_state.iter().sum::<i64>();
    // println!("Iteration 500 is {:?}", sum1);
    current_state = iteration(&current_state, &rules);
    let sum2 = current_state.iter().sum::<i64>();
    // println!("Iteration 501 is {:?}", sum2);
    let slope = sum2 - sum1;
    let dt = 50_000_000_000 - 500;
    println!("Iteration 50e9 is {:?}", sum1 + slope * dt);
}

fn iteration(current_state: &HashSet<i64>, rules: &Vec<bool>) -> HashSet<i64> {
    let mut result = HashSet::new();
    // I am abusing input knowledge here that ..... => ., so I only have to look around
    // existing pots
    for oldpot in current_state {
        for pot in (oldpot - 2)..=(oldpot + 2) {
            let conf: usize = (0..5)
                .map(|i| (current_state.contains(&(pot + 2 - i)) as usize) << i)
                .sum();
            if rules[conf] {
                result.insert(pot);
            }
        }
    }
    result
}
