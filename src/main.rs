use std::io::Write;

fn main(){
    let mut value:i32;

    print!("Number: ");
    std::io::stdout().flush().unwrap();
    let mut digit:i32;
    let mut sum:i32 = 0i32;
    value = get_value();

    while value > 0 {

        digit = value % 10;
        sum += digit;
        value /= 10;

    }
    println!("Sum of elements: {}",sum);


}

fn get_value()->i32{
    let value:i32;
    let mut s:String = String::new();
    std::io::stdin().read_line(&mut s).expect("Readign err");
    value = s.trim().parse().expect("Parsing err");
    value
}