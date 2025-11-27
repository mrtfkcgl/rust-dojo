use std::io::Write;

fn main(){

    let input_Number:i32;

    print!("Lütfen bir sayı giriniz:");
    std::io::stdout().flush().unwrap();
    input_Number = get_val();


    for i in 1..=input_Number{
        if input_Number % i == 0{
            println!("{}",i);
        }
    }


}

fn get_val()->i32{
    let mut s:String = String::new();
    std::io::stdin().read_line(&mut s).expect("Reading err");
    s.trim().parse().expect("Not a number")
}