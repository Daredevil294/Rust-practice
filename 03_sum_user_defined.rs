use std::io;
fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    println!("Enter the first number: ");
    io::stdin().read_line(&mut num1).unwrap();
    println!("Enter the second number:");
    io::stdin().read_line(&mut num2).unwrap();
    let a: i32 = num1.trim().parse().unwrap();
    let b: i32 = num2.trim().parse().unwrap();
    let sum = a + b;
    println!("Result = {}", sum)
}
