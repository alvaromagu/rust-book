#[allow(unreachable_code)]
fn main () {
  // if else
  let number = 3;
  if number < 5 { // if requires a bool expression
    println!("condition was true");
  } else { // else is optional
    println!("condition was false");
  }
  // if else if
  let number = 6;
  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 { // else if requires a bool expression and is optional
    println!("number is divisible by 3");
  } else if number % 2 == 0 { // we can have multiple else if blocks
    println!("number is divisible by 2");
  } else { // we can still have an optional else block
    println!("number is not divisible by 4, 3, or 2");
  }

  // if as an expression
  let condition = true;
  let number = if condition { // if can be used as an expression
    5 // the value of the block is the value of the expression
  } else {
    6 // both expressions must return the same type
  };
  println!("The value of number is: {}", number);

  // loop
  let mut counter = 0;

  let result = loop { // loop is an expression
    counter += 1;
    if counter == 10 {
      // normal loop only stops when break is called
      break counter * 2; // if loop is used as an expression, we must return a value
    }
  };
  println!("The result is {result}");

  // loop labels
  'outer: loop {
    println!("Entered the outer loop");
    loop {
      println!("Entered the inner loop");
      break 'outer; // break can be used to break out of a loop with a label
    }
    println!("This point will never be reached");
  }

  // while
  let mut number = 3;
  while number != 0 { // while requires a bool expression
    println!("{}", number);
    number -= 1;
  }
  println!("LIFTOFF!!!");

  // for
  let a = [10, 20, 30, 40, 50];
  for el in a {
    println!("the value is: {el}");
  }

  // for with range
  for number in (1..4).rev() { // rev() reverses the range
    println!("{number}!");
  }
  println!("LIFTOFF!!!");

  let fahrenheit = 32;
  println!("Fahrenheit [{fahrenheit}] -> Celsius [{}]", fahrenheit_to_celsius(fahrenheit, Deg::Fahrenheit));
  let celsius = 0;
  println!("Celsius [{celsius}] -> Fahrenheit [{}]", fahrenheit_to_celsius(celsius, Deg::Celsius));

  let n = 17;
  println!("fib [{n}]: {}", nth_fibonacci(n));

  twelve_days_of_christmas();
}

#[derive(PartialEq)]
enum Deg {
  Fahrenheit,
  Celsius
}

fn fahrenheit_to_celsius (val: i32, deg: Deg) -> i32 {
  if deg == Deg::Fahrenheit {
    (val - 32) * 5 / 9
  } else {
    9 * val / 5 + 32
  }
}

fn nth_fibonacci (n: i64) -> i64 {
  let mut res = 1;
  if n < 2 {
    return res
  }
  let mut prev = res;
  for _ in 2..n {
    let temp = res;
    res += prev;
    prev = temp;
  }
  res
}

fn twelve_days_of_christmas () {
  for day in 1..=12 {
    let ordinal = ordinal(day);
    println!("On the {ordinal} day of Christmas my true love gave to me");
    for gift in (1..=day).rev() {
      println!("{}", gift_str(day, gift))
    }
  }
}

fn ordinal (n: i32) -> &'static str {
  match n {
    12 => "twelfth",
    11 => "eleventh",
    10 => "tenth",
    9 => "ninth",
    8 => "eighth",
    7 => "seventh",
    6 => "sixth",
    5 => "fifth",
    4 => "fourth",
    3 => "third",
    2 => "second",
    1 => "first",
    _ => "unknown"
  }
}

fn gift_str (day: i32, gift: i32) -> String {
  match gift {
    1 => format!("{} partridge in a pear tree.", if day == 1 { "A" } else { "And a" }),
    2 => String::from("Two turtle doves,"),
    3 => String::from("Three French hens,"),
    4 => String::from("Four calling birds,"),
    5 => String::from("Five golden rings,"),
    6 => String::from("Six geese a-laying,"),
    7 => String::from("Seven swans a-swimming,"),
    8 => String::from("Eight maids a-milking,"),
    9 => String::from("Nine ladies dancing,"),
    10 => String::from("Ten lords a-leaping,"),
    11 => String::from("Eleven pipers piping,"),
    12 => String::from("Twelve drummers drumming,"),
    _ => String::from("unknown")
  }
}
