// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::{Player, PlayerMap};

#[derive(Clone, Debug)]
pub struct Count {
    count: PlayerMap<u64>,
}

impl Default for Count {
    fn default() -> Self {
        Self {
            count: PlayerMap::new(0, 0, 0),
        }
    }
}

impl Count {
    pub fn get(&self, player: Player) -> u64 {
        *self.count.get(player)
    }
    pub fn reset(&mut self) {
        self.count = PlayerMap::new(0, 0, 0);
    }
    pub fn increment(&mut self, player: Player) {
        *self.count.get_mut(player) += 1;
    }
    pub fn decrement(&mut self, player: Player) {
        let count = self.count.get_mut(player);
        *count = count.saturating_sub(1);
    }
}
