use std::io;

fn main() {
    println!("Hello Guessing Ganme!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input!");

    println!("Your guess was: {}", guess);
}
