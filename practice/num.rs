fn main() {
  let max: u32 = 1000;
  let mut sum: u32 = 0;
  for n in 1..max {
    if n % 3 == 0 || n % 5 == 0 {
      sum += n;
    }
  }
  println!("sum: {}", sum);
}
