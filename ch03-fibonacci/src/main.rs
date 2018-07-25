use std::io;

fn main() {
    println!("Hello Fibonacci");

    println!("Please select which n-th Fibonacci number to find:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input from stdin!");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Oops, that's not a number, we will use 8 instead :)");
            8
        },
    };

    let suffix = num_suffix(n);

    println!("The {}{} Fibonacci number is: {}", n, suffix, fib(n));

}

fn num_suffix(x: u32) -> &'static str {
    let i = x % 10;
    let j = x % 100;

    if i == 1 && j != 11 {
        "st"
    } else if i == 2 && j != 12 {
        "nd"
    } else if i == 3 && j != 13 {
        "rd"
    } else {
        "th"
    }
}

fn fib(n: u32) -> u32 {
    fib_r(n, 0, 1)
}

fn fib_r(n: u32, p2: u32, p1: u32) -> u32 {
    match n {
        0 => p2,
        _ => fib_r(n-1, p1, p1+p2),
    }
}