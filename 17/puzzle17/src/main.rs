use regex::Regex;
use std::cmp::max;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;

fn main() {
    let re_vert = Regex::new(r"x=(\d+), y=(\d+)\.\.(\d+)").unwrap();
    let re_horz = Regex::new(r"y=(\d+), x=(\d+)\.\.(\d+)").unwrap();

    let mut horz: Vec<Vec<usize>> = Vec::new();
    let mut vert: Vec<Vec<usize>> = Vec::new();
    let mut xmax = 501; //for the spring
    let mut ymax = 0;

    for l in io::stdin().lock().lines().filter_map(Result::ok) {
        if let Some(caps) = re_vert.captures(&l) {
            let (x, ystart, yend) = (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            );
            vert.push(vec![x, ystart, yend]);
            xmax = max(xmax, x);
            ymax = max(ymax, max(ystart, yend));
        } else if let Some(caps) = re_horz.captures(&l) {
            let (y, xstart, xend) = (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            );
            horz.push(vec![y, xstart, xend]);
            ymax = max(ymax, y);
            xmax = max(xmax, max(xstart, xend));
        }
    }

    xmax += 5;
    ymax += 1;

    let mut map = vec![vec![Tile::Sand; xmax]; ymax];

    for l in vert.iter() {
        for y in l[1]..=l[2] {
            map[y][l[0]] = Tile::Clay;
        }
    }
    for l in horz.iter() {
        for x in l[1]..=l[2] {
            map[l[0]][x] = Tile::Clay;
        }
    }

    map[0][500] = Tile::Spring;

    let mut queue = VecDeque::new();
    queue.push_back(Point { x: 500, y: 1 });
    while let Some(point) = queue.pop_front() {
        map[point.y][point.x] = Tile::Flow;
        // Stop at bottom of the map
        if point.y + 1 == ymax {
            continue;
        }

        // Add falling water below
        if map[point.y + 1][point.x] == Tile::Sand {
            queue.push_back(Point {
                x: point.x,
                y: point.y + 1,
            });
        }
        // If below is clay or standing water, spread falling water to the sides
        if map[point.y + 1][point.x] == Tile::Clay || map[point.y + 1][point.x] == Tile::Water {
            // Find the sides of this pool (if it is one)
            let mut left_bound = point.x;
            let mut right_bound = point.x;
            while map[point.y][left_bound - 1].is_open()
                && map[point.y + 1][left_bound - 1].is_closed()
            {
                left_bound -= 1;
            }
            while map[point.y][right_bound + 1].is_open()
                && map[point.y + 1][right_bound + 1].is_closed()
            {
                right_bound += 1;
            }

            // Either it is the surface of a pool, and we should flow out to the side(s) ...
            let mut is_surface = false;
            if map[point.y][left_bound - 1].is_open() && map[point.y + 1][left_bound - 1].is_open()
            {
                is_surface = true;
                queue.push_back(Point {
                    x: left_bound - 1,
                    y: point.y,
                });
            }
            if map[point.y][right_bound + 1].is_open()
                && map[point.y + 1][right_bound + 1].is_open()
            {
                is_surface = true;
                queue.push_back(Point {
                    x: right_bound + 1,
                    y: point.y,
                });
            }

            if is_surface {
                // Fill up the sand in between with flowing water
                for x in left_bound..=right_bound {
                    map[point.y][x] = Tile::Flow;
                }
            } else {
                // ... or it is the (moving) surface inside a bucket, and the water should rise
                for x in left_bound..=right_bound {
                    map[point.y][x] = Tile::Water;
                    if map[point.y - 1][x] == Tile::Flow {
                        queue.push_back(Point {
                            x: x,
                            y: point.y - 1,
                        });
                    }
                }
            }
        }
    }

    // Done. Print the monster for now.
    for y in 0..ymax {
        for x in 0..xmax {
            print!(
                "{}",
                match map[y][x] {
                    Tile::Sand => '.',
                    Tile::Clay => '#',
                    Tile::Water => '~',
                    Tile::Spring => '+',
                    Tile::Flow => '|',
                }
            );
        }
        println!("");
    }
    println!("");

    println!(
        "Wet tiles: {:?} (check the drawing and subtract the top bars yourself)",
        map.iter()
            .map(|v| v
                .iter()
                .filter(|&&t| t == Tile::Water || t == Tile::Flow)
                .count())
            .sum::<usize>()
    );

    println!(
        "Retained tiles: {:?}",
        map.iter()
            .map(|v| v.iter().filter(|&&t| t == Tile::Water).count())
            .sum::<usize>()
    );
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Sand,
    Clay,
    Water,
    Spring,
    Flow,
}

impl Tile {
    fn is_open(&self) -> bool {
        match self {
            Tile::Sand | Tile::Flow => true,
            _ => false,
        }
    }
    fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
