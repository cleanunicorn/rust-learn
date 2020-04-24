fn say_hello() {
    let s = String::from("Hello, world!");

    {
        let s = String::from("Hello, Dave!");
        println!("conditional s = {}", s);
        drop(s);
    };

    println!("s = {}", s);
}

fn move_string() {
    let a = String::from("world");
    let b = a;

    // `a` does not exist anymore. It was moved to `b`
    // println!("a = {}", a);
    println!("b = {}", b);
}

fn move_integer() {
    let a = 5;
    let b = a;

    println!("a = {}", a);
    println!("b = {}", b);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn ownership_functions() {
    let s = String::from("Hello, Dave! You're looking well today");

    take_ownership(s);

    // `s` does not exist anymore. It was moved to `take_ownership()`
    println!("s = {}", s);
}

fn main() {
    say_hello();
    move_string();
    move_integer();

    ownership_functions();
}