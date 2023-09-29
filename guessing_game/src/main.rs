extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the no!");

    let secret_no = rand::thread_rng().gen_range(1,100);
    println!("secret no : {}", secret_no);

    println!("input your guess");

    loop{
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
                        Ok(num) => num,
                        Err(_) => continue,
        };

        println!("you guessed this : {}", guess);

        match guess.cmp(&secret_no){
            Ordering::Less=> println!("less than no"),
            Ordering::Greater => println!("greater than no"),
            Ordering::Equal => {
                println!("matched!!");
                break;
            }
        }
    }
}
