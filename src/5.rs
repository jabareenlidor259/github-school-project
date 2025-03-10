use std::fs;
fn main() {
    let filename = "hello.txt";
    let contents = fs::read_to_string(filename).unwrap();
    println!("{}", contents);
}
