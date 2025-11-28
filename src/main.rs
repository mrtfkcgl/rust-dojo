use std::io::Write;

fn main() {
    let mut Number:i32;
    let mut reversed_number:i32 = 0;

    print!("Number: ");
    std::io::stdout().flush().unwrap();
    Number = get_number();

    while Number > 0 {
        let digit = Number % 10;
        reversed_number = (reversed_number * 10) + digit;
        Number /= 10;
    };

    println!("Reversed Number: {}", reversed_number);

}
fn get_number() -> i32{
    let mut s:String = String::new();
    let number:i32;

    std::io::stdin().read_line(&mut s).expect("Reading err");
    number = s.trim().parse().expect("Parsing err");
    number

}