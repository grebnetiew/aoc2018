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
        for p in units.iter_mut() {
            // The unit at point p might have died
            if casualties.contains(p) {
                continue;
            }
            // The unit at point p acts
            *p = cave_map.move_to_enemy(p);

            if let Some(enemy_pt) = cave_map.attack_enemy(p) {
                casualties.push(enemy_pt);
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
    None,
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
            x: self.y,
            y: self.x + 1,
        });
        nb
    }
}

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
            for other in p.neighbours().iter().filter_map(|pt| self.get(&pt).unit()) {
                if u.team() != other.team() {
                    return true;
                }
            }
        }
        false
    }

    fn move_to_enemy(&self, p: &Point) -> Point {
        if let Tile::Unit(_) = self.get(p) {
            if self.is_in_range_of_target(p) {
                return *p;
            }
        }
        return *p;
    }

    fn attack_enemy(&mut self, p: &Point) -> Option<Point> {
        // Find target
        let my_team = self.get(p).unit().unwrap().team();
        let my_atk = self.get(p).unit().unwrap().status().atk;

        if let Some((other_pt, mut other_unit)) = p
            .neighbours()
            .iter_mut()
            .filter_map(|pt| match self.get_mut(&pt).unit() {
                Some(other) if my_team != other.team() => Some((pt, other)),
                _ => None,
            })
            .next()
        {
            other_unit.status_mut().hp = other_unit.status().hp.saturating_sub(my_atk);
            if other_unit.status().hp == 0 {
                self.set(other_pt, Tile::Empty);
                return Some(*other_pt);
            }
        }

        None
    }
}
