fn main() {
    let x: i32 = 42;
    let y: f64 = x as f64;  // Cast i32 to f64
    
    let a: u8 = 255;
    let b: i16 = a as i16;  // Cast u8 to i16
    
    let c: f64 = 3.14;
    let d: u8 = c as u8;    // Cast f64 to u8 (fractional part truncated)

    println!("Integer x: {}", x);
    println!("Cast x to f64: {}", y);

    println!("Unsigned 8-bit a: {}", a);
    println!("Cast a to signed 16-bit: {}", b);

    println!("Floating point c: {}", c);
    println!("Cast c to unsigned 8-bit (truncated): {}", d);
}
