fn main() {
  // reference to a string located in read-only memory
  let p: &'static str = "the quick brown fox jumps over the lazy dog";
  println!("Pangram: {}", p);
}
