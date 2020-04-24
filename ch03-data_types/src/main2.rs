// use std::io;

// fn parse_number() {
//     let mut number = String::new();

//     println!("Enter number:");

//     io::stdin()
//         .read_line(&mut number)
//         .expect("Failed to read line");

//     let number: u8 = number
//         .trim()
//         .parse()
//         .expect("Could not transform to number.");

//     println!("number = {}", number);
// }

fn characters() {
    let c = String::from("z");
    let emoji = '🚀';

    println!("{} {}", c, emoji);
}

fn tuples() {
    let x: (i32, f64, u8) = (500, 1.2, 123);

    let _y = 2;

    println!("{}", x.2);
}

fn arrays() {
    const ARR_LENGTH: usize = 500000;

    let a: [i32; ARR_LENGTH] = [1234; ARR_LENGTH];

    println!("{}", a[1]);
}

fn main() {
    arrays();
    tuples();
    characters();
    // parse_number();

    let arr = arrays;

    let mut i = 0;

    loop {
        i = i + 1;
        println!("{}", i)
    }

    arr();
}
