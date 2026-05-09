use std::io;

//function to add two numbers
fn add(a: i32, b: i32) -> i32{
    return a + b;
}

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: i32 = input.trim().parse().expect("Please enter a number");
    input.clear(); //clean the input for reuse

    println!("Enter the second number: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2: i32 = input.trim().parse().expect("Please enter a number");

    let sum: i32 = add(num1, num2);

    println!("The sum of {} and {} is {}", num1, num2, sum);
}
 