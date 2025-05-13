use std::io;
use chrono::Local;

fn main() {
    // Prompt the user for their name
    println!("What is your name?");

    // Read the user's input
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim();

    // Print a personalized greeting
    println!("Hello, {}! Welcome to the Rust Bootcamp!", name);

    // Print the current date
    let current_date = Local::now().format("%B %d, %Y");
    println!("Today is {}.", current_date);
}