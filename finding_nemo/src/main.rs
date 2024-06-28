// Made by Thomas Anwandter
use std::io;
use rand::Rng;
fn main() {
    let mut find = find_nemo();
    println!("{}", find);
}

fn find_nemo() -> String {
    let findings = [ // add more if you want :3
        "Nemo is me",
        "Nemo is in me",
        "I am finding Nemo"
    ];

    let which_string = rand::thread_rng().gen_range(0..3);
    let mut words = findings[which_string].split(' ');
    println!("{}", findings[which_string]);

    let mut which_word = String::new();
    io::stdin()
        .read_line(&mut which_word)
        .unwrap();

    let word_int: i32 = (which_word.trim().parse().unwrap());
    if (words.nth(word_int as usize).unwrap() == "Nemo") {
        return format!("You found nemo at: {}", word_int.to_string());
    }
    return String::from("Better luck next time!");

}