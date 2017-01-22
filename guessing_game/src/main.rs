//
// Rust Tutorial Guesssing Game
// https://doc.rust-lang.org/nightly/book/guessing-game.html
//

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
