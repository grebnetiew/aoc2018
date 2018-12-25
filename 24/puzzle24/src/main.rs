#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::fmt;
use std::io;
use std::io::BufRead;
use std::ops::Deref;
use std::ops::DerefMut;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);

    let mut army_groups = Vec::new();
    let mut current_team = Team::Immunity;
    while let Some(l) = lines.next() {
        if let Some(army) = parse_line(&l, current_team) {
            army_groups.push(army);
        } else if l.starts_with("Infection") {
            current_team = Team::Infection;
        }
    }

    // Part 1
    let mut part1 = Battlefield(army_groups.clone());
    while part1.winner().is_none() {
        part1.play_round();
    }

    println!(
        "Winning army has {} units left",
        part1.iter().map(|a| a.units).sum::<u32>()
    );

    // Part 2
    for boost_amount in 1.. {
        let mut part2 = Battlefield(army_groups.clone());
        for a in part2.iter_mut().filter(|a| a.team == Team::Immunity) {
            a.damage += boost_amount;
        }
        while part2.winner().is_none() {
            let something_happened = part2.play_round();
            if !something_happened {
                // It's a stalemate
                break;
            }
        }

        if part2.winner() == Some(Team::Immunity) {
            println!(
                "Immunity first wins at boost amount {} and has {} units left",
                boost_amount,
                part2.iter().map(|a| a.units).sum::<u32>()
            );
            return;
        }
    }
}

fn parse_line(line: &String, team: Team) -> Option<ArmyGroup> {
    lazy_static! {
        static ref re_group: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (\([\w ;,]*\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        static ref re_weak: Regex = Regex::new(r"(weak|immune) to ([\w, ]+)").unwrap();
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
            let mut kinds = caps[2].split(", ").map(str::to_owned).collect();
            match &caps[1] {
                "weak" => army_group.weaknesses.append(&mut kinds),
                "immune" => army_group.immunities.append(&mut kinds),
                _ => panic!("This match of (weak|immune) matched neither?!"),
            }
        }
        Some(army_group)
    })
}

#[derive(Debug)]
struct Battlefield(Vec<ArmyGroup>);

// By implementing Deref(Mut), we ensure that Vec methods like .iter() and [] still work
impl Deref for Battlefield {
    type Target = Vec<ArmyGroup>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Battlefield {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Battlefield {
    fn play_round(&mut self) -> bool {
        // First we select targets
        let mut order: Vec<usize> = (0..self.len()).collect();
        // Groups select targets in decreasing order of effective power, then initiative
        order.sort_by_key(|&i| (self[i].effective_power(), self[i].initiative));
        order.reverse();

        // For each group, targets[i] is its target
        let mut targets: Vec<Option<usize>> = vec![None; self.len()];
        // For each group, targeted_by[i] is the group targeting it
        let mut targeted_by: Vec<Option<usize>> = vec![None; self.len()];

        for &i in order.iter() {
            // Group i chooses a target ...
            if let Some(target) = self
                .iter()
                .enumerate()
                .filter(|&(idx, candidate)| {
                    // ... considering only untargeted groups from the opposing team that they can damage
                    candidate.team != self[i].team
                        && targeted_by[idx].is_none()
                        && self[i].possible_damage(candidate) != 0
                })
                .max_by_key(|(_, candidate)| {
                    // ... selecting the one they can do the most damage to,
                    // breaking ties by selecting the one with highest power, then initiative
                    (
                        self[i].possible_damage(candidate),
                        candidate.effective_power(),
                        candidate.initiative,
                    )
                })
            {
                targets[i] = Some(target.0);
                targeted_by[target.0] = Some(i);
            }
        }

        // Now we do damage
        let mut order: Vec<usize> = (0..self.len()).collect();
        // Groups do damage in decreasing order of initiative
        order.sort_by_key(|&i| std::u32::MAX - self[i].initiative);

        let mut something_happened = false;

        for &i in order.iter() {
            // Dead groups do no damage
            if self[i].units == 0 {
                continue;
            }
            if let Some(target) = targets[i] {
                let dmg = self[i].possible_damage(&self[target]);
                let taken_damage = self[target].take_damage(dmg);
                something_happened |= taken_damage;
            }
        }

        // Remove any defeated groups
        self.retain(|grp| grp.units != 0);

        something_happened
    }

    fn winner(&self) -> Option<Team> {
        for i in 1..self.len() {
            if self[i].team != self[0].team {
                return None;
            }
        }
        Some(self[0].team)
    }

    #[allow(dead_code)]
    fn print_all(&self) {
        for a in self.iter() {
            println!("{}", a);
        }
        println!();
    }
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
