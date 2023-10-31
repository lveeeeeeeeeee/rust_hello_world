use rand::Rng;
use std::cmp::Ordering;
use std::{io, num::ParseIntError};

fn main() {
    println!("Guessing game.");
    println!(
        "be grateful. youve been given a leeway of 12 moves.
    \nif you lose you get segfaulted (joke)"
    );

    let error_msg: &str = "32 bits number put fam";
    let secret_num: i32 = generate_rnd_num();

    let mut tries: u8 = 12;

    while tries != 0 {
        println!("Enter the number from -1000 to 1000.");

        let guess: i32 = input_number().expect(&error_msg);
        tries -= 1u8;

        let correct_guess: bool = compare_secret_to_guess(&secret_num, &guess);
        if correct_guess {
            println!("you win!");
            return;
        } else {
            println!("{tries} tries remain.");
        }
    }

    println!("you couldnt get {secret_num}!! FOOL!");
    panic!("you LOSE!!!!!!!!!!!");
}

fn compare_secret_to_guess(secret: &i32, guess: &i32) -> bool {
    match guess.cmp(secret) {
        Ordering::Less => {
            println!("Secret is greater.");
            return false;
        }
        Ordering::Greater => {
            println!("Secret is lesser.");
            return false;
        }
        Ordering::Equal => return true,
    }
}

fn generate_rnd_num() -> i32 {
    rand::thread_rng().gen_range(-1000..=1000)
}

fn input_number() -> Result<i32, ParseIntError> {
    let mut number: String = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("system eror");

    let result: Result<i32, ParseIntError> = number.trim().parse::<i32>();

    result
}
