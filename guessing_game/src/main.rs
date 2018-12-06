extern crate rand;

use std::io; //standard input/output
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101); // generate random number each time program is run
    println!("Guess the number!");
    println!("The secret number is: {}", secret_number);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("Please input your guess.");
   
    //let mut guess = String::new(); //mutable
    let guess: u32 = guess.trim().parse()
	.expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
	Ordering::Less => println!("Too small"),
	Ordering::Greater => println!("Too big!"),
	Ordering::Equal => println!("You win!"),
    }
}
