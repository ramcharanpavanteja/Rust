fn main(){
  let score = 80;

    if score > 50 {
      let result = "Pass";
      println!("Result: {}", result);
    }

println!("Result: {}", result); // error : not found in this scope 
}
