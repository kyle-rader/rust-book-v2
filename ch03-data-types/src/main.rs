extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Hello, Data Types!");

    let items = ["apple", "orange", "pear", "rubarb", "strawberry", "mango"];

    loop {
        let i = rand::thread_rng().gen_range(0, items.len());
        println!("You got {}!", items[i]);
        println!("Go again? (y/n)");

        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input!");
        match response.trim().to_lowercase().as_ref() {
            "n" => {
                println!("Goodbye fruit loop!");
                break;
            },
            _ => continue,
        }
    }
}
