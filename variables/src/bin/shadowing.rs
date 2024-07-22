fn main() {
  let x = 5; // x is a immutable variable, but we can shadow it
  let x = x + 1; // x overshadows the previous x, changing its value (by previous value [5] + 1) even though it is immutable
  {
    let x = x * 2; // in this inner scope, x overshadows the previous x, now x is 12
    println!("The value of x in the inner scope is: {x}");
  }
  println!("The value of x is: {x}"); // returning to the outer scope, x is still 6


  let spaces = "   ";
  let spaces = spaces.len(); // with shadowing, we can change the type of the variable
  // in other languages, this would require a new variable name, but in Rust, we can reuse the same name
  println!("The value of spaces is: {spaces}");
  // note that with this code will not compile becouse
  // rust is a statically typed language, so the type of the variable cannot be changed
  // let mut spaces = "   "; // mutable variable
  // spaces = spaces.len(); // but the type of the variable cannot be changed
  // we will get an error like this:
  // expected `&str`, found `usize`
}