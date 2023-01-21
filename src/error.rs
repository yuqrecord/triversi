// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TriversiError {
    #[error("{0} is invalid board range.")]
    InvalidBoardRange(usize),
    #[error("{0} is invalid distance.")]
    InvalidBoardDistance(usize),
    #[error("{0} is an invalid string to get player marks.")]
    InvalidStringForPlayerMarks(String),
    #[error("{0} is an invalid string to get player names.")]
    InvalidStringForPlayerNames(String),
}
