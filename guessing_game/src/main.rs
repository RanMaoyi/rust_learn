use std::io;
use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!!!");

    println!("Please input your guess!");

    let screct_number = thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input value is invalid please input again");
                continue;
            }
        };

        println!("You guess number is : {}", guess);
        println!("Secret number is : {}", screct_number);

        match guess.cmp(&screct_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        }
    }
}
