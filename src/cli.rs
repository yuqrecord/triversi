// Copyright (c) 2023 Yuichi Ishida
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::app::{System, Tui};
use crate::board::LatticeBoard;
use anyhow::Result;
use clap::Parser;

impl Cli {
    pub fn run() -> Result<()> {
        let arg = Cli::parse();
        // You should NOT construct System after constructing Tui in order to display error message correctly.
        let mut system = System::try_new(arg.range, arg.distance, arg.player_marks.try_into()?)?;
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
        help = format!("Distance between positions (>= 2, <= {})", LatticeBoard::MAX_DISTANCE)
    )]
    distance: usize,

    #[clap(
        short,
        long,
        default_value = "1,2,3",
        help = "Marks of each player (ascii characters, delimiters are ','), "
    )]
    player_marks: String,
}
