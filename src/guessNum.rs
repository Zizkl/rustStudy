use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() { 
    println!("Guess the number（1 ～ 100）!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // let guess: u32 = guess.trim().parse().expect("Please try a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("You guessed: {} Too small!", guess),
            Ordering::Greater => println!("You guessed: {} Too big!", guess),
            Ordering::Equal => {
                println!("The true number is : {} you win!!!", guess);
                break;
            }
        } 
    }
}