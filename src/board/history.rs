// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::{Board, Player};
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Getters, Serialize, Deserialize)]
pub struct Record {
    range: usize,
    #[getset(get = "pub")]
    player_positions: Vec<(Player, (usize, usize))>,
}

#[derive(Clone, Debug, CopyGetters, Getters)]
pub struct History {
    #[getset(get_copy = "pub")]
    current_turn: usize,
    #[getset(get = "pub")]
    record: Record,
    boards: Vec<Board>,
}

impl Record {
    pub fn new(range: usize) -> Self {
        Self {
            range,
            player_positions: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        self.player_positions.clear();
    }

    fn push(&mut self, player_positions: (Player, (usize, usize))) {
        self.player_positions.push(player_positions);
    }
}

impl History {
    pub fn new(board: Board) -> Self {
        Self {
            current_turn: 0,
            record: Record::new(board.range()),
            boards: vec![board],
        }
    }

    pub fn init(&mut self, board: Board) {
        self.current_turn = 0;
        self.record.init();
        self.boards.clear();
        self.boards.push(board);
    }

    pub fn push(&mut self, player_position: (Player, (usize, usize)), board: Board) {
        if self.current_turn < self.boards.len() - 1 {
            self.boards.drain(self.current_turn + 1..);
            self.record.player_positions.drain(self.current_turn..);
        }
        self.current_turn += 1;
        self.record.push(player_position);
        self.boards.push(board);
    }

    pub fn go_next(&mut self) {
        if self.current_turn != self.boards.len() - 1 {
            self.current_turn += 1;
        }
    }

    pub fn go_prev(&mut self) {
        if self.current_turn != 0 {
            self.current_turn -= 1;
        }
    }
}
