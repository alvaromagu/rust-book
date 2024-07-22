use std::io;

fn main () {
  // integers table
  // Length  Signed  Unsigned
  // 8-bit   i8      u8
  // 16-bit  i16     u16
  // 32-bit  i32     u32
  // 64-bit  i64     u64
  // 128-bit i128    u128
  // arch    isize   usize
  // example of signed integer
  let signed_integer = -10; // by default all integers are inferred as i32
  println!("Signed integer: {signed_integer}");
  // example of unsigned integer
  let unsigned_integer = 10u32; // we can add suffix to specify the type or specify the type explicitly
  // let unsigned_integer: u32 = 10; // example of specifying the type explicitly
  println!("Unsigned integer: {unsigned_integer}");

  // two types of floating point numbers
  // f32 and f64
  let float = 10.0; // by default all floating point numbers are inferred as f64
  println!("Floating point number: {float}");
  let float = 10.0f32; // we can add suffix to specify the type or specify the type explicitly
  // let float: f32 = 10.0; // example of specifying the type explicitly
  println!("Floating point number: {float}");

  // numeric operations
  // addition
  let sum = 5 + 10;
  // subtraction
  let difference = 95.5 - 4.3;
  // multiplication
  let product = 4 * 30;
  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1
  // remainder
  let remainder = 43 % 5;
  println!("Numeric operations: sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}");

  // boolean type
  let t = true;
  let f: bool = false; // example of specifying the type explicitly
  println!("Boolean: {}, {}", t, f);

  // character type
  let c = 'z'; // use single quotes for character type
  let z: char = 'ℤ'; // example of specifying the type explicitly
  let heart_eyed_cat = '😻'; // char is has 4 bytes of size and represents a Unicode Scalar Value
  println!("Character: {c}, {z}, {heart_eyed_cat}");

  // compound types
  // tuple type, elements can have different types
  let tup: (i32, f64, u8) = (500, 6.4, 1); // type annotation is optional
  let (x, y, z) = tup; // destructuring
  println!("Tuple [x]: {x}, [y]: {y}, [z]: {z}");
  // access tuple elements by index
  println!("Tuple by index [x]: {}, [y]: {}, [z]: {}", tup.0, tup.1, tup.2);

  // array type, elements must have the same type
  let arr: [i32; 5] = [1, 2, 3, 4, 5]; // type annotation is optional, you must specify the length of the array
  let first = arr[0]; // access array elements by index
  let second = arr[1];
  println!("Array [first]: {first}, [second]: {second}");

  let arr_of_3s= [3; 3]; // this syntax creates an array of 3 elements with the value 3
  println!("Arr of 3s: {}, {}, {}", arr_of_3s[0], arr_of_3s[1], arr_of_3s[2]);

  // index_out_of_bounds();
}

#[allow(dead_code)]
fn index_out_of_bounds () {
  let arr = [1, 2, 3, 4, 5];
  println!("Please enter an array index.");
  loop {
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    match index.trim().parse::<usize>() {
      Ok(num) => {
        // check if the index is within the bounds of the array
        // if we don't check this, the program will panic
        // an error similar to: thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10'
        // will be thrown
        if num < arr.len() {
          let element = arr[num];
          println!("The value of the element at index {} is: {}", num, element);
          break;
        }
      },
      Err(_) => {}
    };
    println!("Please enter a valid index.");
  }
}
