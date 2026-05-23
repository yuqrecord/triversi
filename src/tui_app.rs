// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! Terminal (TUI) view layer for Triversi.
//!
//! It owns the rendering backend and the view-only state (board display,
//! colors), translates terminal key events into [`crate::game::Action`]s, and
//! renders the [`crate::game::Game`] state. The game core itself has no
//! knowledge of this layer.

pub mod board_display;
pub mod color_config;
pub mod key_binding;
pub mod render;

pub use color_config::ColorConfig;

use crate::game::{Game, Play, Status};
use crate::tui_app::board_display::BoardDisplay;
use crate::tui_app::key_binding::ViewAction;
use std::io;
use std::io::Stdout;
use termion::event::Key;
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

    pub fn run<D: BoardDisplay>(
        &mut self,
        mut game: Game,
        mut display: D,
        color_config: ColorConfig,
    ) -> anyhow::Result<()> {
        self.terminal
            .draw(|frame| render::render(frame, &game, &display, color_config))?;
        while let Some(Ok(key)) = io::stdin().keys().next() {
            handle_key(key, &mut game, &mut display);
            if let Status::Quit = game.current_status() {
                break;
            } else {
                self.terminal
                    .draw(|frame| render::render(frame, &game, &display, color_config))?;
            }
        }
        Ok(())
    }
}

/// Translate a terminal key into either a view-only action (applied to the
/// display) or a game action (dispatched to the core), matching the historical
/// per-state key behavior.
fn handle_key<D: BoardDisplay>(key: Key, game: &mut Game, display: &mut D) {
    match game.current_status() {
        Status::AskInit | Status::AskQuit => {
            game.dispatch(key_binding::confirmation_action(key));
        }
        Status::Play(play) => {
            if let Some(view_action) = key_binding::view_action(key) {
                // Frame toggling is intentionally inactive while browsing history.
                if view_action != ViewAction::ToggleFrame || play != Play::History {
                    apply_view_action(view_action, display);
                }
            } else if let Some(action) = key_binding::game_action(key) {
                game.dispatch(action);
            }
        }
        Status::Quit => unreachable!(),
    }
}

fn apply_view_action<D: BoardDisplay>(action: ViewAction, display: &mut D) {
    match action {
        ViewAction::ScrollLeft => display.scroll_left(),
        ViewAction::ScrollRight => display.scroll_right(),
        ViewAction::ScrollUp => display.scroll_up(),
        ViewAction::ScrollDown => display.scroll_down(),
        ViewAction::ScrollReset => display.scroll_reset(),
        ViewAction::ZoomIn => display.zoom_in(),
        ViewAction::ZoomOut => display.zoom_out(),
        ViewAction::ToggleFrame => display.toggle_frame_visibility(),
    }
}
