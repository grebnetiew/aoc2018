use regex::Regex;
use std::cmp::max;
use std::io;
use std::io::BufRead;

fn main() {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    let bots: Vec<Nanobot> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| {
            if let Some(caps) = re.captures(&l) {
                Some(Nanobot {
                    pos: Point {
                        x: caps[1].parse().unwrap(),
                        y: caps[2].parse().unwrap(),
                        z: caps[3].parse().unwrap(),
                    },
                    r: caps[4].parse().unwrap(),
                })
            } else {
                None
            }
        })
        .collect();

    // Part 1
    let &Nanobot {
        pos: p_max,
        r: r_max,
    } = bots.iter().max_by_key(|bot| bot.r).unwrap();

    let num_bots_in_range = bots
        .iter()
        .filter(|&b| p_max.diff(&b.pos) as usize <= r_max)
        .count();
    println!("Bots in range: {:?}", num_bots_in_range);

    // Part 2
    let mut best = Point::new();
    for b in bots.iter() {
        // We find the most "Central" point starting from each bot
        // (just the bot from part 1 did not give me the right answer)
        let mut cursor = b.pos;
        let mut step_size = b.r;
        loop {
            let step = step_size as isize;
            // Steps in any of 18 directions are suitable candidates
            let mut candidates = vec![
                cursor,
                cursor.xadd(step),
                cursor.xadd(step / 2).yadd(step / 2),
                cursor.xadd(-step / 2).yadd(step / 2),
                cursor.xadd(-step),
                cursor.xadd(step / 2).yadd(-step / 2),
                cursor.xadd(-step / 2).yadd(-step / 2),
                cursor.yadd(step),
                cursor.xadd(step / 2).zadd(step / 2),
                cursor.xadd(-step / 2).zadd(step / 2),
                cursor.yadd(-step),
                cursor.xadd(step / 2).zadd(-step / 2),
                cursor.xadd(-step / 2).zadd(-step / 2),
                cursor.zadd(step),
                cursor.yadd(step / 2).zadd(step / 2),
                cursor.yadd(-step / 2).zadd(step / 2),
                cursor.zadd(-step),
                cursor.yadd(step / 2).zadd(-step / 2),
                cursor.yadd(-step / 2).zadd(-step / 2),
            ];
            // Sort them first by coordinates, otherwise you end up hopping between two points
            candidates.sort_by_key(|pt| pt.x);
            candidates.sort_by_key(|pt| pt.y);
            candidates.sort_by_key(|pt| pt.z);
            // Sort largest first, because max returns the last one if multiple are maximal
            candidates.sort_by_key(|pt| -pt.size());
            // Find the candidate in range of the most bots
            let new_cursor = candidates
                .iter()
                .max_by_key(|pt| pt.how_many_in_range(&bots))
                .unwrap();
            // Adjust step size
            if *new_cursor == cursor {
                if step_size == 1 {
                    break;
                } else {
                    step_size = max(step_size - max(step_size / 5, 1), 1);
                }
            }
            cursor = *new_cursor;
        }
        if best.how_many_in_range(&bots) < cursor.how_many_in_range(&bots) {
            best = cursor;
        }
    }
    println!(
        "The sweet spot is {:?}, with distance {:?} to the origin",
        best,
        best.size()
    );
}

#[derive(Debug, Eq, PartialEq)]
struct Nanobot {
    pos: Point,
    r: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0, z: 0 }
    }
    fn diff(&self, other: &Self) -> isize {
        (if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        }) + (if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        }) + (if self.z > other.z {
            self.z - other.z
        } else {
            other.z - self.z
        })
    }
    fn size(&self) -> isize {
        self.diff(&Point::new())
    }

    fn xadd(&self, dx: isize) -> Self {
        Point {
            x: self.x + dx,
            y: self.y,
            z: self.z,
        }
    }
    fn yadd(&self, dy: isize) -> Self {
        Point {
            x: self.x,
            y: self.y + dy,
            z: self.z,
        }
    }
    fn zadd(&self, dz: isize) -> Self {
        Point {
            x: self.x,
            y: self.y,
            z: self.z + dz,
        }
    }
    fn how_many_in_range(&self, bots: &Vec<Nanobot>) -> usize {
        bots.iter()
            .filter(|&b| self.diff(&b.pos) as usize <= b.r)
            .count()
    }
}
