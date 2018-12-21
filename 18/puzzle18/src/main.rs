use std::io;
use std::io::BufRead;

fn main() {
    let mut map: Vec<Vec<Tile>> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|l| l.len() > 0)
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '|' => Tile::Trees,
                    '#' => Tile::Lumberyard,
                    _ => Tile::Open,
                })
                .collect()
        })
        .collect();

    for _ in 1..=10 {
        map = next_map(&map);
    }

    println!("Total value after 10 minutes {:?}", value(&map));

    for _ in 11..=999 {
        map = next_map(&map);
    }
    // after a while, it becomes periodic
    let mut vals = Vec::new();
    for _ in 1000..1050 {
        map = next_map(&map);
        let this = value(&map);
        if vals.len() > 0 && vals[0] == this {
            // found repetition!
            // the value at vals[0] is the one after 1000 minutes
            // the period is vals.len()
            // we want the one after 1_000_000_000 minutes, so that's
            // 999_999_000 more minutes, and only the remainder after
            // division by the period "counts"
            println!(
                "Value after 1e9 minutes will be {}",
                vals[(1_000_000_000 - 1_000) % vals.len()]
            );
            return;
        }
        vals.push(value(&map));
        map = next_map(&map);
    }
}

fn next_map(old: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut new = old.clone();

    // Make a matrix of cumulative totals of trees/yards
    let mut tree_count = vec![vec![0; old[0].len()]; old.len()];
    let mut yard_count = vec![vec![0; old[0].len()]; old.len()];
    for y in 0..old.len() {
        for x in 0..old[0].len() {
            tree_count[y][x] = (old[y][x] == Tile::Trees) as usize;
            yard_count[y][x] = (old[y][x] == Tile::Lumberyard) as usize;
            if x > 0 {
                tree_count[y][x] += tree_count[y][x - 1];
                yard_count[y][x] += yard_count[y][x - 1];
            }
            if y > 0 {
                tree_count[y][x] += tree_count[y - 1][x];
                yard_count[y][x] += yard_count[y - 1][x];
            }
            if x > 0 && y > 0 {
                tree_count[y][x] -= tree_count[y - 1][x - 1];
                yard_count[y][x] -= yard_count[y - 1][x - 1];
            }
        }
    }

    // Update the tiles
    for y in 0..old.len() {
        for x in 0..old[0].len() {
            new[y][x] = if match old[y][x] {
                Tile::Open => calc_square_total(&tree_count, x, y) > 2,
                Tile::Trees => calc_square_total(&yard_count, x, y) > 2,
                Tile::Lumberyard => {
                    calc_square_total(&tree_count, x, y) == 0
                        || calc_square_total(&yard_count, x, y) == 1
                }
            } {
                old[y][x].next()
            } else {
                old[y][x]
            };
        }
    }
    new
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

impl Tile {
    fn next(&self) -> Tile {
        match *self {
            Tile::Open => Tile::Trees,
            Tile::Trees => Tile::Lumberyard,
            Tile::Lumberyard => Tile::Open,
        }
    }
}

fn calc_square_total(v: &Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let mut total = *v
        .get(j + 1)
        .unwrap_or(&v[j])
        .get(i + 1)
        .unwrap_or(&v.get(j + 1).unwrap_or(&v[j])[i]);
    if i > 1 && j > 1 {
        total += v[j - 2][i - 2];
    }
    if i > 1 {
        total -= v.get(j + 1).unwrap_or(&v[j])[i - 2];
    }
    if j > 1 {
        total -= v[j - 2].get(i + 1).unwrap_or(&v[j - 2][i]);
    }
    total
}

fn value(map: &Vec<Vec<Tile>>) -> usize {
    map.iter()
        .map(|v| v.iter().filter(|&&t| t == Tile::Trees).count())
        .sum::<usize>()
        * map
            .iter()
            .map(|v| v.iter().filter(|&&t| t == Tile::Lumberyard).count())
            .sum::<usize>()
}
