use std::io;
fn main() {
    println!("Welcome to my guessing game.");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess);
    println!("{}",guess);
}
