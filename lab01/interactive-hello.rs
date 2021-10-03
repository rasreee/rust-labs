use std::io::{self, BufRead};

fn main() {
    let mut a_word = String::new();

    println!("What's your name?\n");

    let stdin = io::stdin();
    stdin.lock().read_line(&mut a_word).unwrap();

    println!(
        "Hey, {} I just really wanted to say hello to you.\nI hope you have a wonderful day.",
        a_word,
    );
}
