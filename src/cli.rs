// Copyright (c) 2023 Yuichi Ishida
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::app::board_display::{BoardDisplay, ParagraphBoard};
use crate::app::system::System;
use crate::app::tui::Tui;
use crate::board::Board;
use anyhow::Result;
use clap::Parser;

impl Cli {
    pub fn run() -> Result<()> {
        let arg = Cli::parse();
        let paragraph_board = ParagraphBoard::try_new(arg.distance, &arg.player_names)?;
        let board = Board::try_new(arg.range)?;
        let mut system = System::try_new(board, paragraph_board)?;
        let mut tui = Tui::try_new()?;
        tui.run(&mut system)?;
        Ok(())
    }
}

#[derive(Parser)]
#[clap(author, version, about, after_help = concat!("Repository: ", env!("CARGO_PKG_REPOSITORY")))]
pub struct Cli {
    #[clap(
        short,
        long,
        default_value = "14",
        help = "Number of positions in one edge (>= 5 & = 0,2 (mod3))"
    )]
    range: usize,

    #[clap(
        short,
        long,
        default_value = "3",
        help = format!("Distance between positions (>= 2, <= {})", ParagraphBoard::MAX_DISTANCE)
    )]
    distance: usize,

    #[clap(
        short,
        long,
        default_value = "Cyan,Magenta,Yellow",
        help = "Marks of each player (delimiters are ','), "
    )]
    player_names: String,
}
