use std::io::{Write};
fn main(){
    let first_edge: f64;
    let second_edge: f64;

    print!("Plase give the first edge: ");
    std::io::stdout().flush().unwrap();
    first_edge = get_value();
    print!("Plase give the second edge: ");
    std::io::stdout().flush().unwrap();
    second_edge = get_value();

    let hipo = (first_edge.powi(2)+second_edge.powi(2)).sqrt();

    println!("Hipo: {}", hipo);
}
fn get_value()->f64{
    let mut input:String=String::new();
    std::io::stdin().read_line(&mut input).expect("Reading Err");
    input.trim().parse().expect("Not Number")
}