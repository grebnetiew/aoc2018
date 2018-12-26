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
            re.captures(&l).map(|caps| Nanobot {
                pos: Point {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                    z: caps[3].parse().unwrap(),
                },
                r: caps[4].parse().unwrap(),
            })
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
            let candidates = vec![
                cursor,
                cursor.add(-step, 0, 0),
                cursor.add(step, 0, 0),
                cursor.add(0, -step, 0),
                cursor.add(0, step, 0),
                cursor.add(0, 0, -step),
                cursor.add(0, 0, step),
                cursor.add(-step / 2, -step / 2, 0),
                cursor.add(-step / 2, step / 2, 0),
                cursor.add(-step / 2, 0, -step / 2),
                cursor.add(-step / 2, 0, step / 2),
                cursor.add(step / 2, -step / 2, 0),
                cursor.add(step / 2, step / 2, 0),
                cursor.add(step / 2, 0, -step / 2),
                cursor.add(step / 2, 0, step / 2),
                cursor.add(0, -step / 2, -step / 2),
                cursor.add(0, -step / 2, step / 2),
                cursor.add(0, step / 2, -step / 2),
                cursor.add(0, step / 2, step / 2),
            ];
            // Find the candidate in range of the most bots
            let new_cursor = candidates
                .iter()
                .max_by_key(|pt| (pt.how_many_in_range(&bots), -pt.size(), pt.x, pt.y, pt.z))
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
        "The sweet spot is {:?}, with distance {:?} to the origin, visible to {:?}",
        best,
        best.size(),
        best.how_many_in_range(&bots)
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
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
    fn size(&self) -> isize {
        self.diff(&Point::new())
    }

    fn add(&self, dx: isize, dy: isize, dz: isize) -> Self {
        Point {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }

    fn how_many_in_range(&self, bots: &Vec<Nanobot>) -> usize {
        bots.iter()
            .filter(|&b| self.diff(&b.pos) as usize <= b.r)
            .count()
    }
}
