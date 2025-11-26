use std::io::{self, Write}; // Import the io module

fn main() {
    println!("--- Triangle Area Calculator ---");

    // Get values by calling the helper function
    let base = get_input("Enter the base length: ");
    let height = get_input("Enter the height: ");

    // Calculation (using f64 allows for decimal results)
    let area = (base * height) / 2.0;

    println!("\nResult:");
    println!("Triangle Area: {:.2} square units.", area);
}

// Function to handle user input
// This function loops until the user enters a valid number.
fn get_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Flush stdout to display the prompt immediately

        let mut input = String::new();

        // Read from keyboard (stdin)
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse String to f64 number
        match input.trim().parse::<f64>() {
            Ok(number) => return number, // Return the number if successful
            Err(_) => println!("Error: Please enter a valid number!"), // Warn user and restart loop
        }
    }
}