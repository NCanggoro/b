fn find_sum(x: &Vec<i32>) -> i32 {
    let n = x.len();
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            //n²
            for k in i + 1..n {
                // n³
                if x[i] + x[j] + x[k] == 2020 {
                    return x[i] * x[j] * x[k];
                }
            }
        }
    }
    unreachable!();
}

// better answer i found from a website (faster)
fn find_sum_better_answer(x: &Vec<i32>) -> i32 {
    // https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable
    let mut xtemp = x.clone();
    let n = xtemp.len();
    xtemp.sort();
    for i in 0..n - 1 {
        if let Ok(j) = xtemp.binary_search(&(2020 - xtemp[i])) {
            if i != j {
                return xtemp[i] * xtemp[j];
            }
        }
    }
    unreachable!();
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    use std::time::{Duration, Instant};

    let input_files = args().skip(1).next().expect("There's no Input File");

    let input = read_to_string(&input_files).unwrap();

    let processed_input = input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // println!("{:?}", &processed_input);
    // println!("input: {}", &input_files);

    // calculate how long to run the function
    // let start = Instant::now();
    println!("res: {}", find_sum_better_answer(&processed_input));
    // let duration = start.elapsed();
    // println!("Time Elapsed {:?}", duration);
}
