// Copyright (c) 2023 Yuichi Ishida
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::board::Board;
use crate::game::Game;
use crate::tui_app::board_display::{BoardDisplay, ParagraphBoard};
use crate::tui_app::{ColorConfig, Tui};
use anyhow::Result;
use clap::Parser;

impl Cli {
    pub fn run() -> Result<()> {
        let arg = Cli::parse();
        let display = ParagraphBoard::try_new(arg.distance, &arg.player_names)?;
        let board = Board::try_new(arg.range)?;
        let game = Game::try_new(board, &arg.player_names)?;
        let color_config = ColorConfig::default();
        let mut tui = Tui::try_new()?;
        tui.run(game, display, color_config)?;
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
