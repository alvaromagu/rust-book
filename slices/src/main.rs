fn main() {
  let my_string = String::from("hello world");

  // `first_word` works on slices of `String`s, whether partial or whole
  let word = first_word(&my_string[0..6]);
  println!("{word}");
  let word = first_word(&my_string[..]);
  println!("{word}");
  // `first_word` also works on references to `String`s, which are equivalent
  // to whole slices of `String`s
  let word = first_word(&my_string);
  println!("{word}");

  let my_string_literal = "hello world";

  // `first_word` works on slices of string literals, whether partial or whole
  let word = first_word(&my_string_literal[0..6]);
  println!("{word}");
  let word = first_word(&my_string_literal[..]);
  println!("{word}");

  // Because string literals *are* string slices already,
  // this works too, without the slice syntax!
  let word = first_word(my_string_literal);
  println!("{word}");
}

fn first_word(s: &str) -> &str { // s: &str is a slice of a string (it accepts both String and string literals)
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i]; // return a slice of the string from the beginning to the index i
      }
  }

  &s[..] // return the whole string
}
