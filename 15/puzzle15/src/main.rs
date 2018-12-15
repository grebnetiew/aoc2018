use std::io;
use std::io::BufRead;

fn main() {
    let mut units: Vec<Point> = Vec::new();
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

    let mut units: Vec<Point> = Vec::new();
    for y in 0..cave_map.len() {
        for x in 0..cave_map.0[y].len() {
            if let Tile::Unit(_) = cave_map.0[y][x] {
                units.push(Point { y: y, x: x });
            }
        }
    }

    let mut round = 0;

    // Combat starts!
    loop {
        // A new round begins
        round += 1;

        let mut casualties: Vec<Point> = Vec::new();
        for p in units.iter() {
            // The unit at point p might have died
            if casualties.contains(p) {
                continue;
            }
            // The unit at point p acts
            if !cave_map.is_in_range_of_target(p) {
                *p = cave_map.move_to_enemy(p);
            }
            if cave_map.is_in_range_of_target(p) {
                if cave_map.attack_enemy(p) {
                    casualties.push(*p);
                }
            }
        }

        for p in casualties.iter() {
            if let Some(pos) = units.iter().position(|x| *x == *p) {
                units.remove(pos);
            }
        }

        if units.iter().all(|p| {
            cave_map.get(p).unit().unwrap().team() == cave_map.get(&units[0]).unit().unwrap().team()
        }) {
            break;
        }

        // Sort unit positions for next round
        units.sort();
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Unit(Unit),
}

impl Tile {
    fn unit(&self) -> Option<Unit> {
        match self {
            Tile::Unit(u) => Some(*u),
            _ => None,
        }
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

struct CaveMap(Vec<Vec<Tile>>);

impl CaveMap {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn get(&self, p: &Point) -> &Tile {
        &self.0[p.y][p.x]
    }
    fn get_mut(&mut self, p: &Point) -> &mut Tile {
        &mut self.0[p.y][p.x]
    }

    fn is_in_range_of_target(&self, p: &Point) -> bool {
        unimplemented!()
    }

    fn move_to_enemy(&self, p: &Point) -> Point {
        unimplemented!()
    }

    fn attack_enemy(&self, p: &Point) -> bool {
        unimplemented!()
    }
}
