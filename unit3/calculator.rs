fn main() {
    let a = 11;
    let b = 30;
    let op = '*';
    match op{
        '+' => print!("{}",a+b),
        '-' => print!("{}",a-b),
        '*' => print!("{}",a*b),
        '/' => print!("{}",a/b),
        '%' => print!("{}",a%b),
        _ => println!("Unsupported operator"),
    }
}
