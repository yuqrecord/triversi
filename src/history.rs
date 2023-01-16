// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::{LogicBoard, Player};
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Getters, Serialize, Deserialize)]
pub struct Record {
    range: usize,
    #[getset(get = "pub")]
    player_putting_list: Vec<(Player, (usize, usize))>,
}

#[derive(Clone, Debug, CopyGetters, Getters, Serialize, Deserialize)]
pub struct History {
    #[getset(get_copy = "pub")]
    current_turn: usize,
    #[getset(get = "pub")]
    record: Record,
    logic_board_list: Vec<Vec<Vec<Option<Player>>>>,
}

impl Record {
    pub fn new(range: usize) -> Self {
        Self {
            range,
            player_putting_list: Vec::new(),
        }
    }

    pub fn push(&mut self, player_putting: (Player, (usize, usize))) {
        self.player_putting_list.push(player_putting);
    }

    pub fn init(&mut self) {
        self.player_putting_list.clear();
    }
}

impl History {
    pub fn new(logic_board: &LogicBoard) -> Self {
        Self {
            current_turn: 0,
            record: Record::new(logic_board.range()),
            logic_board_list: vec![logic_board.board().clone()],
        }
    }

    pub fn push(
        &mut self,
        player_putting: (Player, (usize, usize)),
        logic_board: Vec<Vec<Option<Player>>>,
    ) {
        self.current_turn += 1;
        self.record.push(player_putting);
        self.logic_board_list.push(logic_board);
    }

    pub fn init(&mut self, logic_board: &LogicBoard) {
        self.current_turn = 0;
        self.record.init();
        self.logic_board_list.clear();
        self.logic_board_list.push(logic_board.board().clone());
    }
}
