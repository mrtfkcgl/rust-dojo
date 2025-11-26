use std::io::{self, Write};
fn main(){
    println!("-Triangle Area Calculator (SAS Method)-");
    println!("Formula: 0.5 * side_a * side_b * sin(angle)");

    let side_a = get_input("Enter the first side lenght: ");
    let side_b = get_input("Enter the second side lenght: ");
    let angel_degress = get_input("Enter the angle between them (in degress): ");

    //Rust => expect Radians not degres.

    let angle_radians = angel_degress.to_radians();
    let area = area(side_a, side_b, angle_radians);
    println!("\nResult:");
    println!("Side A: {}", side_a);
    println!("Side B: {}", side_b);
    println!("Angle : {} degrees", angle_radians);
    println!("---------------------------------");
    println!("Calculated Area: {:.2} square units.", area);
}

fn area(side_a:f64,side_b:f64,angel_degress:f64)->f64{
    0.5*side_a*side_b*angel_degress.sin()
}
fn get_input(prompt: &str) -> f64{
    loop{
        println!("{}",prompt);
        std::io::stdout().flush().unwrap();
        let mut input:String = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input)");
        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("Error: Please enter a valid number!"),
        }


    }

}