fn main() {
    let mut vec = vec![];
    let mut res: usize = 0;

    for i in 0..50 {
        if vec.len() <= 2 {
            vec.push(i);
        } else {
            let temp = vec[i - 1] + vec[i - 2];
            if temp > 4000000 {
                break;
            } else {
                vec.push(vec[i - 1] + vec[i - 2]);
            }
        }
    }

    for i in &vec {
        if i % 2 == 0 {
            res += i;
        }
    }

    println!("{}", res);
}
