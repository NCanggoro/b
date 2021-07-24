use std::env::args;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::read_to_string;

fn count_tree(input: &Vec<String>, r: usize, d: usize) -> usize {
    let line_c = input.len();
    let line_l = input[0].len();
    let mut trees = 0;

    let mut current = 0;
    let mut pos = 0;

    while current < line_c - d {
        current += d;
        pos = (pos + r) % line_l;
        let item = input[current].chars().nth(pos).unwrap();
        println!("{}", item);
        if(item == '#') {
            trees += 1;
        }
    }
    trees
}

fn main() {
    let tree: u32;
    let pos: u32;
    let input_files = 
              args()
              .skip(1)
              .next()
              .expect("There's no Input File");

    let open_file = Path::new(&input_files);
    let file = File::open(&open_file).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut str_lines: Vec<String> = vec![];
    for line in lines {
        if let Ok(line) = line {
            str_lines.push(line);
        }
    }
    

    // let input2 = read_to_string(&input_files).unwrap();

    // let pros_input2 = input2
    //                   .split("\n")
    //                   .collect::<Vec<str>>();
    
    // println!("{:?}", str_lines);

    println!("{}", count_tree(&str_lines, 3, 1));


//    let char_input: Vec<char> = input2
//                                .chars()
//                                .collect();



//    println!("{:?}", pros_input2);
//    toboggan_1(char_input, (3,1));
   
//    for c in char_input {
//        println!("{:?}", c);
//    }


}
