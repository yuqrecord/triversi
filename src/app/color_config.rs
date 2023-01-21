// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::Player;
use derive_new::new;
use tui::style::Color;

#[derive(Clone, Copy, Debug, new)]
pub struct ColorConfig {
    player: (Color, Color, Color),
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            player: (Color::Cyan, Color::Magenta, Color::Yellow),
        }
    }
}

impl ColorConfig {
    pub fn player(&self, player: Player) -> Color {
        match player {
            Player::Zero => self.player.0,
            Player::One => self.player.1,
            Player::Two => self.player.2,
        }
    }
}
