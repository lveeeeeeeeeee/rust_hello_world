// TODO
// - interval of guess is fixed. change it to be user-generated.
// - number of tries is fixed. change it to be log2(elements_to_guess_from) + 1.

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

    // While there are still tries, take a number and compare it to a secret one.
    while tries != 0 {
        println!("Enter the number from -1000 to 1000.");

        // Get the user number.
        let guess: i32 = input_number().expect(&error_msg);
        tries -= 1u8;

        // Compare the secret number and the guessed number
        let correct_guess: bool = compare_secret_to_guess(&secret_num, &guess);

        // If guess is correct, close the program
        if correct_guess {
            println!("you win!");
            return;
        } 
        // Continue looping.
        else {
            println!("{tries} tries remain.");
        }
    }

    // no more tries. panic.
    println!("you couldnt get {secret_num}!! FOOL!");
    panic!("you LOSE!!!!!!!!!!!");
}

/// Compare secret generated number to a guessed one.
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

/// Generate random number to guess.
fn generate_rnd_num() -> i32 {
    rand::thread_rng().gen_range(-1000..=1000)
}

/// Input number
/// returns: i32 number if parsing is successful, otherwise ParseIntError.
fn input_number() -> Result<i32, ParseIntError> {
    let mut number: String = String::new();

    // Get inputted string.
    io::stdin()
        .read_line(&mut number)
        .expect("system eror");

    // Try to extract number from the string.
    let result: Result<i32, ParseIntError> = number.trim().parse::<i32>();

    // return result.
    result
}
