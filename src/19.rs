use std::vec::Vec;

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("The original vector:");
    for num in &numbers {
        println!("{}", *num);
    }
}
