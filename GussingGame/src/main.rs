extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the Number! (1~100)");

    loop {
        println!("Please input the number:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Fail to read line");

        let guess: u32 = guess.trim().parse()
            .ok()
            .expect("Please type a number");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
