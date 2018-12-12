use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let input: Vec<String> = io::stdin().lock().lines().filter_map(Result::ok).collect();

    // The first line is the starting state. Look only for . # characters.
    let mut pots_with_plants = HashSet::new();
    for (pot, c) in input[0]
        .chars()
        .filter(|&c| c == '#' || c == '.')
        .enumerate()
    {
        match c {
            '#' => pots_with_plants.insert(pot as i64),
            _ => false,
        };
    }

    // Rules will be indexed by an integer representing the five pots "before",
    // encoding them as bits. So #..#. => # will be rules[18] == true.
    let mut rules = vec![false; 2usize.pow(5)];
    let re = Regex::new(r"(?P<before>[#.]{5}) => (?P<after>[#.])").unwrap();
    for caps in input.iter().skip(1).filter_map(|s| re.captures(s)) {
        // Compute the index by turning #/. into 1/0 and shifting it left.
        let conf: usize = caps["before"]
            .chars()
            .enumerate()
            .map(|(i, c)| ((c == '#') as usize) << (4 - i))
            .fold(0, std::ops::BitOr::bitor);
        rules[conf] = caps["after"].chars().next().unwrap() == '#';
    }

    // Part 1
    for _ in 0..20 {
        pots_with_plants = iteration(&pots_with_plants, &rules);
    }
    println!("{:?}", pots_with_plants.iter().sum::<i64>());

    // Part 2. Wait for the game of pots to stabilize
    for _ in 20..500 {
        pots_with_plants = iteration(&pots_with_plants, &rules);
    }

    // The answer is just the sum, so if there are any traveling bits
    // ..###...
    // ...###..
    // this will lead to a linear change in the sum of pot numbers.
    // Find out the rate of change, and extrapolate.

    let sum1 = pots_with_plants.iter().sum::<i64>();
    // println!("Iteration 500 is {:?}", sum1);
    pots_with_plants = iteration(&pots_with_plants, &rules);
    let sum2 = pots_with_plants.iter().sum::<i64>();
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
                .fold(0, std::ops::BitOr::bitor);
            if rules[conf] {
                result.insert(pot);
            }
        }
    }
    result
}
