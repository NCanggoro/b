enum Direction {
    Right,
    Down,
    Left,
    Up,
}
use Direction::*;

fn main() {
    #[rustfmt::skip]
    let input = vec![
        [ 1,  2,  3,  4,  5,  6],
        [20, 21, 22, 23, 24,  7],
        [19, 32, 33, 34, 25,  8],
        [18, 31, 36, 35, 26,  9],
        [17, 30, 29, 28, 27, 10],
        [16, 15, 14, 13, 12, 11],
    ];

    let mut output: Vec<i32> = Vec::new();
    let mut rows = std::collections::VecDeque::from_iter(input.iter().map(|row| row.iter()));
    let mut dir = Right;
    loop {
        dir = match dir {
            Right => match rows.pop_front() {
                Some(top) => {
                    output.extend(top);
                    Down
                }
                None => break,
            },
            Down => {
                output.extend(rows.iter_mut().flat_map(|row| row.next_back()));
                Left
            }
            Left => match rows.pop_back() {
                Some(bottom) => {
                    output.extend(bottom.rev());
                    Up
                }
                None => break,
            },
            Up => {
                output.extend(rows.iter_mut().rev().flat_map(|row| row.next()));
                Right
            }
        }
    }

    println!("{:?}", output);
    assert_eq!(output, Vec::from_iter(1..=36));
}
