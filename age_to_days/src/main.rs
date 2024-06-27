// Made by Thomas Anwandter
use std::io;

fn main() {
    guess_age();
}

fn guess_age() {
    println!("Enter your age: ");
    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input!");

    let age_int: i32 = age.trim().parse().unwrap();

    println!("You are roughly: {} days old!", age_int * 365);
}