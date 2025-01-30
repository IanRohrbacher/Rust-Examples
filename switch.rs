fn main() {
    let x = 3;
    match x {
        1 => println!("X is 1"),
        2..5 => println!("X is within the range 2-5"),
        _ => println!("X is greater that 5")
    }
}