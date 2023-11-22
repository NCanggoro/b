fn main() {
    let mut res = 0i64;

    for i in 100..1000 {
        for j in 100..1000 {
            let temp = i * j;
            let rev = temp
              .to_string()
              .chars()
              .rev()
              .collect::<String>()
              .parse::<i64>()
              .unwrap();
            if rev == temp {
                if res < temp {
                    res = temp;
                }
            }
        }
    }
    
    println!("{}", res);
}
