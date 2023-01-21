// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Player {
    #[default]
    One,
    Two,
    Three,
}

pub const PLAYERS: &[Player] = &[Player::One, Player::Two, Player::Three];

impl Player {
    pub fn advance(&mut self) {
        match self {
            Player::One => *self = Player::Two,
            Player::Two => *self = Player::Three,
            Player::Three => *self = Player::One,
        }
    }
}
