// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TriversiError {
    #[error("{0} is invalid board range.")]
    InvalidBoardRange(usize),
    // #[error("{0} is invalid board interval.")]
    // InvalidBoardInterval(usize),
    #[error("{0} is invalid distance.")]
    InvalidBoardDistance(usize),
    #[error("({0}, {0}) is invalid stone position.")]
    InvalidStonePosition(usize, usize),
}
