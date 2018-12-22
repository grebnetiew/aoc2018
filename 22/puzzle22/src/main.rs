use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;
use std::io::Read;

const PRIME: usize = 20183;
const WIDTH: usize = 100;
const HEIGHT: usize = 900;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Read error");

    let re = Regex::new(r"(\d+)").unwrap();
    let mut numbers = re.find_iter(&input);
    let depth: usize = numbers.next().unwrap().as_str().parse().unwrap();
    let target_x = numbers.next().unwrap().as_str().parse().unwrap();
    let target_y = numbers.next().unwrap().as_str().parse().unwrap();

    let mut geologic_index = vec![vec![0; WIDTH + 1]; HEIGHT + 1];
    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            geologic_index[y][x] = match (x, y) {
                (0, 0) => 0,
                (_, 0) => (16807 * x) % PRIME,
                (0, _) => (48271 * y) % PRIME,
                (_, _) => {
                    if x == target_x && y == target_y {
                        0
                    } else {
                        ((geologic_index[y - 1][x] + depth) * (geologic_index[y][x - 1] + depth))
                            % PRIME
                    }
                }
            }
        }
    }

    let mut terrain_type = vec![vec![0; WIDTH + 1]; HEIGHT + 1];
    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            terrain_type[y][x] = ((geologic_index[y][x] + depth) % PRIME) % 3;
        }
    }
    let mut risk_index = 0;
    for y in 0..=target_y {
        for x in 0..=target_x {
            risk_index += terrain_type[y][x];
        }
    }

    println!("Risk index {:?}", risk_index);

    println!(
        "Time spent {:?}",
        shortest_path(
            &terrain_type,
            Point { x: 0, y: 0 },
            Point {
                x: target_x,
                y: target_y
            }
        )
    );
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
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
    time_spent: usize,
    tool: usize,
}

// This implementation of Ord stolen from the docs, as they also use a bin heap
impl Ord for PointStep {
    fn cmp(&self, other: &PointStep) -> Ordering {
        // Normally you'd get the "highest" element first.
        // Notice that the we flip the orderings, so we get the element with minimal distance.
        other.time_spent.cmp(&self.time_spent)
    }
}
// `PartialOrd` needs to be implemented as well.
impl PartialOrd for PointStep {
    fn partial_cmp(&self, other: &PointStep) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct PointAndTool {
    p: Point,
    t: usize,
}

// Yes, tool's not an enum. It's more easy to manipulate this way. I know, it's a hack :)

fn shortest_path(terrain: &Vec<Vec<usize>>, start: Point, end: Point) -> usize {
    let mut prio_queue = BinaryHeap::new();
    let mut visited: HashMap<PointAndTool, usize> = HashMap::new();
    // Put the neighbours in as origin points
    prio_queue.push(PointStep {
        point: start,
        time_spent: 0,
        tool: 1,
    });
    while let Some(step) = prio_queue.pop() {
        if !compatible(terrain[step.point.y][step.point.x], step.tool) {
            panic!("Tool??!!");
        }
        if step.point == end && step.tool == 1 {
            return step.time_spent;
        }

        for nb in step
            .point
            .neighbours()
            .iter()
            .filter(|pt| pt.x < terrain[0].len() && pt.y < terrain.len())
        {
            if (!visited.contains_key(&PointAndTool {
                p: *nb,
                t: step.tool,
            }) || visited[&PointAndTool {
                p: *nb,
                t: step.tool,
            }] > step.time_spent + 1)
                && compatible(terrain[nb.y][nb.x], step.tool)
            {
                visited.insert(
                    PointAndTool {
                        p: *nb,
                        t: step.tool,
                    },
                    step.time_spent + 1,
                );
                prio_queue.push(PointStep {
                    point: *nb,
                    time_spent: step.time_spent + 1,
                    tool: step.tool,
                });
            }
        }
        let other_tool = if compatible(terrain[step.point.y][step.point.x], (step.tool + 1) % 3) {
            (step.tool + 1) % 3
        } else {
            (step.tool + 2) % 3
        };
        if !visited.contains_key(&PointAndTool {
            p: step.point,
            t: other_tool,
        }) {
            visited.insert(
                PointAndTool {
                    p: step.point,
                    t: other_tool,
                },
                step.time_spent + 7,
            );
            prio_queue.push(PointStep {
                point: step.point,
                time_spent: step.time_spent + 7,
                tool: other_tool,
            });
        }
    }
    panic!("Can't find it")
}

fn compatible(terrain: usize, tool: usize) -> bool {
    // 0: rocky; neither
    // 1: wet; torch
    // 2: narrow; climbing
    terrain != tool
}
