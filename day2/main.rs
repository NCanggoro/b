fn is_valid(mut x: &str) -> bool {
	let mut vec = x
					.split_whitespace()
					.collect::<Vec<&str>>();
  let temp = vec[0]
          .split("-")
          .map(|t| t.parse::<usize>().unwrap())
          .collect::<Vec<_>>();
  let temp2 = vec[1]
          .replace(":", "");
  let matched = vec[2].matches(&temp2).count();
	if(&matched >= &temp[0] && &matched <= &temp[1]) {
		return true
	} else {
		return false;
	}
}

// fn is_valid_2()

fn main() {
  use std::env::args;
	use std::fs::read_to_string;
	
	let mut x = 0;
  let input_files = 
              args()
              .skip(1)
              .next()
              .expect("There's no Input File");

  let input = read_to_string(&input_files).unwrap();

  let processed_input = input
								.split("\n")
								.collect::<Vec<_>>();

  // let test_pass = "1-3 a: abcdefg";
	// println!("{:?}", &processed_input);
	let arr = &processed_input.len();

	for i in 0..*arr{
		if (is_valid(&processed_input[i])) {
			x += 1
		} 
	}
	println!("{}", x);

} 