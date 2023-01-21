// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

// use crate::app::{Status, System};
use crate::app::board_display::BoardDisplay;
use crate::app::system::{Status, System};
use std::io;
use std::io::Stdout;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use tui::backend::{Backend, TermionBackend};
use tui::terminal::Terminal;

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
}

impl Tui<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>> {
    pub fn try_new() -> anyhow::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?.into_alternate_screen()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Self { terminal })
    }

    pub fn run<D: BoardDisplay>(&mut self, app: &mut System<D>) -> anyhow::Result<()> {
        self.terminal.draw(|frame| app.ui(frame))?;
        while let Some(Ok(key)) = io::stdin().keys().next() {
            app.transition(key);
            if let Status::Quit = app.current_status() {
                break;
            } else {
                self.terminal.draw(|frame| app.ui(frame))?;
            }
        }
        Ok(())
    }
}
