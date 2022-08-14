fn main() {
    let mut input: Vec<i32> = vec!(1,2,3,4,5);
    let res = input
        .iter()
        .enumerate()
        .filter(|(i, val)| i % 2 == 0)
        .map(|(_, val)| val)
        .fold(0, |acc, x| acc + x);
    println!("{}", res);
}

