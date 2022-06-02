use std::env::args;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::read_to_string;

fn count_trees(input: &Vec<String>, r: usize, d: usize) -> usize {
    let line_c = input.len();
    let line_l = input[0].len();
    let mut trees = 0;

    let mut current = 0;
    let mut pos = 0;

    while current < line_c - d {
        current += d;
        pos = (pos + r) % line_l;
        let item = input[current].chars().nth(pos).unwrap();
        // println!("{}", item);
        if(item == '#') {
            trees += 1;
        }
    }
    trees
}

fn main() {
    let input_files = 
              args()
              .skip(1)
              .next()
              .expect("There's no Input File");

    // let temp = read_to_string(&input_files).unwrap();

    // println!("{:?}", temp);

    // Better File reader instead of using read_to_string 
    // cause theres empty string at the last element of vec
    let open_file = Path::new(&input_files);
    let file = File::open(&open_file).unwrap();
    let lines = io::BufReader::new(file).lines();
    // println!("{:?}", lines);
    let mut str_lines: Vec<String> = vec![];
    for line in lines {
        // println!("{:?}", line);
        if let Ok(line) = line {
            str_lines.push(line);
        }
    }
    

    let input2 = read_to_string(&input_files).unwrap();


    // but still i dont understand why it produce an empty string
    let pros_input2: Vec<String> = input2
                                    .split("\n")
                                    .filter(|f| !f.is_empty())
                                    .map(|s| s.to_string())
                                    .collect();
    println!("{:?}", pros_input2);                                
    
    println!("{}", count_trees(&pros_input2, 3, 1));

}
