fn main () {
  another_function(five());
  print_labeled_measurement(10, 'm');
}

fn five () -> i32 {
  5
}

fn another_function (x: i32) {
  println!("The value of x is {x}");
}

fn print_labeled_measurement (value: i32, unit_label: char) {
  println!("The value is: {value}{unit_label}");
}