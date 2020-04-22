use std::io;

fn main() {
    let mut number = String::new();
    
    println!("Enter number:");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u8 = number.trim().parse().expect("Could not transform to number.");

    println!("number = {}", number);
}
