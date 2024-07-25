fn main() {
  let mut s1 = String::from("hello");

  let len = calculate_length(&s1); // pass a reference of s1 so we dont move ownership of s1 to the function

  println!("The length of {s1} is {len}."); // s1 is still valid here because we have the ownership of s1
  change(&mut s1); // pass a mutable reference of s1 so we can change the value of s1
  println!("The new value of s1 is {s1}"); // s1 is still valid here because we have the ownership of s1
  // let _inR1 = &s1; // we can have multiple immutable references to a particular piece of data
  // if previously we have an immutable reference to a piece of data, we cannot have a mutable reference to that piece of data
  let _r1 = &mut s1; // we can have only one mutable reference to a particular piece of data in a particular scope at a time
  // let r2 = &mut s1; // this will cause an error because we already have a mutable reference to s1

  let s = no_dangle(); // this will cause an error because the reference to s will be dropped after the function dangle() is called
  println!("{}", s);
}

fn no_dangle() -> String /*&String*/ {
  let s = String::from("hello");
  // &s // return a reference to s is not allowed because s will be dropped after the function dangle() is called
  s // return s is allowed because s will be moved to the calling function
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String so the function does not take ownership of s
  s.len()
}

fn change(some_string: &mut String) { // mutable reference (we can change the value of the reference)
  some_string.push_str(", world"); // we can change the value of the reference
}