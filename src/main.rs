use std::io::Write;

fn main() {
    print!("Enter a number: ");
    std::io::stdout().flush().unwrap();

    // We make the variable 'mut' because its value will change at every step
    let mut number = get_number();
    let mut steps = 0;

    // RULE 1: We use an infinite 'loop' instead of 'while'.
    // RULE 2: We assign the result of the loop to a variable (total_steps).
    let total_steps = loop {
        // If the number reaches 1, break the loop and RETURN the step count.
        if number == 1 {
            break steps;
        }

        // RULE 3: We use 'match' instead of 'if'.
        // We update 'number' based on the result.
        number = match number % 2 {
            0 => number / 2,       // If even
            _ => number * 3 + 1,   // If odd (wildcard handles remainder 1)
        };

        // Increment the step and observe the progress
        steps += 1;
        println!("Step {}: Number -> {}", steps, number);
    };

    println!("--------------------------------");
    println!("Result: Total {} steps required to reach 1.", total_steps);
}

fn get_number() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Read error");
    // Result unwrap operation (Panics if error occurs)
    s.trim().parse().expect("Please enter a valid number")
}