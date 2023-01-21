// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Player {
    #[default]
    Zero,
    One,
    Two,
}

pub const PLAYERS: &[Player] = &[Player::Zero, Player::One, Player::Two];

impl Player {
    pub fn advance(&mut self) {
        match self {
            Player::Zero => *self = Player::One,
            Player::One => *self = Player::Two,
            Player::Two => *self = Player::Zero,
        }
    }
}
