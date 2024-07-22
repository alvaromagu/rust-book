fn main() {
  let mut x = 5; // mut keyword makes the variable mutable
  // let x = 5; // without 'mut' keyword, the variable is immutable and cannot be reassigned
  println!("The value of x is: {x}");
  x = 6; // This is allowed because x is annotated with 'mut' keyword
  println!("The value of x is: {x}");
}