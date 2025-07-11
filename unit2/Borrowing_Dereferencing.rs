fn main() {
    let x = 10;
    let y = &x;  // Borrowing: y is a reference to x

    println!("Value of x: {}", x);
    println!("Value of y (reference to x): {}", y);
    
    // Dereferencing y to get the value of x
    println!("Dereferenced y: {}", *y);

 
}
