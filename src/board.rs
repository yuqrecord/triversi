// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! Board of Triversi.
//! The shape of the board is a triangle.
//! Bellow is the shape of the board which size is 4.
//!
//! ```text
//! o
//! oo
//! ooo
//! oooo
//! ```

pub mod availables;
pub mod count;
pub mod history;
pub mod player;

pub use availables::Availables;
pub use count::Count;
pub use history::History;
pub use player::{Player, PLAYERS};

use crate::error::TriversiError;
use getset::{CopyGetters, Getters, MutGetters};
use std::iter;

#[derive(Clone, Debug, CopyGetters, Getters, MutGetters)]
pub struct Board {
    #[getset(get = "pub", get_mut = "pub")]
    board: Vec<Vec<Option<Player>>>,
    #[getset(get_copy = "pub")]
    range: usize,
    #[getset(get = "pub")]
    count: Count,
}

impl Board {
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
            count: Count::default(),
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
        self.count.reset();
        match self.range % 3 {
            0 => {
                // Player 0
                let player = Some(Player::Zero);
                self.set_player((self.range / 3, 2 * self.range / 3), player);
                self.set_player((self.range / 3 + 1, 2 * self.range / 3 - 1), player);
                self.set_player((self.range / 3 - 1, 2 * self.range / 3 - 2), player);
                self.set_player((self.range / 3 - 2, 2 * self.range / 3 - 1), player);
                // Player 1
                let player = Some(Player::One);
                self.set_player((self.range / 3, 2 * self.range / 3 - 1), player);
                self.set_player((self.range / 3 - 2, 2 * self.range / 3 - 2), player);
                self.set_player((self.range / 3 - 1, 2 * self.range / 3), player);
                self.set_player((self.range / 3 + 1, 2 * self.range / 3 + 1), player);
                // Player 2
                let player = Some(Player::Two);
                self.set_player((self.range / 3 - 1, 2 * self.range / 3 - 1), player);
                self.set_player((self.range / 3, 2 * self.range / 3 + 1), player);
                self.set_player((self.range / 3 + 1, 2 * self.range / 3), player);
                self.set_player((self.range / 3, 2 * self.range / 3 - 2), player);
            }
            2 => {
                // Player 0
                let player = Some(Player::Zero);
                self.set_player(((self.range - 2) / 3, (2 * self.range - 4) / 3), player);
                self.set_player(((self.range - 5) / 3, (2 * self.range - 1) / 3), player);
                self.set_player(((self.range + 1) / 3, (2 * self.range + 2) / 3), player);
                self.set_player(((self.range + 4) / 3, (2 * self.range - 1) / 3), player);
                // Player 1
                let player = Some(Player::One);
                self.set_player(((self.range - 2) / 3, (2 * self.range - 1) / 3), player);
                self.set_player(((self.range + 4) / 3, (2 * self.range + 2) / 3), player);
                self.set_player(((self.range + 1) / 3, (2 * self.range - 4) / 3), player);
                self.set_player(((self.range - 5) / 3, (2 * self.range - 7) / 3), player);
                // Player 2
                let player = Some(Player::Two);
                self.set_player(((self.range + 1) / 3, (2 * self.range - 1) / 3), player);
                self.set_player(((self.range - 2) / 3, (2 * self.range - 7) / 3), player);
                self.set_player(((self.range - 5) / 3, (2 * self.range - 4) / 3), player);
                self.set_player(((self.range - 2) / 3, (2 * self.range + 2) / 3), player);
            }
            _ => (),
        }
    }

    /// Player in a position.
    ///
    /// # Panics
    ///
    /// Panics if a position is out of range, i.e., `y` >= `self.range` or `x` > `y`.
    pub fn player(&self, (x, y): (usize, usize)) -> Option<Player> {
        *self.board.get(y).unwrap().get(x).unwrap()
    }

    /// Player in a position.
    ///
    /// # Panics
    ///
    /// Panics if a position is out of range, i.e., `y` >= `self.range` or `x` > `y`.
    pub fn set_player(&mut self, (x, y): (usize, usize), player: Option<Player>) {
        if let Some(player) = player {
            self.count.increment(player);
        }
        if let Some(player) = self.player((x, y)) {
            self.count.decrement(player);
        }
        *self.board.get_mut(y).unwrap().get_mut(x).unwrap() = player;
    }

    pub fn initial_position(&self) -> (usize, usize) {
        (0, 0)
    }

    pub fn move_position_up(&mut self, (ref mut x, ref mut y): &mut (usize, usize)) {
        if *y > 0 {
            *y -= 1;
            if x > y {
                *x -= 1;
            }
        }
    }

    pub fn move_position_down(&mut self, (_, ref mut y): &mut (usize, usize)) {
        if *y < self.range - 1 {
            *y += 1;
        }
    }

    pub fn move_position_left(&mut self, (ref mut x, ref mut y): &mut (usize, usize)) {
        if *x > 0 {
            *x -= 1;
        } else if *y < self.range - 1 {
            *y += 1;
        }
    }

    pub fn move_position_right(&mut self, (ref mut x, ref mut y): &mut (usize, usize)) {
        if *x < self.range - 1 {
            *x += 1;
            if x > y {
                *y += 1;
            }
        }
    }

    pub fn update_availables(&self, availables: &mut Availables) {
        for &player in PLAYERS {
            availables.get_mut(&player).unwrap().clear();
            for (y, row) in self.board.iter().enumerate() {
                for (x, target_player) in row.iter().enumerate() {
                    if target_player.is_none() {
                        if x != 0 {
                            self.add_available_for_left(player, (x, y), availables);
                            self.add_available_for_left_up(player, (x, y), availables);
                        }
                        if x != y {
                            self.add_available_for_right(player, (x, y), availables);
                            self.add_available_for_up(player, (x, y), availables);
                        }
                        if y != self.range - 1 {
                            self.add_available_for_down(player, (x, y), availables);
                            self.add_available_for_right_down(player, (x, y), availables);
                        }
                    }
                }
            }
        }
    }

    fn add_available_for_left(
        &self,
        player: Player,
        (x, y): (usize, usize),
        availables: &mut Availables,
    ) {
        self.add_available(
            player,
            (x, y),
            (x - 1, y),
            (0..x).rev(),
            iter::repeat(y),
            availables,
        );
    }

    fn add_available_for_right(
        &self,
        player: Player,
        (x, y): (usize, usize),
        availables: &mut Availables,
    ) {
        self.add_available(
            player,
            (x, y),
            (x + 1, y),
            x + 1..=y,
            iter::repeat(y),
            availables,
        );
    }

    fn add_available_for_up(
        &self,
        player: Player,
        (x, y): (usize, usize),
        availables: &mut Availables,
    ) {
        self.add_available(
            player,
            (x, y),
            (x, y - 1),
            iter::repeat(x),
            (x..y).rev(),
            availables,
        );
    }

    fn add_available_for_down(
        &self,
        player: Player,
        (x, y): (usize, usize),
        availables: &mut Availables,
    ) {
        self.add_available(
            player,
            (x, y),
            (x, y + 1),
            iter::repeat(x),
            y + 1..self.range,
            availables,
        );
    }

    fn add_available_for_left_up(
        &self,
        player: Player,
        (x, y): (usize, usize),
        availables: &mut Availables,
    ) {
        self.add_available(
            player,
            (x, y),
            (x - 1, y - 1),
            (0..x).rev(),
            (y - x..y).rev(),
            availables,
        );
    }

    fn add_available_for_right_down(
        &self,
        player: Player,
        (x, y): (usize, usize),
        availables: &mut Availables,
    ) {
        self.add_available(
            player,
            (x, y),
            (x + 1, y + 1),
            x + 1..=self.range + x - y + 1,
            y + 1..self.range,
            availables,
        );
    }

    fn add_available<IX: IntoIterator<Item = usize>, IY: IntoIterator<Item = usize>>(
        &self,
        player: Player,
        target_position: (usize, usize),
        neighbor_position: (usize, usize),
        x_iter: IX,
        y_iter: IY,
        availables: &mut Availables,
    ) {
        if let Some(neighbor) = self.player(neighbor_position) {
            availables.positions_buf_mut().clear();
            availables.positions_buf_mut().push(target_position);
            if neighbor != player {
                for under_line_position in x_iter.into_iter().zip(y_iter) {
                    if let Some(under_line_player) = self.player(under_line_position) {
                        if under_line_player == player {
                            availables.add_or_extend(
                                player,
                                target_position,
                                availables.positions_buf().clone(),
                            );
                            break;
                        } else {
                            availables.positions_buf_mut().push(under_line_position);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
