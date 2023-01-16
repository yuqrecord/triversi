// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! Logical board of Triversi.
//! The shape of the board is a triangle.
//! Bellow is the shape of the board which size is 4.
//!
//! ```text
//! o
//! oo
//! ooo
//! oooo
//! ```

use crate::error::TriversiError;
use getset::{CopyGetters, Getters, MutGetters};
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::iter;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Player {
    #[default]
    One,
    Two,
    Three,
}

pub const PLAYER_LIST: &[Player] = &[Player::One, Player::Two, Player::Three];

#[derive(Clone, Copy, Debug)]
pub struct PlayerMark(char, char, char);

#[derive(Clone, Debug)]
pub struct AvailableList {
    #[allow(clippy::type_complexity)]
    available_list: HashMap<Player, HashMap<(usize, usize), HashSet<(usize, usize)>>>,
    position_list_buf: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
pub struct Count {
    count: HashMap<Player, u64>,
}

#[derive(Clone, Debug, CopyGetters, Getters, MutGetters)]
pub struct LogicBoard {
    #[getset(get = "pub", get_mut = "pub")]
    board: Vec<Vec<Option<Player>>>,
    #[getset(get_copy = "pub")]
    range: usize,
    #[getset(get_copy = "pub")]
    cursor: (usize, usize),
    #[getset(get = "pub")]
    count: Count,
}

impl Player {
    pub fn advance(&mut self) {
        match self {
            Player::One => *self = Player::Two,
            Player::Two => *self = Player::Three,
            Player::Three => *self = Player::One,
        }
    }
}

impl PlayerMark {
    pub fn new(mark_0: char, mark_1: char, mark_2: char) -> Self {
        Self(mark_0, mark_1, mark_2)
    }
    pub fn convert(&self, player: Player) -> char {
        match player {
            Player::One => self.0,
            Player::Two => self.1,
            Player::Three => self.2,
        }
    }
}

impl Default for AvailableList {
    fn default() -> Self {
        Self {
            available_list: PLAYER_LIST
                .iter()
                .map(|player| (*player, HashMap::new()))
                .collect::<HashMap<_, _>>(),
            position_list_buf: Vec::new(),
        }
    }
}

impl Deref for AvailableList {
    type Target = HashMap<Player, HashMap<(usize, usize), HashSet<(usize, usize)>>>;
    fn deref(&self) -> &Self::Target {
        &self.available_list
    }
}

impl DerefMut for AvailableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.available_list
    }
}

impl AvailableList {
    fn add_or_extend(
        &mut self,
        player: Player,
        position: (usize, usize),
        candidate_list: Vec<(usize, usize)>,
    ) {
        for candidate in candidate_list {
            self.available_list
                .get_mut(&player)
                .unwrap()
                .entry(position)
                .or_insert_with(|| HashSet::from([candidate]))
                .insert(candidate);
        }
    }
}

