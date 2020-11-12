use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod leetcode;


fn main() {
    println!("Guess Game v0.1");
    
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

pub fn num_teams(rating: Vec<i32>) -> i32 {
    let mut result:i32 = 0;

    for i in 0..rating.len() {
        for j in i..rating.len() {
            for z in j..rating.len() {
                if rating[z] > rating[j] && rating[j] > rating[i] {
                    result= result + 1;
                }
                if rating[z] < rating[j] && rating[j] < rating[i] {
                    result= result + 1;
                } 
            }
        }
    }

    return result;
}