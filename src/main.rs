use rand::Rng;
use std::io;

fn main() {
    println!("===   Monday 07 August 2023   ===");
    println!();
    println!("Today I am studying: Rust: Guessing game again");
    println!();
    println!();

    let secret_num = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    print!("Guess a number between 1 and 100\nnumber: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("err 3392: Failed to read line.");

    println!("You guessed: {guess}");

    // for i in 4..=11 {
    //     println!("i: {i}")
    // }

    guess.cmp(&secret_num)
}
