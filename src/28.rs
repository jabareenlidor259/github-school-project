use std::cmp;

fn main() {
    let numbers: Vec<isize> = vec![1, 2, 3, 4, 5];
    if let Some(lowest) = numbers.iter().min_by_key(|&x| x) {
        println!("The lowest number is {}", lowest);
    } else {
        eprintln!("There are no elements in the list.");
    }
}
