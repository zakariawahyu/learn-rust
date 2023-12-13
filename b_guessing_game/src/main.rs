use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guset the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number {}", secret_number);

    loop {
        // create mutable variable
        let mut guess =  String::new();

        // catch the user input
        io::stdin()
            .read_line(&mut guess) // return enum
            .expect("Failed to read line");

        println!("Your guessed {}", guess);

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){ // arm
            Ordering::Less => println!("To small !!"),
            Ordering::Greater => println!("To big !!"),
            Ordering::Equal => {
                println!("You winn !!");
                break
            }
        }
    }
}
