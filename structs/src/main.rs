#[derive(Debug)] // trait to print struct
#[allow(dead_code)] // ignore the warning about unused fields
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Color(i32, i32, i32); // tuple struct

#[derive(Debug)]
struct Empty; // unit-like struct

fn main() {
  let mut user = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1
  };
  println!("User: {:?}", user);
  user.email = String::from("anotheremail@example.com"); // update email because user is mutable
  println!("User email: {}", user.email);
  let user = build_user(String::from("email@example.com"), String::from("username"));
  println!("User: {:?}", user);
  // user.username = String::from("newusername"); // error: cannot assign to `user.username` because it is borrowed
  let user = increase_sign_in_count(user);
  println!("User: {:?}", user);

  let black = Color(0, 0, 0);
  println!("Black: {:?}", black);

  let empty = Empty;
  println!("Empty: {:?}", empty);
}

fn build_user(email: String, username: String) -> User {
  User {
    email, // field init shorthand
    username,
    active: true, // normal field init
    sign_in_count: 1
  }
}

fn increase_sign_in_count(user: User) -> User {
  User {
    sign_in_count: user.sign_in_count + 1,
    ..user // struct update syntax
  }
}
