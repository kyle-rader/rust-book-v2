fn main() {
    println!("Hello, Functions!");

    my_first_function();

    println!("2 to the power of 8 is: {}", pow(2, 8));
}

fn my_first_function() {
    println!("I come from a function :)");
}

fn pow(base: i32, power: u32) -> i32 {
    base.pow(power)
}