#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::fmt;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);
    lines.next(); // skip 1st heading

    let mut army_groups = Vec::new();
    while let Some(l) = lines.next() {
        if let Some(army) = parse_line(&l, Team::Immunity) {
            army_groups.push(army);
        } else {
            break;
        }
    }
    while let Some(l) = lines.next() {
        if let Some(army) = parse_line(&l, Team::Infection) {
            army_groups.push(army);
        }
    }

    // Part 1
    let mut part1_armies = army_groups.clone();
    while winner(&part1_armies).is_none() {
        play_round(&mut part1_armies);
        part1_armies.retain(|a| a.units != 0);
    }

    println!(
        "Winning army has {} units left",
        part1_armies.iter().map(|a| a.units).sum::<u32>()
    );

    // Part 2
    let mut boost_amount = 0;
    loop {
        boost_amount += 1;
        let mut part2_armies = army_groups.clone();
        for a in part2_armies.iter_mut().filter(|a| a.team == Team::Immunity) {
            a.damage += boost_amount;
        }
        while winner(&part2_armies).is_none() {
            let something_happened = play_round(&mut part2_armies);
            if !something_happened {
                // It's a stalemate
                break;
            }
            part2_armies.retain(|a| a.units != 0);
        }

        if winner(&part2_armies) == Some(Team::Immunity) {
            println!(
                "Immunity first wins at boost amount {} and has {} units left",
                boost_amount,
                part2_armies.iter().map(|a| a.units).sum::<u32>()
            );
            return;
        }
    }
}

fn parse_line(line: &String, team: Team) -> Option<ArmyGroup> {
    lazy_static! {
        static ref re_group: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (\([a-z ;,]*\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        static ref re_weak: Regex = Regex::new(r"(weak|immune) to (\w+)(, (\w+))?").unwrap();
    }

    re_group.captures(line).and_then(|caps| {
        let mut army_group = ArmyGroup {
            team: team,
            units: caps[1].parse().unwrap(),
            hp_each: caps[2].parse().unwrap(),
            damage: caps[4].parse().unwrap(),
            damage_type: caps[5].to_owned(),
            initiative: caps[6].parse().unwrap(),
            weaknesses: Vec::new(),
            immunities: Vec::new(),
        };
        for caps in re_weak.captures_iter(line) {
            let mut kinds = vec![caps[2].to_owned()];
            if let Some(kind) = caps.get(4) {
                kinds.push(kind.as_str().to_owned());
            }
            match &caps[1] {
                "weak" => army_group.weaknesses.append(&mut kinds),
                "immune" => army_group.immunities.append(&mut kinds),
                _ => panic!("Unknown immunity modifier"),
            }
        }
        Some(army_group)
    })
}

fn play_round(armies: &mut Vec<ArmyGroup>) -> bool {
    // First we select targets
    let mut order: Vec<usize> = (0..armies.len()).collect();
    order.sort_by_key(|&i| (armies[i].effective_power(), armies[i].initiative));
    order.reverse();

    // For each army, targets[i] is its target
    let mut targets: Vec<Option<usize>> = vec![None; armies.len()];
    // For each army, targeted_by[i] is the army targeting it
    let mut targeted_by: Vec<Option<usize>> = vec![None; armies.len()];

    for i in order.iter() {
        // army i chooses a target
        if let Some(target) = armies
            .iter()
            .enumerate()
            .filter(|(idx, candidate)| {
                candidate.team != armies[*i].team
                    && targeted_by[*idx].is_none()
                    && armies[*i].possible_damage(candidate) != 0
            })
            .max_by_key(|(_, candidate)| {
                (
                    armies[*i].possible_damage(candidate),
                    candidate.effective_power(),
                    candidate.initiative,
                )
            })
        {
            targets[*i] = Some(target.0);
            targeted_by[target.0] = Some(*i);
        }
    }

    // Now we do damage
    let mut order: Vec<usize> = (0..armies.len()).collect();
    order.sort_by_key(|&i| std::u32::MAX - armies[i].initiative);

    let mut something_happened = false;

    for i in order.iter() {
        if armies[*i].units == 0 {
            continue;
        }
        if let Some(target) = targets[*i] {
            let dmg = armies[*i].possible_damage(&armies[target]);
            let taken_damage = armies[target].take_damage(dmg);
            something_happened |= taken_damage;
        }
    }

    something_happened
}

fn winner(armies: &Vec<ArmyGroup>) -> Option<Team> {
    for i in 1..armies.len() {
        if armies[i].team != armies[0].team {
            return None;
        }
    }
    Some(armies[0].team)
}

#[allow(dead_code)]
fn print_all(armies: &Vec<ArmyGroup>) {
    for a in armies.iter() {
        println!("{}", a);
    }
    println!();
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ArmyGroup {
    team: Team,
    units: u32,
    hp_each: u32,
    damage: u32,
    damage_type: String,
    initiative: u32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Team {
    Immunity,
    Infection,
}

impl fmt::Display for ArmyGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} ({} x {}) !{}({}) W{:?} I{:?}",
            self.team,
            self.units,
            self.hp_each,
            self.damage,
            self.damage_type,
            self.weaknesses,
            self.immunities
        )
    }
}

impl ArmyGroup {
    fn effective_power(&self) -> u32 {
        self.units * self.damage
    }
    fn possible_damage(&self, victim: &ArmyGroup) -> u32 {
        let (base_damage, mut multiplier) = (self.effective_power(), 1);
        if victim.immunities.contains(&self.damage_type) {
            multiplier = 0;
        } else if victim.weaknesses.contains(&self.damage_type) {
            multiplier = 2;
        }
        base_damage * multiplier
    }
    fn take_damage(&mut self, dmg: u32) -> bool {
        let old_units = self.units;
        self.units = self.units.saturating_sub(dmg / self.hp_each);
        self.units != old_units
    }
}
