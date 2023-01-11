// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>

use crate::error::TriversiError;
use getset::{CopyGetters, Getters, MutGetters};

#[derive(Clone, Copy, Debug)]
pub enum Player {
    One,
    Two,
    Three,
}

#[derive(Clone, Debug, CopyGetters, Getters, MutGetters)]
pub struct LogicBoard {
    #[getset(get = "pub", get_mut = "pub")]
    board: Vec<Vec<Option<Player>>>,
    #[getset(get_copy = "pub")]
    range: usize,
}

impl LogicBoard {
    pub fn try_new(range: usize) -> Result<Self, TriversiError> {
        if range < 5 {
            return Err(TriversiError::InvalidBoardRange(range));
        }
        match range % 3 {
            0 | 2 => (),
            _ => return Err(TriversiError::InvalidBoardRange(range)),
        };
        let mut logic_board = Self {
            board: (1..=range)
                .map(|i_row| vec![None; i_row])
                .collect::<Vec<_>>(),
            range,
        };
        logic_board.init();
        Ok(logic_board)
    }

    pub fn init(&mut self) {
        for row in self.board.iter_mut() {
            for player in row.iter_mut() {
                *player = None;
            }
        }
        match self.range % 3 {
            0 => {
                // Player 1
                let player = Some(Player::One);
                self.set_player(self.range / 3, 2 * self.range / 3, player);
                self.set_player(self.range / 3 + 1, 2 * self.range / 3 - 1, player);
                self.set_player(self.range / 3 - 1, 2 * self.range / 3 - 2, player);
                self.set_player(self.range / 3 - 2, 2 * self.range / 3 - 1, player);
                // Player 2
                let player = Some(Player::Two);
                self.set_player(self.range / 3, 2 * self.range / 3 - 1, player);
                self.set_player(self.range / 3 - 2, 2 * self.range / 3 - 2, player);
                self.set_player(self.range / 3 - 1, 2 * self.range / 3, player);
                self.set_player(self.range / 3 + 1, 2 * self.range / 3 + 1, player);
                // Player 3
                let player = Some(Player::Three);
                self.set_player(self.range / 3 - 1, 2 * self.range / 3 - 1, player);
                self.set_player(self.range / 3, 2 * self.range / 3 + 1, player);
                self.set_player(self.range / 3 + 1, 2 * self.range / 3, player);
                self.set_player(self.range / 3, 2 * self.range / 3 - 2, player);
            }
            2 => {
                // Player 1
                let player = Some(Player::One);
                self.set_player((self.range - 2) / 3, (2 * self.range - 4) / 3, player);
                self.set_player((self.range - 5) / 3, (2 * self.range - 1) / 3, player);
                self.set_player((self.range + 1) / 3, (2 * self.range + 2) / 3, player);
                self.set_player((self.range + 4) / 3, (2 * self.range - 1) / 3, player);
                // Player 2
                let player = Some(Player::Two);
                self.set_player((self.range - 2) / 3, (2 * self.range - 1) / 3, player);
                self.set_player((self.range + 4) / 3, (2 * self.range + 2) / 3, player);
                self.set_player((self.range + 1) / 3, (2 * self.range - 4) / 3, player);
                self.set_player((self.range - 5) / 3, (2 * self.range - 7) / 3, player);
                // Player 3
                let player = Some(Player::Three);
                self.set_player((self.range + 1) / 3, (2 * self.range - 1) / 3, player);
                self.set_player((self.range - 2) / 3, (2 * self.range - 7) / 3, player);
                self.set_player((self.range - 5) / 3, (2 * self.range - 4) / 3, player);
                self.set_player((self.range - 2) / 3, (2 * self.range + 2) / 3, player);
            }
            _ => (),
        }
    }

    pub fn player(&self, x: usize, y: usize) -> Option<Player> {
        *self.board.get(y).unwrap().get(x).unwrap()
    }

    pub fn set_player(&mut self, x: usize, y: usize, player: Option<Player>) {
        *self.board.get_mut(y).unwrap().get_mut(x).unwrap() = player;
    }
}