impl Default for Count {
    fn default() -> Self {
        Self {
            count: PLAYER_LIST
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
    fn reset(&mut self) {
        *self.count.get_mut(&Player::One).unwrap() = 0;
        *self.count.get_mut(&Player::Two).unwrap() = 0;
        *self.count.get_mut(&Player::Three).unwrap() = 0;
    }
    fn increment(&mut self, player: Player) {
        *self.count.get_mut(&player).unwrap() += 1;
    }
    fn decrement(&mut self, player: Player) {
        *self.count.get_mut(&player).unwrap() -= 1;
    }
}

impl TryFrom<String> for PlayerMark {
    type Error = TriversiError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mark_list = s.split(',').collect::<Vec<_>>();
        if mark_list.len() != 3
            || mark_list.iter().any(|mark| !mark.is_ascii())
            || mark_list.iter().any(|mark| mark.len() != 1)
        {
            return Err(TriversiError::InvalidStringForPlayerMarks(s));
        }
        Ok(Self(
            mark_list.first().unwrap().chars().next().unwrap(),
            mark_list.get(1).unwrap().chars().next().unwrap(),
            mark_list.get(2).unwrap().chars().next().unwrap(),
        ))
    }
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
            cursor: (0, 0),
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
                // Player 1
                let player = Some(Player::One);
                self.set_player((self.range / 3, 2 * self.range / 3), player);
                self.set_player((self.range / 3 + 1, 2 * self.range / 3 - 1), player);
                self.set_player((self.range / 3 - 1, 2 * self.range / 3 - 2), player);
                self.set_player((self.range / 3 - 2, 2 * self.range / 3 - 1), player);
                // Player 2
                let player = Some(Player::Two);
                self.set_player((self.range / 3, 2 * self.range / 3 - 1), player);
                self.set_player((self.range / 3 - 2, 2 * self.range / 3 - 2), player);
                self.set_player((self.range / 3 - 1, 2 * self.range / 3), player);
                self.set_player((self.range / 3 + 1, 2 * self.range / 3 + 1), player);
                // Player 3
                let player = Some(Player::Three);
                self.set_player((self.range / 3 - 1, 2 * self.range / 3 - 1), player);
                self.set_player((self.range / 3, 2 * self.range / 3 + 1), player);
                self.set_player((self.range / 3 + 1, 2 * self.range / 3), player);
                self.set_player((self.range / 3, 2 * self.range / 3 - 2), player);
            }
            2 => {
                // Player 1
                let player = Some(Player::One);
                self.set_player(((self.range - 2) / 3, (2 * self.range - 4) / 3), player);
                self.set_player(((self.range - 5) / 3, (2 * self.range - 1) / 3), player);
                self.set_player(((self.range + 1) / 3, (2 * self.range + 2) / 3), player);
                self.set_player(((self.range + 4) / 3, (2 * self.range - 1) / 3), player);
                // Player 2
                let player = Some(Player::Two);
                self.set_player(((self.range - 2) / 3, (2 * self.range - 1) / 3), player);
                self.set_player(((self.range + 4) / 3, (2 * self.range + 2) / 3), player);
                self.set_player(((self.range + 1) / 3, (2 * self.range - 4) / 3), player);
                self.set_player(((self.range - 5) / 3, (2 * self.range - 7) / 3), player);
                // Player 3
                let player = Some(Player::Three);
                self.set_player(((self.range + 1) / 3, (2 * self.range - 1) / 3), player);
                self.set_player(((self.range - 2) / 3, (2 * self.range - 7) / 3), player);
                self.set_player(((self.range - 5) / 3, (2 * self.range - 4) / 3), player);
                self.set_player(((self.range - 2) / 3, (2 * self.range + 2) / 3), player);
            }
            _ => (),
        }
    }

    pub fn player(&self, (x, y): (usize, usize)) -> Option<Player> {
        *self.board.get(y).unwrap().get(x).unwrap()
    }

    pub fn set_player(&mut self, (x, y): (usize, usize), player: Option<Player>) {
        if let Some(player) = player {
            self.count.increment(player);
        }
        if let Some(player) = self.player((x, y)) {
            self.count.decrement(player);
        }
        *self.board.get_mut(y).unwrap().get_mut(x).unwrap() = player;
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
            if self.cursor.0 > self.cursor.1 {
                self.cursor.0 -= 1;
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.1 < self.range - 1 {
            self.cursor.1 += 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
        } else if self.cursor.1 < self.range - 1 {
            self.cursor.1 += 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor.0 < self.range - 1 {
            self.cursor.0 += 1;
            if self.cursor.0 > self.cursor.1 {
                self.cursor.1 += 1;
            }
        }
    }

    pub fn update_available_list(&self, available_list: &mut AvailableList) {
        for &player in PLAYER_LIST {
            available_list.get_mut(&player).unwrap().clear();
            for (y, row) in self.board.iter().enumerate() {
                for (x, target_player) in row.iter().enumerate() {
                    if target_player.is_none() {
                        if x != 0 {
                            self.add_available_for_left(player, (x, y), available_list);
                            self.add_available_for_left_up(player, (x, y), available_list);
                        }
                        if x != y {
                            self.add_available_for_right(player, (x, y), available_list);
                            self.add_available_for_up(player, (x, y), available_list);
                        }
                        if y != self.range - 1 {
                            self.add_available_for_down(player, (x, y), available_list);
                            self.add_available_for_right_down(player, (x, y), available_list);
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
        available_list: &mut AvailableList,
    ) {
        self.add_available(
            player,
            (x, y),
            (x - 1, y),
            (0..x).rev(),
            iter::repeat(y),
            available_list,
        );
    }

    fn add_available_for_right(
        &self,
        player: Player,
        (x, y): (usize, usize),
        available_list: &mut AvailableList,
    ) {
        self.add_available(
            player,
            (x, y),
            (x + 1, y),
            x + 1..=y,
            iter::repeat(y),
            available_list,
        );
    }

    fn add_available_for_up(
        &self,
        player: Player,
        (x, y): (usize, usize),
        available_list: &mut AvailableList,
    ) {
        self.add_available(
            player,
            (x, y),
            (x, y - 1),
            iter::repeat(x),
            (x..y).rev(),
            available_list,
        );
    }

    fn add_available_for_down(
        &self,
        player: Player,
        (x, y): (usize, usize),
        available_list: &mut AvailableList,
    ) {
        self.add_available(
            player,
            (x, y),
            (x, y + 1),
            iter::repeat(x),
            y + 1..self.range,
            available_list,
        );
    }

    fn add_available_for_left_up(
        &self,
        player: Player,
        (x, y): (usize, usize),
        available_list: &mut AvailableList,
    ) {
        self.add_available(
            player,
            (x, y),
            (x - 1, y - 1),
            (0..x).rev(),
            (y - x..y).rev(),
            available_list,
        );
    }

    fn add_available_for_right_down(
        &self,
        player: Player,
        (x, y): (usize, usize),
        available_list: &mut AvailableList,
    ) {
        self.add_available(
            player,
            (x, y),
            (x + 1, y + 1),
            x + 1..=self.range + x - y + 1,
            y + 1..self.range,
            available_list,
        );
    }

    fn add_available<IX: IntoIterator<Item = usize>, IY: IntoIterator<Item = usize>>(
        &self,
        player: Player,
        target_position: (usize, usize),
        neighbor_position: (usize, usize),
        x_iter: IX,
        y_iter: IY,
        available_list: &mut AvailableList,
    ) {
        if let Some(neighbor) = self.player(neighbor_position) {
            available_list.position_list_buf.clear();
            available_list.position_list_buf.push(target_position);
            if neighbor != player {
                for under_line_position in x_iter.into_iter().zip(y_iter) {
                    if let Some(under_line_player) = self.player(under_line_position) {
                        if under_line_player == player {
                            available_list.add_or_extend(
                                player,
                                target_position,
                                available_list.position_list_buf.clone(),
                            );
                            break;
                        } else {
                            available_list.position_list_buf.push(under_line_position);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
