use std::io::Write;

fn main(){
    let Number:i32;
    print!("Please enter a positive integer to check: ");
    std::io::stdout().flush().unwrap();
    Number = get_number();

    if(is_perfect(Number)){
        println!("Yes Perct Number");
    }else {  println!("NOT a 2 Perct Number");}

}

fn get_number()->i32{
    let mut s:String = String::new();
    std::io::stdin().read_line(&mut s).expect("Reading err");
    match s.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }

}

fn is_perfect(number:i32) -> bool {
    let mut sum:i32 = 0;
        for i in 1..=number/2 {

            if number % i == 0 {
                sum += i;
            }
        }
        sum == number

}