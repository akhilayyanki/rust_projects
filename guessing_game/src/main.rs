use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess a number !");
    

    let rand_num = rand::thread_rng().gen_range(1..=100);


    loop{ 

        let mut guess = String::new();
        println!("Please input your guess");
        io::stdin().read_line(& mut guess).expect("Failed to read line");
        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) =>num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct !");
                break;
            },
        }
    }
    

}
