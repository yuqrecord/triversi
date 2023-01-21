// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::{Player, PLAYERS};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct Count {
    count: HashMap<Player, u64>,
}

impl Default for Count {
    fn default() -> Self {
        Self {
            count: PLAYERS
                .iter()
                .map(|player| (*player, 0))
                .collect::<HashMap<_, _>>(),
        }
    }
}

impl Deref for Count {
    type Target = HashMap<Player, u64>;
    fn deref(&self) -> &Self::Target {
        &self.count
    }
}

impl DerefMut for Count {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.count
    }
}

impl Count {
    pub fn reset(&mut self) {
        *self.count.get_mut(&Player::One).unwrap() = 0;
        *self.count.get_mut(&Player::Two).unwrap() = 0;
        *self.count.get_mut(&Player::Three).unwrap() = 0;
    }
    pub fn increment(&mut self, player: Player) {
        *self.count.get_mut(&player).unwrap() += 1;
    }
    pub fn decrement(&mut self, player: Player) {
        *self.count.get_mut(&player).unwrap() -= 1;
    }
}
