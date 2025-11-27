use std::io::Write;

fn main(){
    let first_side:f64;
    let second_side:f64;
    let angle_degree:f64;
    print!("Please enter the first side: ");
    std::io::stdout().flush().unwrap();
    first_side = get_value();
    print!("Please ente the second side: ");
    std::io::stdout().flush().unwrap();
    second_side = get_value();
    print!("Plaese enter the angele side:");
    std::io::stdout().flush().unwrap();
    angle_degree = get_value();
    // Calculation
    // Formula: 0.5 * a * b * sin(angle)
    // Important: We must convert the angle from degrees to radians!

    let area = 0.5*first_side*second_side*angle_degree.to_radians().sin();

    println!("Area of the triangle: {}", area);

}
fn get_value()->f64{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().expect("Not valid number")}