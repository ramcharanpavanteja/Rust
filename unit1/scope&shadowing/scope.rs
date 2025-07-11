fn main(){
  fn msg(){
  let m = "hello";
  println!("{}",m);
}
msg();
println!("{}",m); //not found in scope error
}
