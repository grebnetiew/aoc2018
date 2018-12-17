use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;
use std::io;
use std::io::BufRead;

fn main() {
    let cave_map: CaveMap = CaveMap(
        io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '#' => Tile::Wall,
                        'G' => Tile::Unit(Unit::Goblin(Status::new())),
                        'E' => Tile::Unit(Unit::Elf(Status::new())),
                        _ => Tile::Empty,
                    })
                    .collect()
            })
            .collect(),
    );

    let mut elf_strength = 3;

    let winning_outcome = loop {
        if let Some(outcome) = outcome_elves_win(cave_map.clone(), elf_strength) {
            break outcome;
        }
        elf_strength += 1;
    };

    println!(
        "Elves win for the first time with strength {:?}, outcome {:?}",
        elf_strength, winning_outcome
    );
}
fn outcome_elves_win(mut cave_map: CaveMap, elf_strength: usize) -> Option<usize> {
    let mut units: Vec<Point> = Vec::new();
    for y in 0..cave_map.len() {
        for x in 0..cave_map.0[y].len() {
            if let Tile::Unit(u) = cave_map.0[y][x] {
                units.push(Point { y: y, x: x });
                if let Unit::Elf(_) = u {
                    cave_map.0[y][x]
                        .unit_mut()
                        .unwrap()
                        .set_strength(elf_strength);
                }
            }
        }
    }

    let mut round = 1;

    // Combat starts!
    'combat: loop {
        // A new round begins
        let mut casualties: Vec<Point> = Vec::new();
        for i in 0..units.len() {
            // The unit at point p might have died
            if casualties.contains(&units[i]) {
                continue;
            }

            let team = cave_map.get(&units[i]).unit().unwrap().team();
            // The unit at point p checks if any targets remain
            if units
                .iter()
                .filter(|p| cave_map.get(p).unit().is_some())
                .all(|p| cave_map.get(p).unit().unwrap().team() == team)
            {
                break 'combat;
            }

            let p = &mut units[i];

            // The unit at point p acts

            *p = cave_map.move_to_enemy(&p);

            if let Some((enemy_pt, unit)) = cave_map.attack_enemy(p) {
                if unit.team() == Unit::Elf(Status::new()).team() {
                    return None;
                }
                casualties.push(enemy_pt);
            }
        }

        for p in casualties.iter() {
            if let Some(pos) = units.iter().position(|x| *x == *p) {
                units.remove(pos);
            }
        }

        // Sort unit positions for next round
        units.sort();
        round += 1;
    }
    let completed_rounds = round - 1; // Combat ends during a round

    let hp_total = units
        .iter()
        .filter_map(|p| Some(cave_map.get(&p).unit()?.status().hp))
        .sum::<usize>();
    let outcome = completed_rounds * hp_total;
    return Some(outcome);
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Unit(Unit),
    None,
}

