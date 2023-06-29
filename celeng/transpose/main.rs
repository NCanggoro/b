fn transpose(x: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut temp: Vec<Vec<u32>> = vec!();
    for i in 0..x[0].len() {
        let mut temp_vec: Vec<u32> = vec!();
        for j in 0..x.len()  {
            temp_vec.push(x[j][i]);
        }
        temp.push(temp_vec);
    }
    temp
}

fn main() {
    let arr1 = vec!( vec!(1,2,3), vec!(4,5,6), vec!(7,8,9));
    let arr2 = vec!( vec!(1,2), vec!(3,4), vec!(5,6), vec!(7,8));
    println!("{:?}",transpose(arr1));
    println!("{:?}",transpose(arr2));
}
