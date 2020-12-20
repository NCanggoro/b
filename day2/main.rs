fn is_valid(limit: &Vec<usize>,chars: &str,pass: &str) -> bool {
	let matched = pass
				.matches(chars)
				.count();
	if(&matched >= &limit[0] && &matched <= &limit[1]) {
		return true;
	} else {
		return false;
	}
	unreachable!();
}

fn is_valid_2(limit: &Vec<usize>,chars: &str,pass: &str) -> bool {
	let mut total = 0;
	let pass_chars: Vec<char> = pass
									.chars()
									.collect();
	for i in 0..2  {
		if(limit[i] > pass_chars.len()) {
			if(chars.chars().nth(0).unwrap() == pass_chars[limit[i] -2 ]) {
				total += 1;
			}
		} else if(chars.chars().nth(0).unwrap() == pass_chars[limit[i] -1 ]) {
			total += 1;
		}
	}
	if(total == 1 ) {
		return true;
	} else {
		return false;
	}
	unreachable!();
}

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

	let arr = &processed_input.len();

	for i in 0..*arr{
		let mut vec = &processed_input[i]
						.split_whitespace()
						.collect::<Vec<&str>>();
		let temp = &vec[0]
						.split("-")
						.map(|t| t.parse::<usize>().unwrap())
						.collect::<Vec<_>>();
		let temp2 = &vec[1]
						.replace(":", "");
		if (is_valid_2(temp, temp2, vec[2])) {
			x += 1;
		} 
	}
	println!("{}", x);

} 