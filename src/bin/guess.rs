use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("--Guess Game v0.1--");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The Secret Number is {}", secret_number);

    loop {
        println!("Input Your Number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // continue make the other logic do not run
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Equal");
                break;
            }
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less"),
        };
    }
}
