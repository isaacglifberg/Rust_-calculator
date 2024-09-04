use std::io;
use std::process; 



fn main(){

    println!("Welcome to the simple Rust calculator!");

    loop {
        println!("Enter the first number:");
        let num1 = read_number();

        println!("Enter an operation (+, -, *, /):");
        let operator = read_operator();

        println!("Enter the second number:");
        let num2 = read_number();

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => {
                if num2 == 0.0 {
                    println!("Error: Division by zero");
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("Error: Invalid operator");
                continue;
            }
            
        };
    println!("The result is: {}", result);

    println!("Would you like to perform another calculation? (y/n):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    if input.trim().to_lowercase() != "y" {
        println!("Exiting, goodbye!!");
        break;
    }

    }
    
}

fn read_number() -> f64{
    loop {

        let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => return num,
        Err(_) => {
            println!("Error: Please enter a valid number.");
        }
    }
}
}

fn read_operator() -> char {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().chars().next() {
            Some(op) => return op, 
            None => {
                println!("Error: Please enter a valid operator."); 
            }
        }
    }
}