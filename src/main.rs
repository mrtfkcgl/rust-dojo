use std::io::Write;

fn main(){
    let number:i32;
    print!("Number: ");
    std::io::stdout().flush().unwrap();

//     number = get_number();

 for i in 1..=1096 {
     if is_armstrong(i) {
         println!("{} is an Armstrong number!", i);
     } else {
        // println!("{} is NOT an Armstrong number!", i);
     }
 }
}

fn get_number()->i32{
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).expect("Reading err");
    // Negatif girerse hata vermesin diye unwrap yerine match veya basit parse
    match s.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

fn is_armstrong(original_number: i32)->bool{
    if original_number<0{
        return false;
    }

    let digit_count = original_number.to_string().len() as u32;

    let mut sum:i32 = 0;
    let mut temp_number:i32 = original_number;
    while temp_number>0 {
        let digit = temp_number % 10; //en son bas.
        sum += digit.pow(digit_count);
        temp_number/=10;
    }
    sum == original_number



}