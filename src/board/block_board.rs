// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>

use super::logic_board::{LogicBoard, Player};
use crate::error::TriversiError;
use getset::{Getters, MutGetters};

#[derive(Clone, Copy, Debug)]
pub enum Bond {
    LeftDown,
    RightDown,
    Horizontal,
}

#[derive(Clone, Copy, Debug)]
pub enum Block {
    Stone(Option<Player>),
    Bond(Bond),
    Background,
}

#[derive(Clone, Debug, Getters, MutGetters)]
pub struct BlockBoard {
    #[getset(get = "pub", get_mut = "pub")]
    block_board: Vec<Vec<Block>>,
    logic_board: LogicBoard,
    distance: usize,
}

impl From<Block> for char {
    fn from(f: Block) -> Self {
        match f {
            Block::Stone(player) => match player {
                Some(Player::One) => 'S',
                Some(Player::Two) => 'O',
                Some(Player::Three) => 'B',
                None => '.',
            },
            Block::Bond(bond) => match bond {
                Bond::LeftDown => '/',
                Bond::RightDown => '\\',
                Bond::Horizontal => '-',
            },
            Block::Background => ' ',
        }
    }
}

impl BlockBoard {
    pub fn try_new(range: usize, distance: usize) -> Result<Self, TriversiError> {
        if range < 5 {
            return Err(TriversiError::InvalidBoardRange(range));
        }
        if distance < 2 {
            return Err(TriversiError::InvalidBoardDistance(distance));
        }
        match range % 3 {
            0 | 2 => (),
            _ => return Err(TriversiError::InvalidBoardRange(range)),
        };
        let mut block_board = (0..distance * (range - 1) + 1)
            .map(|i_row| vec![Block::Background; distance * (range - 1) + 1 + i_row])
            .collect::<Vec<_>>();
        // Horizontal bond
        for (i_row, row) in block_board
            .iter_mut()
            .skip(distance)
            .step_by(distance)
            .enumerate()
        {
            for offset in 1..=2 * (distance - 1) - 1 {
                for block in row
                    .iter_mut()
                    .skip(distance * (range - i_row - 2) + offset + 1)
                    .step_by(distance * 2)
                {
                    *block = Block::Bond(Bond::Horizontal);
                }
            }
        }
        // LeftDown and RightDown bonds
        for offset in 1..=(distance - 1) {
            for (i_row, row) in block_board
                .iter_mut()
                .skip(offset)
                .step_by(distance)
                .enumerate()
            {
                for block in row
                    .iter_mut()
                    .skip(distance * (range - i_row - 1) - offset)
                    .step_by(distance * 2)
                {
                    *block = Block::Bond(Bond::LeftDown);
                }
                for block in row
                    .iter_mut()
                    .skip(distance * (range - i_row - 1) + offset)
                    .step_by(distance * 2)
                {
                    *block = Block::Bond(Bond::RightDown);
                }
            }
        }
        // Player None
        for (i_row, row) in block_board.iter_mut().step_by(distance).enumerate() {
            for block in row
                .iter_mut()
                .skip(distance * (range - i_row - 1))
                .step_by(distance * 2)
            {
                *block = Block::Stone(None);
            }
        }
        let mut block_board = Self {
            block_board,
            logic_board: LogicBoard::try_new(range)?,
            distance,
        };
        block_board.reflect_from_logic();
        Ok(block_board)
    }

    fn reflect_from_logic(&mut self) {
        for y_logic in 0..self.logic_board.range() {
            for x_logic in 0..=y_logic {
                let player = self.logic_board.player(x_logic, y_logic);
                let (x_block, y_block) = self.logic_to_block(x_logic, y_logic);
                self.set_block(x_block, y_block, Block::Stone(player))
            }
        }
    }

    fn logic_to_block(&self, x_logic: usize, y_logic: usize) -> (usize, usize) {
        let x_block =
            self.distance * (self.logic_board.range() - y_logic - 1) + x_logic * self.distance * 2;
        let y_block = self.distance * y_logic;
        (x_block, y_block)
    }

    fn set_block(&mut self, x_block: usize, y_block: usize, block: Block) {
        *self
            .block_board
            .get_mut(y_block)
            .unwrap()
            .get_mut(x_block)
            .unwrap() = block;
    }

    fn set_player(&mut self, x_logic: usize, y_logic: usize, player: Option<Player>) {
        let (x_block, y_block) = self.logic_to_block(x_logic, y_logic);
        *self
            .block_board
            .get_mut(y_block)
            .unwrap()
            .get_mut(x_block)
            .unwrap() = Block::Stone(player);
        self.logic_board.set_player(x_logic, y_logic, player);
    }
}
