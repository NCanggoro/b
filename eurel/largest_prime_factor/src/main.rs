fn main() {
    let mut num     : i64       = 600851475143;
    let mut res     : Vec<i64>  = Vec::new();
    let mut divider : i64       = 2;

    while num > 1 {
        while num % divider == 0 {
            res.push(divider);
            num /= divider;
        }

        divider = divider + 1;
    }

    println!("{:?}", res);
}
