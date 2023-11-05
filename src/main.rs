// TODO
// - interval of guess is fixed. change it to be user-generated. DONE
// - number of tries is fixed. change it to be log2(elements_to_guess_from) + 1. DONE
// - program panics. explode the computer instead.

use rand::Rng;
use rand::rngs::ThreadRng;
use std::cmp::Ordering;
use std::{io, num::ParseIntError};
use std::ops::RangeInclusive;

fn main() {
    println!("Guessing game.");

    const ERROR_MSG: &str = "32 bits number put fam";

    println!("enter lowest bound of interval of i32 numbers to guess from.");
    let start = input_number().expect(&ERROR_MSG);

    println!("enter highest bound of interval of i32 numbers to guess from.");
    let end = input_number().expect(&ERROR_MSG);

    let secret_num: i32 = generate_rnd_num(&start, &end);    
    let mut tries: u32 = calculate_tries(&start, &end) + 1;

    println!(
        "be grateful. youve been given a leeway of {tries} moves.
        if you lose you get segfaulted (joke)\n"
    );

    // While there are still tries, take a number and compare it to a secret one.
    while tries != 0 {
        println!("Enter the number from {start} to {end}.");

        // Get the user number.
        let guess: i32 = input_number().expect(&ERROR_MSG);
        tries -= 1u32;

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
fn generate_rnd_num(start: &i32, end: &i32) -> i32 {
     
    // Create a range struct
    let random_range: RangeInclusive<i32> = RangeInclusive::new(*start, *end);
    let mut rng: ThreadRng = rand::thread_rng();

    return rng.gen_range(random_range);
}


// ceil(log2(6)) = 3
fn calculate_tries(start: &i32, end: &i32) -> u32 {
    const ERROR_MSG_MISMATCH: &str = "";
    loop {
        let len: i32 = *end - *start;
        
        if len <= 1 {
            println!("the range was inputted wrong: 1st argument should be less than second by more than 1");
        }
        
        else {
            let tries: u32 = len.wrapping_abs() as u32;
            return u32::BITS - tries.leading_zeros();
        }

        println!("enter lowest bound of interval of i32 numbers to guess from.");
        let start: i32 = input_number().expect(&ERROR_MSG_MISMATCH);

        println!("enter highest bound of interval of i32 numbers to guess from.");
        let end: i32 = input_number().expect(&ERROR_MSG_MISMATCH);
    }
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
