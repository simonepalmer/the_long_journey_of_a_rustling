use std::io::{self, Write};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..=10);

    let name = get_name();
    welcome_player(name);

    let your_guess = make_guess();
    affirm_guess(your_guess);

    if your_guess == random_number {
        println!("Holy smokes! You got it! it's indeed {random_number}!");
    }
    else {
        println!("Too bad! I was actually thinking of {random_number}!");
    }
}

fn get_name() -> String {
    print!("Please enter your name: ");
    flush_output();
    let name = take_input();
    name
}

fn welcome_player(name: String) {
    println!("Welcome {}!", name.trim());
    println!("I am going to think of a number and your are going to guess it!");
}

fn make_guess() -> u32 {
    print!("please make your guess: ");
    flush_output();
    let guess = take_input();

    let guess_as_int: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Oh no! You wasted your guess!");
            0
        }
    };
    guess_as_int
}

fn affirm_guess(your_guess: u32) {
    println!("You guessed {your_guess}!")
}

fn take_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read line");
    input
}

fn flush_output() {
    io::stdout().flush().expect("Failed to flush");
}
