fn main() {
    let x = 5;
    let x = x + 1; // shadowing the previous `x`
    let x = x * 2;

    println!("The value of x is: {}", x); // Outputs: 12
}
