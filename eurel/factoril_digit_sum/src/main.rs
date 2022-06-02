use num::BigUint;

// okay TIL about BigUint tbh
// https://stackoverflow.com/questions/49355144/cannot-store-large-values-when-calculating-factorials
fn factorial(number: BigUint) -> BigUint {
    let big_1 = 1u32.into();
    let big_2 = 2u32.into();
    if number < big_2 {
        big_1
    } else {
        let prev_factorial = factorial(number.clone() - big_1);
        number * prev_factorial
    }
}

fn main () {
    let num = 100u32.into();
    
    let temp = format!("{}",factorial(num));
    println!("{}", temp);
    let v = temp
        .into_bytes()
        .into_iter()
        .map( |b| b as i32 - 48 ) // - 48 for bytes hack
        .collect::<Vec<i32>>(); 
    let res: i32 = v.iter().sum();
    println!("{:?}", res);
}