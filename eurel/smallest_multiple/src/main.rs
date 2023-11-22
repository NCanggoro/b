fn main() {
  let result = (1..)
  .find(|num| (1..=20).all(|i| num % i == 0))
  .unwrap();


  println!("{}", result);
}