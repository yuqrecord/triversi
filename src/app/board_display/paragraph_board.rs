// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::app::board_display::{BoardDisplay, ColorConfig};
use crate::app::system::Play;
use crate::board::{Board, Player};
use crate::error::TriversiError;
use std::cmp;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

#[derive(Clone, Copy, Debug)]
pub struct PlayerMark(char, char, char);

impl PlayerMark {
    // fn new(mark_0: char, mark_1: char, mark_2: char) -> Self {
    //     Self(mark_0, mark_1, mark_2)
    // }
    fn convert(&self, player: Player) -> char {
        match player {
            Player::Zero => self.0,
            Player::One => self.1,
            Player::Two => self.2,
        }
    }
}

impl TryFrom<String> for PlayerMark {
    type Error = TriversiError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mark_list = s.split(',').collect::<Vec<_>>();
        if mark_list.len() != 3
            || mark_list.iter().any(|mark| mark.is_empty())
            || mark_list
                .iter()
                .any(|mark| !mark.chars().next().unwrap().is_ascii())
        {
            return Err(TriversiError::InvalidStringForPlayerMarks(s));
        }
        Ok(Self(
            mark_list.first().unwrap().chars().next().unwrap(),
            mark_list.get(1).unwrap().chars().next().unwrap(),
            mark_list.get(2).unwrap().chars().next().unwrap(),
        ))
    }
}

pub struct ParagraphBoard {
    distance: usize,
    offset: (i16, i16),
    player_mark: PlayerMark,
    player_name: (String, String, String),
    frame_visibility: bool,
}

impl ParagraphBoard {
    pub fn try_new(distance: usize, player_names_str: &str) -> Result<Self, TriversiError> {
        let names = player_names_str.split(',').collect::<Vec<_>>();
        let player_mark = PlayerMark::try_from(player_names_str.to_owned())?;
        if names.len() != 3 {
            return Err(TriversiError::InvalidStringForPlayerNames(
                player_names_str.to_owned(),
            ));
        }
        Ok(Self {
            distance,
            offset: (0, 0),
            player_mark,
            player_name: (
                names.first().unwrap().to_string(),
                names.get(1).unwrap().to_string(),
                names.get(2).unwrap().to_string(),
            ),
            frame_visibility: false,
        })
    }

    fn cell_position(&self, board: &Board, (x, y): (usize, usize)) -> (usize, usize) {
        let x_block = self.distance * (board.range() - y - 1) + x * self.distance * 2;
        let y_block = self.distance * y;
        (x_block, y_block)
    }

    fn cell_none(&self) -> char {
        ' '
    }

    fn cell_background(&self) -> char {
        ' '
    }

    fn cell_bottom_frame(&self) -> char {
        match self.frame_visibility {
            true => '-',
            false => ' ',
        }
    }

    fn cell_left_frame(&self) -> char {
        match self.frame_visibility {
            true => '/',
            false => ' ',
        }
    }

    fn cell_right_frame(&self) -> char {
        match self.frame_visibility {
            true => '\\',
            false => ' ',
        }
    }

    fn cell_player(&self, player: Option<Player>) -> char {
        match player {
            Some(player) => self.player_mark.convert(player),
            None => match self.frame_visibility {
                true => ' ',
                false => '.',
            },
        }
    }

    fn make_empty_board_cells(
        &self,
        board: &Board,
        (net_offset_x, net_offset_y): (usize, usize),
    ) -> Vec<Spans> {
        let mut board_cells = Vec::new();
        for _ in 0..net_offset_y {
            board_cells.push(Spans::from(vec![Span::raw("")]));
        }
        for _ in 0..=self.distance * (board.range() - 1) {
            let mut line =
                Vec::with_capacity(net_offset_x + 2 * self.distance * (board.range() - 1) + 1);
            line.extend(vec![
                Span::raw(format!("{}", self.cell_none()));
                net_offset_x
            ]);
            line.extend(vec![
                Span::raw(format!("{}", self.cell_background()));
                2 * self.distance * (board.range() - 1) + 1
            ]);
            board_cells.push(Spans::from(line))
        }
        board_cells
    }

    fn put_bottom_frame(
        &self,
        board: &Board,
        (net_offset_x, net_offset_y): (usize, usize),
        board_cells: &mut [Spans],
    ) {
        for (i_row, row) in board_cells
            .iter_mut()
            .skip(net_offset_y + self.distance)
            .step_by(self.distance)
            .enumerate()
        {
            for offset_in_board in 1..=2 * self.distance - 3 {
                for cell in row
                    .0
                    .iter_mut()
                    .skip(
                        net_offset_x
                            + self.distance * (board.range() - i_row - 2)
                            + offset_in_board
                            + 1,
                    )
                    .step_by(2 * self.distance)
                    .take(i_row + 1)
                {
                    *cell = Span::raw(format!("{}", self.cell_bottom_frame()));
                }
            }
        }
    }

    fn put_left_frame(
        &self,
        board: &Board,
        (net_offset_x, net_offset_y): (usize, usize),
        board_cells: &mut [Spans],
    ) {
        for offset_in_board in 1..=(self.distance - 1) {
            for (i_row, row) in board_cells
                .iter_mut()
                .skip(net_offset_y + offset_in_board)
                .step_by(self.distance)
                .enumerate()
            {
                for cell in row
                    .0
                    .iter_mut()
                    .skip(
                        net_offset_x + self.distance * (board.range() - i_row - 1)
                            - offset_in_board,
                    )
                    .step_by(2 * self.distance)
                    .take(i_row + 1)
                {
                    *cell = Span::raw(format!("{}", self.cell_left_frame()));
                }
            }
        }
    }

