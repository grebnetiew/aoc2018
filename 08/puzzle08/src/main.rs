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

    println!("{:?}", take_metadata(&mut numbers.iter().map(|s| *s)));
    println!("{:?}", take_node_val(&mut numbers.iter().map(|s| *s)));
}

// For my own purposes of learning the language, these functions each take an
// Iterator<usize> but they do so in different ways.
// This one takes the iterator as a generic/templated function. The compiler
// instantiates it for the type I will actually feed to it (likely std::iter::Map).
// This performs faster than the alternative, but takes more space if you have many
// kinds of iterators.
// Using an Item = &usize, which would be more elegant, leads to a headache with
// lifetimes I have not been able to solve (because of the recursive call).

fn take_metadata<T>(iter: &mut T) -> usize
where
    T: Iterator<Item = usize>,
{
    let num_nodes = iter.next().unwrap();
    let num_mdata = iter.next().unwrap();
    let total_meta = (0..num_nodes).map(|_| take_metadata(iter)).sum::<usize>();
    total_meta
        + (0..num_mdata)
            .map(|_| (*iter).next().unwrap())
            .sum::<usize>()
}

// This function takes a "dyn" iterator, which Rust calls a Trait object (cf "impl"
// which would be a Trait implementation). As I understand it, it is like the vtables
// in C++: results in only one function take_node_val, and .next checks at runtime
// what type your iterator is and which function .next it should call. Takes some time,
// saves space if you have many kinds of iterators.
// Typing "impl" instead of "dyn" fails catastrophically. I'm not sure why.

fn take_node_val(iter: &mut dyn Iterator<Item = usize>) -> usize {
    let num_nodes = iter.next().unwrap();
    let num_mdata = iter.next().unwrap();

    // We still have to iterate over the child nodes, even if they're not needed
    let child_values = (0..num_nodes)
        .map(|_| take_node_val(iter))
        .collect::<Vec<_>>();

    if num_nodes == 0 {
        return (0..num_mdata)
            .map(|_| (*iter).next().unwrap())
            .sum::<usize>();
    }

    (0..num_mdata)
        .map(|_| (*iter).next().unwrap() - 1)
        .filter(|&index| index < num_nodes)
        .map(|index| child_values[index])
        .sum()
}
