// Rust playground example of an infinite loop with a counter variable
let mut counter = 0;

while true {
    if counter == 5 {
        break;
    }
    println!("{}", counter);
    counter += 1;
}
