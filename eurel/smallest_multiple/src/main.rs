fn main() {
    let mut num = 1u128;
    loop {
        let mut temp = Vec::new();
        for i in 1..20 {
            temp.push(num%i);
        }
        let temp_max = temp.iter().max().unwrap();
        if *temp_max == 0 {
            break;
        } else {
            temp.clear();
        }
        num += 1;
    }   
    
    println!("{}", num)
}