    fn put_right_frame(
        &self,
        board: &Board,
        (net_offset_x, net_offset_y): (usize, usize),
        board_cells: &mut [Spans],
    ) {
        for offset_in_board in 1..=(self.distance - 1) {
            for (i_row, row) in board_cells
                .iter_mut()
                .skip(net_offset_y + offset_in_board)
                .step_by(self.distance)
                .enumerate()
            {
                for cell in row
                    .0
                    .iter_mut()
                    .skip(
                        net_offset_x
                            + self.distance * (board.range() - i_row - 1)
                            + offset_in_board,
                    )
                    .step_by(2 * self.distance)
                    .take(i_row + 1)
                {
                    *cell = Span::raw(format!("{}", self.cell_right_frame()));
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn put_player(
        &self,
        board: &Board,
        (net_offset_x, net_offset_y): (usize, usize),
        net_scroll: (usize, usize),
        color_config: ColorConfig,
        current_player: Player,
        current_position: (usize, usize),
        board_cells: &mut [Spans],
    ) {
        for (i_row, row) in board_cells
            .iter_mut()
            .skip(net_offset_y)
            .step_by(self.distance)
            .enumerate()
        {
            for (i_col, cell) in row
                .0
                .iter_mut()
                .skip(net_offset_x + self.distance * (board.range() - i_row - 1))
                .step_by(self.distance * 2)
                .take(i_row + 1)
                .enumerate()
            {
                let player = board.player((i_col, i_row));
                *cell = Span::styled(
                    format!("{}", self.cell_player(player)),
                    self.make_player_style(
                        board,
                        net_scroll,
                        color_config,
                        current_player,
                        current_position,
                        player,
                        (i_col, i_row),
                    ),
                );
            }
        }
    }

    fn make_board_cells(
        &self,
        board: &Board,
        net_scroll: (usize, usize),
        color_config: ColorConfig,
        current_player: Player,
        current_position: (usize, usize),
    ) -> Vec<Spans> {
        let net_offset = (
            cmp::max(0, self.offset.0 * self.distance as i16) as usize,
            cmp::max(0, self.offset.1 * self.distance as i16) as usize,
        );
        let mut board_cells = self.make_empty_board_cells(board, net_offset);
        self.put_bottom_frame(board, net_offset, &mut board_cells);
        self.put_left_frame(board, net_offset, &mut board_cells);
        self.put_right_frame(board, net_offset, &mut board_cells);
        self.put_player(
            board,
            net_offset,
            net_scroll,
            color_config,
            current_player,
            current_position,
            &mut board_cells,
        );
        board_cells
    }

    fn make_boarder_style(
        &self,
        color_config: ColorConfig,
        play: Play,
        current_player: Player,
    ) -> Style {
        let mut boarder_style_of_board = Style::default();
        match play {
            Play::Finished | Play::History => (),
            _ => {
                boarder_style_of_board =
                    boarder_style_of_board.fg(color_config.player(current_player))
            }
        }
        boarder_style_of_board
    }

    #[allow(clippy::too_many_arguments)]
    fn make_player_style(
        &self,
        board: &Board,
        (net_scroll_x, _): (usize, usize),
        color_config: ColorConfig,
        current_player: Player,
        current_position: (usize, usize),
        player: Option<Player>,
        position: (usize, usize),
    ) -> Style {
        let mut style = Style::default();
        if self.cell_position(board, position).0 as i64 - net_scroll_x as i64 >= 0 {
            if let Some(player) = player {
                style = style.fg(color_config.player(player));
                if player == current_player {
                    style = style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
                }
            }
            if current_position == position {
                style = style.add_modifier(Modifier::REVERSED);
            }
        }
        style
    }
}

impl BoardDisplay for ParagraphBoard {
    const MAX_DISTANCE: usize = 10;

    fn player_name(&self, player: Player) -> &str {
        match player {
            Player::Zero => &self.player_name.0,
            Player::One => &self.player_name.1,
            Player::Two => &self.player_name.2,
        }
    }

    fn scroll_left(&mut self) {
        self.offset.0 += 1
    }

    fn scroll_right(&mut self) {
        self.offset.0 -= 1
    }

    fn scroll_up(&mut self) {
        self.offset.1 += 1
    }

    fn scroll_down(&mut self) {
        self.offset.1 -= 1
    }

    fn scroll_reset(&mut self) {
        self.offset = (0, 0)
    }

    fn zoom_in(&mut self) {
        if self.distance < Self::MAX_DISTANCE {
            self.distance += 1;
        }
    }

    fn zoom_out(&mut self) {
        if self.distance > 2 {
            self.distance -= 1;
        }
    }

    fn toggle_frame_visibility(&mut self) {
        self.frame_visibility ^= true;
    }

    fn render_scroll_block<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        frame.render_widget(
            Paragraph::new(format!("{}, {}", self.offset.0, self.offset.1))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Scroll")),
            rect,
        );
    }

    fn render_zoom_block<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        frame.render_widget(
            Paragraph::new(format!("{}", self.distance))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Zoom")),
            rect,
        );
    }

    fn render_board_block<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        rect: Rect,
        board: &Board,
        color_config: ColorConfig,
        play: Play,
        current_player: Player,
        current_position: (usize, usize),
    ) {
        let net_scroll_x = cmp::max(0, -self.offset.0 * self.distance as i16) as u16;
        let net_scroll_y = cmp::max(0, -self.offset.1 * self.distance as i16) as u16;
        let board_cells = self.make_board_cells(
            board,
            (net_scroll_x as usize, net_scroll_y as usize),
            color_config,
            current_player,
            current_position,
        );
        frame.render_widget(
            Paragraph::new(board_cells)
                .scroll((net_scroll_y, net_scroll_x))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Board")
                        .border_style(self.make_boarder_style(color_config, play, current_player)),
                ),
            rect,
        );
    }
}
