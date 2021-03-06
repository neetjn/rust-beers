fn main() {
  // reference to a string located in read-only memory
  let p: &'static str = "the quick brown fox jumps over the lazy dog";
  println!("Pangram: {}", p);
  // iterate over words
  // - can alternatively use `split_whitespace`
  for s in p.split(' ') {
    println!("> {}", s);
  }
  // mutable string
  let mut s = "";
  // concat string with another string
  // concat string with a character
  // regular concatenation
  s = concat!(s, 'a');
  println!("{}", s);
  // raw string with quotes
  let quotes = r#"Then I screamed, "Hello World""#;
  println!("{}", quotes);
  // multiline strings
  let multi = "
  this
  is
  a
  test
  ";
  // reverse string
  let p1: &str = "hello world";
  let p2: String = p1.chars().rev().collect();
  println!("{}", p2);
}
