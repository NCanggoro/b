fn main() {
  use std::env::args;
  use std::fs::read_to_string;
  use std::time::{Duration, Instant};

  let input_files = 
    args()
    .skip(1)
    .next()
    .expect("There's no Input File");

  let input = read_to_string(&input_files).unwrap();

  let processed_input = input
                .split("\n")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

  // println!("{:?}", &processed_input);
  // println!("input: {}", &input_files);

  // calculate how long to run the function
  // let start = Instant::now();
  println!("res: {}", find_sum(&processed_input));
  // let duration = start.elapsed();
  // println!("Time Elapsed {:?}", duration);
}

fn find_sum(x: &Vec<i32>) -> i32 {
  let n = x.len();
  for i in 0..n -1 {
    for j in i+ 1..n {
      if x[i] + x[j] == 2020 {
        return x[i] * x[j];
      }
    }
  }

  unreachable!();
}