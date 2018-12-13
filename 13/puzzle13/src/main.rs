use std::cell::RefCell;
use std::io;
use std::io::BufRead;

fn main() {
    // Turn the input into a table of chars
    let mut tracks: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|s| s.chars().collect())
        .collect();

    // Collect all the trains, and replace the ><^v by tracks
    let mut trains: Vec<RefCell<Train>> = Vec::new();
    for (y, l) in tracks.iter_mut().enumerate() {
        for (x, c) in l.iter_mut().enumerate() {
            if let Some(d) = Direction::try_from(c) {
                trains.push(RefCell::new(Train::new(x, y, d)));
                *c = match d {
                    Direction::North | Direction::South => '|',
                    _ => '-',
                }
            }
        }
    }

    // Simulate train movement
    loop {
        let mut crashed_trains: Vec<usize> = Vec::new();
        // Step all the trains
        // Sort them first
        trains.sort_by(|t1, t2| {
            t1.borrow()
                .y
                .cmp(&t2.borrow().y)
                .then(t1.borrow().x.cmp(&t2.borrow().x))
        });
        for (idx, t) in trains.iter().enumerate() {
            // Maybe it's already dead
            if crashed_trains.contains(&idx) {
                continue;
            }

            let mut t = t.borrow_mut();
            // Move it
            t.advance();
            // Turn it
            let track = tracks[t.y][t.x];
            t.turn(track);

            // Crash it
            for (jdx, other_t) in trains.iter().enumerate() {
                if idx == jdx {
                    continue;
                }
                let other_t = other_t.borrow();
                if t.x == other_t.x && t.y == other_t.y && !crashed_trains.contains(&jdx) {
                    // crash found!
                    println!("Crash at {:?},{:?}", t.x, t.y);
                    crashed_trains.push(idx);
                    crashed_trains.push(jdx);
                }
            }
        }

        // Remove all the crashed trains
        crashed_trains.sort_by(|a, b| b.cmp(a));
        // Sort in reverse, so we remove high indices first
        for idx in crashed_trains.iter() {
            trains.remove(*idx);
        }
        if trains.len() == 1 {
            println!(
                "Last train at {:?},{:?}",
                trains[0].borrow().x,
                trains[0].borrow().y
            );
            return;
        }
    }
}

struct Train {
    x: usize,
    y: usize,
    dir: Direction,
    next_turn: RelativeDirection,
}

impl Train {
    fn new(x: usize, y: usize, dir: Direction) -> Train {
        Train {
            x: x,
            y: y,
            dir: dir,
            next_turn: RelativeDirection::Left,
        }
    }
    fn advance(&mut self) {
        match self.dir {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        }
    }
    fn turn(&mut self, track: char) {
        let rd = match (track, self.dir) {
            ('/', Direction::North)
            | ('/', Direction::South)
            | ('\\', Direction::East)
            | ('\\', Direction::West) => RelativeDirection::Right,
            ('/', Direction::East)
            | ('/', Direction::West)
            | ('\\', Direction::North)
            | ('\\', Direction::South) => RelativeDirection::Left,
            ('+', _) => {
                let d = self.next_turn;
                self.next_turn = self.next_turn.next();
                d
            }
            _ => RelativeDirection::Straight,
        };
        self.dir.add_to(rd);
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn add(self, rd: RelativeDirection) -> Direction {
        From::from((self as i32 + rd as i32) % 4)
    }
    fn add_to(&mut self, rd: RelativeDirection) {
        *self = self.add(rd);
    }
    fn try_from(c: &char) -> Option<Direction> {
        match *c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        }
    }
}

impl From<i32> for Direction {
    fn from(n: i32) -> Direction {
        match n {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West, // also catches -1 from Direction::add :)
        }
    }
}

#[derive(Clone, Copy)]
enum RelativeDirection {
    Left = -1,
    Straight = 0,
    Right = 1,
}

impl RelativeDirection {
    fn next(self) -> RelativeDirection {
        From::from(self as i32 + 1)
    }
}

impl From<i32> for RelativeDirection {
    fn from(n: i32) -> RelativeDirection {
        match n {
            0 => RelativeDirection::Straight,
            1 => RelativeDirection::Right,
            _ => RelativeDirection::Left,
        }
    }
}