impl Tile {
    fn unit(&self) -> Option<&Unit> {
        match self {
            Tile::Unit(u) => Some(&*u),
            _ => None,
        }
    }
    fn unit_mut(&mut self) -> Option<&mut Unit> {
        match self {
            Tile::Unit(u) => Some(&mut *u),
            _ => None,
        }
    }
    fn is_empty(&self) -> bool {
        match self {
            Tile::Empty => true,
            _ => false,
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::Unit(Unit::Elf(_)) => 'E',
                Tile::Unit(Unit::Goblin(_)) => 'G',
                Tile::None => '!',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Unit {
    Elf(Status),
    Goblin(Status),
}

impl Unit {
    fn team(&self) -> usize {
        match self {
            Unit::Elf(_) => 0,
            Unit::Goblin(_) => 1,
        }
    }
    fn status(&self) -> &Status {
        match self {
            Unit::Elf(s) | Unit::Goblin(s) => &*s,
        }
    }
    fn status_mut(&mut self) -> &mut Status {
        match self {
            Unit::Elf(s) | Unit::Goblin(s) => &mut *s,
        }
    }
    fn set_strength(&mut self, str: usize) {
        match self {
            Unit::Elf(s) | Unit::Goblin(s) => (*s).atk = str,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Status {
    atk: usize,
    hp: usize,
}

impl Status {
    fn new() -> Status {
        Status { atk: 3, hp: 200 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: usize, // To make y more important in orderings
    x: usize,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        // In reading order
        let mut nb = Vec::new();
        if self.y > 0 {
            nb.push(Point {
                x: self.x,
                y: self.y - 1,
            })
        }
        if self.x > 0 {
            nb.push(Point {
                x: self.x - 1,
                y: self.y,
            })
        }
        nb.push(Point {
            x: self.x + 1,
            y: self.y,
        });
        nb.push(Point {
            x: self.x,
            y: self.y + 1,
        });
        nb
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PointStep {
    point: Point,
    origin: Point,
    distance: usize,
}

// This implementation of Ord stolen from the docs, as they also use a bin heap
impl Ord for PointStep {
    fn cmp(&self, other: &PointStep) -> Ordering {
        // Notice that the we flip the orderings, so we get the element with minimal distance.
        // Same for positions: the lowest y,x takes precedence.
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| other.origin.cmp(&self.origin))
            .then_with(|| other.point.cmp(&self.point))
    }
}
// `PartialOrd` needs to be implemented as well.
impl PartialOrd for PointStep {
    fn partial_cmp(&self, other: &PointStep) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct CaveMap(Vec<Vec<Tile>>);

impl CaveMap {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn get(&self, p: &Point) -> &Tile {
        match &self.0.get(p.y) {
            Some(row) => row.get(p.x).unwrap_or(&Tile::None),
            None => &Tile::None,
        }
    }
    fn get_mut(&mut self, p: &Point) -> &mut Tile {
        &mut self.0[p.y][p.x]
    }
    fn set(&mut self, p: &Point, t: Tile) {
        *self.get_mut(p) = t;
    }

    fn is_in_range_of_target(&self, p: &Point) -> bool {
        if let Tile::Unit(u) = self.get(p) {
            self.is_in_range_of_team_target(p, u.team())
        } else {
            false
        }
    }
    fn is_in_range_of_team_target(&self, p: &Point, t: usize) -> bool {
        for other in p.neighbours().iter().filter_map(|pt| self.get(&pt).unit()) {
            if t != other.team() {
                return true;
            }
        }
        false
    }

    fn move_to_enemy(&mut self, p: &Point) -> Point {
        if let Tile::Unit(u) = self.get(p) {
            if self.is_in_range_of_target(p) {
                return *p;
            }
            if let Some(next_point) = self.shortest_path(p, u.team()) {
                self.set(&next_point, *self.get(p));
                self.set(p, Tile::Empty);

                return next_point;
            }
        }
        return *p;
    }
    fn shortest_path(&self, p: &Point, team: usize) -> Option<Point> {
        let mut prio_queue = BinaryHeap::new();
        let mut visited: Vec<Point> = Vec::new();
        // Put the neighbours in as origin points
        for other in p.neighbours().iter().filter(|pt| self.get(&pt).is_empty()) {
            prio_queue.push(PointStep {
                point: *other,
                origin: *other,
                distance: 1,
            });
        }
        while let Some(step) = prio_queue.pop() {
            if self.is_in_range_of_team_target(&step.point, team) {
                return Some(step.origin);
            }
            if step.distance > 50 {
                // Give up on long paths
                continue;
            }
            for other in step
                .point
                .neighbours()
                .iter()
                .filter(|pt| self.get(&pt).is_empty())
            {
                if !visited.contains(other) {
                    visited.push(*other);
                    prio_queue.push(PointStep {
                        point: *other,
                        origin: step.origin,
                        distance: step.distance + 1,
                    });
                }
            }
        }
        None
    }

    fn attack_enemy(&mut self, p: &Point) -> Option<(Point, Unit)> {
        // Find target
        let my_team = self.get(p).unit().unwrap().team();
        let my_atk = self.get(p).unit().unwrap().status().atk;

        let mut adjacent_enemies: Vec<Point> = p
            .neighbours()
            .iter()
            .filter(|pt| match self.get(&pt).unit() {
                Some(other) if my_team != other.team() => true,
                _ => false,
            })
            .map(|pt| *pt)
            .collect();
        adjacent_enemies.sort_by_key(|pt| self.get(&pt).unit().unwrap().status().hp);

        if let Some(other_pt) = adjacent_enemies.into_iter().next() {
            let other_unit = self.get_mut(&other_pt).unit_mut().unwrap();
            other_unit.status_mut().hp = other_unit.status().hp.saturating_sub(my_atk);
            if other_unit.status().hp == 0 {
                let dead_unit = other_unit.clone();
                self.set(&other_pt, Tile::Empty);
                return Some((other_pt, dead_unit));
            }
        }
        None
    }

    fn print(&self) {
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                print!("{}", self.0[y][x]);
            }
            println!();
        }
    }
}
