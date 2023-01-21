// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

pub mod paragraph_board;

pub use paragraph_board::ParagraphBoard;

use crate::app::system::Play;
use crate::app::ColorConfig;
use crate::board::{Board, Player};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::terminal::Frame;

pub trait BoardDisplay {
    const MAX_DISTANCE: usize;
    fn player_name(&self, player: Player) -> &str;
    fn scroll_left(&mut self);
    fn scroll_right(&mut self);
    fn scroll_up(&mut self);
    fn scroll_down(&mut self);
    fn scroll_reset(&mut self);
    fn zoom_in(&mut self);
    fn zoom_out(&mut self);
    fn toggle_frame_visibility(&mut self);
    fn render_scroll_block<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect);
    fn render_zoom_block<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect);
    fn render_board_block<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        rect: Rect,
        boad: &Board,
        color_config: ColorConfig,
        play: Play,
        current_player: Player,
        current_position: (usize, usize),
    );
    // fn render(&self);
}
