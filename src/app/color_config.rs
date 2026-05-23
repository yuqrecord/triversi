// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::{Player, PlayerMap};
use tui::style::Color;

#[derive(Clone, Copy, Debug)]
pub struct ColorConfig {
    player: PlayerMap<Color>,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            player: PlayerMap::new(Color::Cyan, Color::Magenta, Color::Yellow),
        }
    }
}

impl ColorConfig {
    pub fn player(&self, player: Player) -> Color {
        *self.player.get(player)
    }
}
