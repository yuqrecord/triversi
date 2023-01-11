// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>

use triversi::board::BlockBoard;

fn main() {
    let range = 18;
    let distance = 3;
    let block_board = BlockBoard::try_new(range, distance).unwrap();
    for row in block_board.block_board().iter() {
        for c in row.iter() {
            let c: char = (*c).into();
            print!("{}", c);
        }
        println!();
    }
}
