use std::io;

fn main() {
    let mut input_1 = String::new();
    println!("Please enter your first number:");

    io::stdin()
        .read_line(&mut input_1)
        .expect("Failed to read line");

    let number_1: i32 = input_1
        .trim() // Remove whitespace
        .parse() // Convert to i32
        .expect("Please enter a valid number");

    let mut input_2 = String::new();
    println!("Please enter a second number:");

    io::stdin()
        .read_line(&mut input_2)
        .expect("Failed to read line");

    let number_2: i32 = input_2
        .trim() // Remove whitespace
        .parse() // Convert to i32
        .expect("Please enter a valid number");

    let sum = number_1 + number_2;
    let result = sum * sum;

    println!("Your squared result is : {}", result);
}
