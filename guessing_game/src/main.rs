use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Your secret number is {secret_number}");
    loop {
    println!("Input a number !");
    let mut guess = String::new();
    // let x = 2;
    // let x = 4;
    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line");
    println!("You guessed {guess}!");
    // println!("x is {x}")
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    match guess.cmp(&secret_number){
        Ordering::Less =>println!("{guess} is too small"),
        Ordering::Greater =>println!("{guess} is too large"),
        Ordering::Equal =>{println!("You guessed right"); break;}
    }
    }

}
