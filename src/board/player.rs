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
    pub fn index(self) -> usize {
        match self {
            Player::Zero => 0,
            Player::One => 1,
            Player::Two => 2,
        }
    }

    pub fn advance(&mut self) {
        match self {
            Player::Zero => *self = Player::One,
            Player::One => *self = Player::Two,
            Player::Two => *self = Player::Zero,
        }
    }
}

/// A fixed-size map keyed by [`Player`], backed by an array indexed via
/// [`Player::index`]. Replaces the ad-hoc `(T, T, T)` tuples and
/// `HashMap<Player, T>` that were previously scattered across the codebase.
#[derive(Clone, Copy, Debug)]
pub struct PlayerMap<T>([T; 3]);

impl<T> PlayerMap<T> {
    pub fn new(zero: T, one: T, two: T) -> Self {
        Self([zero, one, two])
    }

    pub fn get(&self, player: Player) -> &T {
        &self.0[player.index()]
    }

    pub fn get_mut(&mut self, player: Player) -> &mut T {
        &mut self.0[player.index()]
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}
