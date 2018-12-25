use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() {
    let re = Regex::new(r"(-?\d+),(-?\d+),(-?\d+),(-?\d+)").unwrap();
    let points: Vec<Point> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| {
            re.captures(&l)
                .and_then(|c| Some(Point::new(&mut (1..=4).map(|i| c[i].parse().unwrap()))))
        })
        .collect();

    let mut constellations: Vec<Vec<Point>> = Vec::new();

    for p in points.iter() {
        // Is this point part of a constellation?
        let mut added_to: Option<usize> = None;
        for i in 0..constellations.len() {
            if p.part_of_const(&constellations[i]) {
                if let Some(other_const_index) = added_to {
                    let mut temp = constellations[i].drain(..).collect();
                    constellations[other_const_index].append(&mut temp);
                } else {
                    constellations[i].push(*p);
                    added_to = Some(i);
                }
            }
        }
        if added_to.is_none() {
            constellations.push(vec![*p]);
        }
    }
    println!(
        "There are {:?} constellations",
        constellations.iter().filter(|c| c.len() > 0).count()
    );
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}
impl Point {
    fn new(it: &mut Iterator<Item = i32>) -> Self {
        Point {
            w: it
                .next()
                .expect("Tried to construct a Point with no coordinates"),
            x: it
                .next()
                .expect("Tried to construct a Point with only 1 coordinate"),
            y: it
                .next()
                .expect("Tried to construct a Point with only 2 coordinates"),
            z: it
                .next()
                .expect("Tried to construct a Point with only 3 coordinates"),
        }
    }
    fn dist(&self, &other: &Self) -> i32 {
        (self.w - other.w).abs()
            + (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
    }
    fn part_of_const(&self, constellation: &Vec<Point>) -> bool {
        for p in constellation.iter() {
            if self.dist(&p) <= 3 {
                return true;
            }
        }
        false
    }
}
