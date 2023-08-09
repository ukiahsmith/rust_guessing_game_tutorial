use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("===   Monday 07 August 2023   ===");
    println!();
    println!("Today I am studying: Rust: Guessing game again");
    println!();
    println!();

    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("shhhh, the secret number is: {secret_num} \n");

    // Why won't print!() output before the read_line waits for input? print!() does not emit
    // newline, which makes it buffered, and would require it to be flushed to see it before the
    // read_line waits.
    //
    // https://stackoverflow.com/questions/34993744/why-does-this-read-input-before-printing
    //
    // print!("Guess a number between 1 and 100: ");
    // io::stdout().flush();

    println!("Guess a number between 1 and 100: ");
    print!("number: ");
    let mut guess_count = 0;

    loop {
        if guess_count > 0 {
            print!("guess again: ");
        }
        guess_count += 1;

        io::stdout()
            .flush()
            .expect("err 584: Faild to flush output.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("err 3392: Failed to read line.");

        print!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Yikes, you didn't enter a number");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{guess} is too small\n"),
            Ordering::Greater => println!("{guess} is too BIG!\n"),
            Ordering::Equal => {
                println!("just right, You WIN!!");
                break;
            }
        }
    }
}
