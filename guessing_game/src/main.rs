use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Wel come to my guessing game.");
    loop {
    println!("Guess a number between 1 and 100.");
    let number =rand::thread_rng().gen_range(0,101);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Input not gotten.");
    println!("Your input value is {}",guess);
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    match guess.cmp(&number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
}
}
