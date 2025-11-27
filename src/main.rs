use std::io::Write;
fn main()
{
    let side_a:f64;
    let side_b:f64;
    let side_c:f64;

    print!("Side a:");
    std::io::stdout().flush().unwrap();
    side_a = get_value();
    print!("Side b:");
    std::io::stdout().flush().unwrap();
    side_b = get_value();

    print!("Side c:");
    std::io::stdout().flush().unwrap();
    side_c = get_value();

    let  formulaU:f64 = (side_a+side_b+side_c)/(2 as f64);

    let areaFormula:f64 = (formulaU*((formulaU-side_a)*(formulaU-side_b)*(formulaU - side_c))).sqrt();
    print!("Area: {}",areaFormula);

}
fn get_value()->f64{
    let mut s:String = String::new();
    std::io::stdin().read_line(&mut s).expect("Input format error");
    s.trim().parse().expect("Not a number")
}