use std::env::args;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::read_to_string;

fn toboggan_1(x: &Vec<Vec<u8>>, s: (usize, usize)) -> usize {
    let (s_x, s_y) = s;
    let mut pos = 0;
    let mut count = 0;
    for row in x.iter().step_by(s_y) {
        if *row.get(pos % (row.len())).unwrap() == 1 {
            count += 1
        }
        pos += s_x;
    }
    count
}

fn parse_input(input_files: String) -> Vec<Vec<u8>> {
    input_files
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _   => panic!("Unexpected character!"),
                })
                .collect()
        })
        .collect()

}

fn count_tree(input: &Vec<&str>, r: usize, d: usize) -> usize {
    let line_c = input.len();
    println!("{:?} \n {}", input, line_c);
    let line_l = input[0].len();
    let mut trees = 0;

    let mut current = 0;
    let mut pos = 0;

    while current < line_c - d {
        current += d;
        pos = (pos + r) % line_l;
    }

    pos
}

fn main() {
    let tree: u32;
    let pos: u32;
    let input_files = 
              args()
              .skip(1)
              .next()
              .expect("There's no Input File");

    let input = parse_input(read_to_string(&input_files).unwrap());

    let open_file = Path::new(&input_files);
    let file = File::open(&open_file).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut str_lines: Vec<String> = vec![];
    for line in lines {
        if let Ok(line) = line {
            str_lines.push(line);
        }
    }
    
//    println!("{:?}",toboggan_1(&input, (3,1)));

    let input2 = read_to_string(&input_files).unwrap();

    let pros_input2 = input2
                      .split("\n")
                      .collect::<Vec<&str>>();

    count_tree(&pros_input2, 3, 1);


//    let char_input: Vec<char> = input2
//                                .chars()
//                                .collect();



//    println!("{:?}", pros_input2);
//    toboggan_1(char_input, (3,1));
   
//    for c in char_input {
//        println!("{:?}", c);
//    }


}
