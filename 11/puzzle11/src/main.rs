use std::default::Default;
use std::io;
use std::io::BufRead;

const GRIDSZ: usize = 300;

fn main() {
    let input: usize = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("Error: No lines")
        .expect("Error: Read error")
        .parse()
        .expect("Error: type a number");

    println!("{:?}", max_power_square(3, input));
    println!("{:?}", max_power(input));
}

fn power(x: usize, y: usize, serial: usize) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y + serial;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level as i32 - 5
}

fn max_power_square(block_size: usize, serial: usize) -> (usize, usize, i32) {
    // Precompute the partial sum grid
    let mut partial_sum_grid = vec![0; GRIDSZ * GRIDSZ];
    for y in 0..GRIDSZ {
        for x in 0..GRIDSZ {
            partial_sum_grid[y * GRIDSZ + x] =
                power(x + 1, y + 1, serial) - partial_sum(&partial_sum_grid, x, y, 1);
        }
    }

    let mut maximum_power = (0, 0, std::i32::MIN);

    for y in 0..(GRIDSZ - block_size + 1) {
        for x in 0..(GRIDSZ - block_size + 1) {
            let current_power = partial_sum(&partial_sum_grid, x, y, block_size);

            if current_power > maximum_power.2 {
                maximum_power = (x + 1, y + 1, current_power);
            }
        }
    }
    maximum_power
}

fn max_power(input: usize) -> (usize, usize, usize, i32) {
    let mut maximum_power = (0, 0, 0, std::i32::MIN);
    for bs in 1..=300 {
        let (x, y, pow) = max_power_square(bs, input);
        if pow > maximum_power.3 {
            maximum_power = (x, y, bs, pow);
        }
    }
    maximum_power
}

fn lenient_matrix_access<T>(m: &Vec<T>, x: Option<usize>, y: Option<usize>) -> T
where
    T: std::default::Default + std::marker::Copy,
{
    match (x, y) {
        (Some(x), Some(y)) => m[y * GRIDSZ + x],
        _ => Default::default(),
    }
}

fn partial_sum(m: &Vec<i32>, x: usize, y: usize, block_size: usize) -> i32 {
    lenient_matrix_access(m, Some(x + block_size - 1), Some(y + block_size - 1))
        - lenient_matrix_access(m, Some(x + block_size - 1), y.checked_sub(1))
        - lenient_matrix_access(m, x.checked_sub(1), Some(y + block_size - 1))
        + lenient_matrix_access(m, x.checked_sub(1), y.checked_sub(1))
}

#[test]
fn test() {
    assert_eq!(power(3, 5, 8), 4);
    assert_eq!(power(122, 79, 57), -5);
    assert_eq!(power(217, 196, 39), 0);
    assert_eq!(power(101, 153, 71), 4);

    assert_eq!(max_power_square(3, 18), (33, 45, 29));
    assert_eq!(max_power_square(3, 42), (21, 61, 30));

    assert_eq!(max_power(18), (90, 269, 16, 113));
    assert_eq!(max_power(42), (232, 251, 12, 119));
}
