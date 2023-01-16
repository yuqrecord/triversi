// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! LatticeBoard is created by changing from LogicBoard into a lattice.

use super::logic_board::{LogicBoard, Player, PlayerMark};
use crate::error::TriversiError;
use getset::{CopyGetters, Getters, MutGetters};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Bond {
    LeftDown,
    RightDown,
    Horizontal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LatticeBlock {
    Stone(Option<Player>),
    Bond(Bond),
    Background,
}

#[derive(Clone, Debug, CopyGetters, Getters, MutGetters)]
pub struct LatticeBoard {
    #[getset(get = "pub", get_mut = "pub")]
    lattice_board: Vec<Vec<LatticeBlock>>,
    #[getset(get = "pub", get_mut = "pub")]
    logic_board: LogicBoard,
    #[getset(get_copy = "pub")]
    distance: usize,
    #[getset(get_copy = "pub")]
    player_mark: PlayerMark,
    frame_visibility: bool,
}

impl fmt::Display for LatticeBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.lattice_board.iter() {
            for block in row.iter() {
                write!(f, "{}", self.block_to_char(*block))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl LatticeBoard {
    pub const MAX_DISTANCE: usize = 10;
    pub fn try_new(
        range: usize,
        distance: usize,
        player_mark: PlayerMark,
    ) -> Result<Self, TriversiError> {
        if !(2..=Self::MAX_DISTANCE).contains(&distance) {
            return Err(TriversiError::InvalidBoardDistance(distance));
        }
        let mut lattice_board = Self {
            lattice_board: Vec::new(),
            logic_board: LogicBoard::try_new(range)?,
            distance,
            player_mark,
            frame_visibility: false,
        };
        lattice_board.init();
        Ok(lattice_board)
    }

    pub fn init(&mut self) {
        self.logic_board.init();
        self.redraw();
        self.reflect_from_logic();
    }

    pub fn toggle_frame_visibility(&mut self) {
        self.frame_visibility ^= true;
    }

    pub fn zoom_in(&mut self) {
        if self.distance < Self::MAX_DISTANCE {
            self.distance += 1;
            self.redraw();
        }
    }

    pub fn zoom_out(&mut self) {
        if self.distance > 2 {
            self.distance -= 1;
            self.redraw();
        }
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.logic_to_block(self.logic_board.cursor())
    }

    pub fn block_to_char(&self, block: LatticeBlock) -> char {
        match block {
            LatticeBlock::Stone(player) => match player {
                Some(player) => self.player_mark.convert(player),
                None => {
                    if self.frame_visibility {
                        ' '
                    } else {
                        '.'
                    }
                }
            },
            LatticeBlock::Bond(bond) => {
                if self.frame_visibility {
                    match bond {
                        Bond::LeftDown => '/',
                        Bond::RightDown => '\\',
                        Bond::Horizontal => '-',
                    }
                } else {
                    match bond {
                        Bond::LeftDown => ' ',
                        Bond::RightDown => ' ',
                        Bond::Horizontal => ' ',
                    }
                }
            }
            LatticeBlock::Background => ' ',
        }
    }

    pub fn redraw(&mut self) {
        self.lattice_board = vec![
            vec![
                LatticeBlock::Background;
                self.distance * (self.logic_board.range() - 1)
                    + 1
                    + self.distance * (self.logic_board.range() - 1)
            ];
            self.distance * (self.logic_board.range() - 1) + 1
        ];
        for (i_row, row) in self
            .lattice_board
            .iter_mut()
            .skip(self.distance)
            .step_by(self.distance)
            .enumerate()
        {
            for offset in 1..=2 * (self.distance - 1) - 1 {
                for block in row
                    .iter_mut()
                    .skip(self.distance * (self.logic_board.range() - i_row - 2) + offset + 1)
                    .step_by(self.distance * 2)
                    .take(i_row + 1)
                {
                    *block = LatticeBlock::Bond(Bond::Horizontal);
                }
            }
        }
        // LeftDown and RightDown bonds
        for offset in 1..=(self.distance - 1) {
            for (i_row, row) in self
                .lattice_board
                .iter_mut()
                .skip(offset)
                .step_by(self.distance)
                .enumerate()
            {
                for block in row
                    .iter_mut()
                    .skip(self.distance * (self.logic_board.range() - i_row - 1) - offset)
                    .step_by(self.distance * 2)
                    .take(i_row + 1)
                {
                    *block = LatticeBlock::Bond(Bond::LeftDown);
                }
                for block in row
                    .iter_mut()
                    .skip(self.distance * (self.logic_board.range() - i_row - 1) + offset)
                    .step_by(self.distance * 2)
                    .take(i_row + 1)
                {
                    *block = LatticeBlock::Bond(Bond::RightDown);
                }
            }
        }
        // Player None
        for (i_row, row) in self
            .lattice_board
            .iter_mut()
            .step_by(self.distance)
            .enumerate()
        {
            for block in row
                .iter_mut()
                .skip(self.distance * (self.logic_board.range() - i_row - 1))
                .step_by(self.distance * 2)
                .take(i_row + 1)
            {
                *block = LatticeBlock::Stone(None);
            }
        }
        // Each Player
        self.reflect_from_logic();
    }

    fn reflect_from_logic(&mut self) {
        for y_logic in 0..self.logic_board.range() {
            for x_logic in 0..=y_logic {
                let player = self.logic_board.player((x_logic, y_logic));
                let block_position = self.logic_to_block((x_logic, y_logic));
                self.set_block(block_position, LatticeBlock::Stone(player))
            }
        }
    }

    fn logic_to_block(&self, (x_logic, y_logic): (usize, usize)) -> (usize, usize) {
        let x_block =
            self.distance * (self.logic_board.range() - y_logic - 1) + x_logic * self.distance * 2;
        let y_block = self.distance * y_logic;
        (x_block, y_block)
    }

    fn set_block(&mut self, (x_block, y_block): (usize, usize), block: LatticeBlock) {
        *self
            .lattice_board
            .get_mut(y_block)
            .unwrap()
            .get_mut(x_block)
            .unwrap() = block;
    }

    pub fn set_player(&mut self, logical_position: (usize, usize), player: Option<Player>) {
        let block_position = self.logic_to_block(logical_position);
        self.set_block(block_position, LatticeBlock::Stone(player));
        self.logic_board.set_player(logical_position, player);
    }
}
