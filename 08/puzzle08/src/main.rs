use std::io;
use std::io::BufRead;

fn main() {
    let numbers: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .next()
        .expect("Error: No lines")
        .expect("Error: Read error")
        .split(" ")
        .map(|s| s.parse().expect("Error: That wasn't a number"))
        .collect();

    println!("{:?}", take_metadata(&mut numbers.iter()));
    println!("{:?}", sum_nodevalue(&mut numbers.iter()));
}

// For my own purposes of learning the language, these functions each take an
// Iterator<usize> but they do so in different ways.
// This one takes the iterator as a generic/templated function. The compiler
// instantiates it for the type I will actually feed to it (likely std::iter::Map).
// This performs faster than the alternative, but takes more space if you have many
// kinds of iterators.
// You don't see the generic type T, as "impl" is syntactic sugar for that.

fn take_metadata<'a>(iter: &mut impl Iterator<Item = &'a usize>) -> usize {
    let num_nodes = *iter.next().unwrap();
    let num_mdata = *iter.next().unwrap();

    let total_meta: usize = (0..num_nodes).map(|_| take_metadata(iter)).sum();
    total_meta + iter.take(num_mdata).map(|s| *s).sum::<usize>()
}

// This function takes a "dyn" iterator, which Rust calls a Trait object (cf "impl"
// which would be a Trait implementation). As I understand it, it is like the vtables
// in C++: results in only one function sum_nodevalue, and .next checks at runtime
// what type your iterator is and which function .next it should call. Takes some time,
// saves space if you have many kinds of iterators.

fn sum_nodevalue(iter: &mut dyn Iterator<Item = &usize>) -> usize {
    let num_nodes = *iter.next().unwrap();
    let num_mdata = *iter.next().unwrap();

    if num_nodes == 0 {
        return iter.take(num_mdata).map(|s| *s).sum();
    }

    let child_values: Vec<_> = (0..num_nodes).map(|_| sum_nodevalue(iter)).collect();
    iter.take(num_mdata)
        .filter(|&&index| index <= num_nodes)
        .map(|&index| child_values[index.wrapping_sub(1)])
        .sum()
}
