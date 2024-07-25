fn main () {
  // with fixed size inmutables values nothing changes
  let x = 5;
  let y = x;
  println!("x: {x}, y: {y}"); // we can use both x and y
  // example ends here: x and y are dropped

  // with unknownsize values the ownership is transfered
  let s1 = String::from("hello");
  let s2 = s1;
  // println!("s1: {}", s1); // this will fail because s1 is no longer valid
  println!("s2: {s2}"); // this will work because s2 is the new owner (s1 was moved to s2)
  // example ends here: s2 is dropped

  // to avoid this we can use the clone method
  let s1 = String::from("hello"); // redefine s1 and s2 for the next example
  let s2 = s1.clone();
  println!("s1: {s1}, s2: {s2}"); // this will work because s1 is cloned
  // example ends here: s1 and s2 are dropped

  // owenership with functions
  let s = String::from("hello");
  takes_ownership(s); // s is moved to the function
  // println!("s: {}", s); // this will fail because s is no longer valid (moved to the function)
  // example ends here: s is dropped in the function

  // returning values from functions
  let s1 = gives_ownership(); // s1 is the owner of the value returned by the function
  let s2 = String::from("hello");
  let s3 = takes_and_gives_back(s2); // s2 is moved to the function and then returned to s3
  // println!("s2: {}", s2); // this will fail because s2 is no longer valid (moved to the function)
  println!("s1: {s1}, s3: {s3}"); // this will work because s1 and s3 are the owners of the values
  // example ends here: s1 and s3 are dropped

  // tuple example
  let s1 = String::from("hello");
  let (s1, len, ) = calculate_length(s1); // s1 is moved to the function and then returned to redefined s1
  // s1 is redefined because we lost the ownership of the original s1
  println!("s1: {s1}, len: {len}"); // this will work because s1 is the owner of the value returned by the function
  // example ends here: s1 and len are dropped
  
}

fn calculate_length(s1: String) -> (String, usize) {
  let len = s1.len(); // we have to calculate the length before returning the value because s1 will be moved
  (s1, len)
}

fn takes_and_gives_back(s2: String) -> String {
  s2
} // s2 is moved to the caller

fn gives_ownership() -> String {
  let s = String::from("hello");
  s
} // s is moved to the caller

fn takes_ownership (s: String) {
  println!("s: {s}");
} // s is dropped here