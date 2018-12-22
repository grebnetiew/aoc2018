use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Read error");

    let mut iter = input.chars();
    println!("{:?}", solve(&mut iter));
}

fn solve(mut iter: &mut Iterator<Item = char>) -> usize {
    let mut maze = HashMap::new();
    maze.insert(Point { x: 0, y: 0 }, DoorToThe::new());
    build_maze(&mut iter, &mut maze, Point { x: 0, y: 0 });
    max_depth(&maze)
}

fn build_maze(
    mut iter: &mut Iterator<Item = char>,
    mut maze: &mut HashMap<Point, DoorToThe>,
    start: Point,
) -> (i32, i32, i32, i32) {
    let mut cursor = start;
    let (mut xmin, mut xmax, mut ymin, mut ymax) =
        (std::i32::MAX, std::i32::MIN, std::i32::MAX, std::i32::MIN);
    while let Some(c) = iter.next() {
        xmin = min(cursor.x, xmin);
        xmax = max(cursor.x, xmax);
        ymin = min(cursor.y, ymin);
        ymax = max(cursor.y, ymax);
        match c {
            '|' => cursor = start,
            '^' => continue,
            ')' | '$' => break,
            '(' => {
                let (nxmin, nxmax, nymin, nymax) = build_maze(&mut iter, &mut maze, cursor);
                xmin = min(nxmin, xmin);
                xmax = max(nxmax, xmax);
                ymin = min(nymin, ymin);
                ymax = max(nymax, ymax);
            }
            'N' => {
                maze.get_mut(&cursor).unwrap().north = true;
                cursor.y -= 1;
                maze.entry(cursor).or_insert(DoorToThe::new()).south = true;
            }
            'E' => {
                maze.get_mut(&cursor).unwrap().east = true;
                cursor.x += 1;
                maze.entry(cursor).or_insert(DoorToThe::new()).west = true;
            }
            'S' => {
                maze.get_mut(&cursor).unwrap().south = true;
                cursor.y += 1;
                maze.entry(cursor).or_insert(DoorToThe::new()).north = true;
            }
            'W' => {
                maze.get_mut(&cursor).unwrap().west = true;
                cursor.x -= 1;
                maze.entry(cursor).or_insert(DoorToThe::new()).east = true;
            }
            z => panic!("{:?}", z),
        }
    }
    (xmin, xmax, ymin, ymax)
}

fn max_depth(maze: &HashMap<Point, DoorToThe>) -> usize {
    let mut depth_map = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Status {
        p: Point { x: 0, y: 0 },
        d: 0,
    });

    while let Some(s) = queue.pop() {
        if depth_map.contains_key(&s.p) && depth_map[&s.p] <= s.d {
            continue;
        }
        depth_map.insert(s.p, s.d);

        if maze[&s.p].north {
            queue.push(Status {
                p: Point {
                    x: s.p.x,
                    y: s.p.y - 1,
                },
                d: s.d + 1,
            });
        }
        if maze[&s.p].east {
            queue.push(Status {
                p: Point {
                    x: s.p.x + 1,
                    y: s.p.y,
                },
                d: s.d + 1,
            });
        }
        if maze[&s.p].south {
            queue.push(Status {
                p: Point {
                    x: s.p.x,
                    y: s.p.y + 1,
                },
                d: s.d + 1,
            });
        }
        if maze[&s.p].west {
            queue.push(Status {
                p: Point {
                    x: s.p.x - 1,
                    y: s.p.y,
                },
                d: s.d + 1,
            });
        }
    }

    println!(
        "Answer to part 2: {:?} rooms are 1000 deep",
        depth_map
            .iter()
            .filter(|(&_p, &depth)| depth >= 1000)
            .count()
    );
    *depth_map
        .iter()
        .max_by_key(|(&_p, &depth)| depth)
        .unwrap()
        .1
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DoorToThe {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}
impl DoorToThe {
    fn new() -> Self {
        DoorToThe {
            north: false,
            east: false,
            south: false,
            west: false,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Status {
    p: Point,
    d: usize,
}

// This implementation of Ord stolen from the docs, as they also use a bin heap
impl Ord for Status {
    fn cmp(&self, other: &Status) -> Ordering {
        // Normally you'd get the "highest" element first.
        // Notice that the we flip the orderings, so we get the element with minimal depth.
        other.d.cmp(&self.d)
    }
}
// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Status) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test() {
    assert_eq!(solve(&mut "^WNE$".chars()), 3);
    assert_eq!(solve(&mut "^ENWWW(NEEE|SSE(EE|N))$".chars()), 10);
    assert_eq!(
        solve(&mut "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".chars()),
        18
    );
    assert_eq!(
        solve(&mut "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$".chars()),
        23
    );
    assert_eq!(
        solve(&mut "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".chars()),
        31
    );
}
