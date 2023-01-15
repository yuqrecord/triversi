// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::app::key_binding;
use crate::board::{AvailableList, LatticeBlock, LatticeBoard, Player, PlayerMark, PLAYER_LIST};
use crate::error::TriversiError;
use getset::CopyGetters;
use std::cmp;
use std::fmt::Write as _;
use termion::event::Key;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
#[cfg(debug_assertions)]
use tui::widgets::Wrap;
use tui::widgets::{Block, Borders, Paragraph};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Play(Play),
    AskInit,
    AskQuit,
    Quit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Play {
    Turn,
    Skipped,
    Finished,
}

#[derive(Clone, Copy, Debug)]
struct ColorConfig {
    player: (Color, Color, Color),
}

#[derive(CopyGetters)]
pub struct System {
    lattice_board: LatticeBoard,
    message: String,
    message_color: Color,
    key_binding_guidance: String,
    current_player: Player,
    #[getset(get_copy = "pub")]
    current_status: Status,
    previous_status: Status,
    board_offset: (i16, i16),
    color_config: ColorConfig,
    available_list: AvailableList,
    #[cfg(debug_assertions)]
    debug_information: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            player: (Color::Cyan, Color::Magenta, Color::Yellow),
        }
    }
}

impl ColorConfig {
    fn player(&self, player: Player) -> Color {
        match player {
            Player::One => self.player.0,
            Player::Two => self.player.1,
            Player::Three => self.player.2,
        }
    }
}

impl System {
    pub fn try_new(
        range: usize,
        distance: usize,
        player_mark: PlayerMark,
    ) -> Result<Self, TriversiError> {
        let lattice_board = LatticeBoard::try_new(range, distance, player_mark)?;
        let current_player = Player::default();
        let mut available_list = AvailableList::default();
        lattice_board
            .logic_board()
            .update_available_list(&mut available_list);
        Ok(Self {
            lattice_board,
            current_player,
            message: String::new(),
            message_color: Color::Reset,
            key_binding_guidance: key_binding::make_guidance(),
            current_status: Status::Play(Play::Turn),
            previous_status: Status::Play(Play::Turn),
            board_offset: (0, 0),
            color_config: ColorConfig::default(),
            available_list,
            #[cfg(debug_assertions)]
            debug_information: String::new(),
        })
    }

    fn init(&mut self) {
        self.lattice_board.init();
        self.current_player = Player::default();
        self.clear_message();
        self.current_status = Status::Play(Play::Turn);
        self.previous_status = Status::Play(Play::Turn);
        self.update_available_list();
    }

    fn clear_message(&mut self) {
        self.message.clear();
        self.message_color = Color::Reset;
    }

    fn update_status(&mut self, status: Status) {
        self.previous_status = self.current_status;
        self.current_status = status;
    }

    fn update_available_list(&mut self) {
        self.lattice_board
            .logic_board()
            .update_available_list(&mut self.available_list);
    }

    fn set_player(&mut self) {
        for position in self
            .available_list
            .get(&self.current_player)
            .unwrap()
            .get(&self.lattice_board.logic_board().cursor())
            .unwrap()
        {
            self.lattice_board
                .set_player(*position, Some(self.current_player));
        }
        self.update_available_list();
    }

    pub fn transition(&mut self, key: Key) {
        match self.current_status {
            Status::Play(play) => self.play(key, play),
            Status::AskInit => self.ask_init(key),
            Status::AskQuit => self.ask_quit(key),
            Status::Quit => unreachable!(),
        }
    }

    pub fn ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        match self.current_status {
            Status::Play(play) => self.ui_play(frame, play),
            Status::AskInit => self.ui_ask_init(frame),
            Status::AskQuit => self.ui_ask_quit(frame),
            Status::Quit => unreachable!(),
        }
    }

    fn play(&mut self, key: Key, play: Play) {
        match play {
            Play::Turn => match key {
                key_binding::key::QUIT => self.update_status(Status::AskQuit),
                key_binding::key::INIT => self.update_status(Status::AskInit),
                key_binding::key::BONE_TOGGLE => self.lattice_board.toggle_bone_visibility(),
                key_binding::key::MOVE_LEFT => {
                    self.lattice_board.logic_board_mut().move_cursor_left()
                }
                key_binding::key::MOVE_RIGHT => {
                    self.lattice_board.logic_board_mut().move_cursor_right()
                }
                key_binding::key::MOVE_UP => self.lattice_board.logic_board_mut().move_cursor_up(),
                key_binding::key::MOVE_DOWN => {
                    self.lattice_board.logic_board_mut().move_cursor_down()
                }
                key_binding::key::SCROLL_LEFT => self.scroll_left(),
                key_binding::key::SCROLL_RIGHT => self.scroll_right(),
                key_binding::key::SCROLL_UP => self.scroll_up(),
                key_binding::key::SCROLL_DOWN => self.scroll_down(),
                key_binding::key::SCROLL_RESET => self.scroll_reset(),
                key_binding::key::ZOOM_IN => self.lattice_board.zoom_in(),
                key_binding::key::ZOOM_OUT => self.lattice_board.zoom_out(),
                key_binding::key::SELECT => self.select_in_play_turn(),
                _ => (),
            },
            Play::Skipped => match key {
                key_binding::key::QUIT => self.update_status(Status::AskQuit),
                key_binding::key::INIT => self.update_status(Status::AskInit),
                key_binding::key::BONE_TOGGLE => self.lattice_board.toggle_bone_visibility(),
                key_binding::key::MOVE_LEFT => {
                    self.lattice_board.logic_board_mut().move_cursor_left()
                }
                key_binding::key::MOVE_RIGHT => {
                    self.lattice_board.logic_board_mut().move_cursor_right()
                }
                key_binding::key::MOVE_UP => self.lattice_board.logic_board_mut().move_cursor_up(),
                key_binding::key::MOVE_DOWN => {
                    self.lattice_board.logic_board_mut().move_cursor_down()
                }
                key_binding::key::SCROLL_LEFT => self.scroll_left(),
                key_binding::key::SCROLL_RIGHT => self.scroll_right(),
                key_binding::key::SCROLL_UP => self.scroll_up(),
                key_binding::key::SCROLL_DOWN => self.scroll_down(),
                key_binding::key::SCROLL_RESET => self.scroll_reset(),
                key_binding::key::ZOOM_IN => self.lattice_board.zoom_in(),
                key_binding::key::ZOOM_OUT => self.lattice_board.zoom_out(),
                key_binding::key::SELECT => {
                    self.clear_message();
                    self.current_player.advance();
                    self.update_status(Status::Play(Play::Turn));
                }
                _ => (),
            },
            Play::Finished => match key {
                key_binding::key::QUIT => self.update_status(Status::AskQuit),
                key_binding::key::INIT => self.update_status(Status::AskInit),
                key_binding::key::BONE_TOGGLE => self.lattice_board.toggle_bone_visibility(),
                key_binding::key::MOVE_LEFT => {
                    self.lattice_board.logic_board_mut().move_cursor_left()
                }
                key_binding::key::MOVE_RIGHT => {
                    self.lattice_board.logic_board_mut().move_cursor_right()
                }
                key_binding::key::MOVE_UP => self.lattice_board.logic_board_mut().move_cursor_up(),
                key_binding::key::MOVE_DOWN => {
                    self.lattice_board.logic_board_mut().move_cursor_down()
                }
                key_binding::key::SCROLL_LEFT => self.scroll_left(),
                key_binding::key::SCROLL_RIGHT => self.scroll_right(),
                key_binding::key::SCROLL_UP => self.scroll_up(),
                key_binding::key::SCROLL_DOWN => self.scroll_down(),
                key_binding::key::SCROLL_RESET => self.scroll_reset(),
                key_binding::key::ZOOM_IN => self.lattice_board.zoom_in(),
                key_binding::key::ZOOM_OUT => self.lattice_board.zoom_out(),
                _ => (),
            },
        }
    }

    fn select_in_play_turn(&mut self) {
        if self
            .available_list
            .get(&self.current_player)
            .unwrap()
            .contains_key(&self.lattice_board.logic_board().cursor())
        {
            self.set_player();
            if self
                .available_list
                .values()
                .all(|available| available.is_empty())
            {
                self.update_status(Status::Play(Play::Finished));
                self.clear_message();
                self.lattice_board.logic_board().count();
                write!(self.message, " Game is finished! Final Score is").unwrap();
                let mut player_iter = PLAYER_LIST.iter().peekable();
                while let Some(player) = player_iter.next() {
                    if player_iter.peek().is_none() {
                        write!(self.message, " and").unwrap();
                    }
                    write!(
                        self.message,
                        " Player-{} = {}",
                        self.lattice_board.player_mark().convert(*player),
                        self.lattice_board
                            .logic_board()
                            .count()
                            .get(player)
                            .unwrap(),
                    )
                    .unwrap();
                    if player_iter.peek().is_none() {
                        write!(self.message, ".").unwrap();
                    } else {
                        write!(self.message, ",").unwrap();
                    }
                }
            } else {
                self.current_player.advance();
                self.clear_message();
                if self
                    .available_list
                    .get(&self.current_player)
                    .unwrap()
                    .is_empty()
                {
                    self.update_status(Status::Play(Play::Skipped));
                    self.message_color = Color::Red;
                    write!(self.message, " Player-{}: Your turn is skipped, you cannot select any position. Pless [{}].",
                        self.lattice_board.player_mark().convert(self.current_player),
                        key_binding::change_key_to_str(key_binding::key::SELECT)
                    ).unwrap();
                }
            }
        } else {
            self.clear_message();
            self.message_color = Color::Red;
            write!(
                self.message,
                " Player-{}: You cannot select ({}, {}).",
                self.lattice_board
                    .player_mark()
                    .convert(self.current_player),
                self.lattice_board.logic_board().cursor().0,
                self.lattice_board.logic_board().cursor().1
            )
            .unwrap();
        }
    }

    fn scroll_left(&mut self) {
        self.board_offset.0 += 1
    }
    fn scroll_right(&mut self) {
        self.board_offset.0 -= 1
    }
    fn scroll_up(&mut self) {
        self.board_offset.1 += 1
    }
    fn scroll_down(&mut self) {
        self.board_offset.1 -= 1
    }
    fn scroll_reset(&mut self) {
        self.board_offset = (0, 0)
    }

    fn ask_quit(&mut self, key: Key) {
        match key {
            Key::Char('Y') => self.update_status(Status::Quit),
            _ => self.update_status(self.previous_status),
        }
    }

    fn ask_init(&mut self, key: Key) {
        match key {
            Key::Char('Y') => self.init(),
            _ => self.update_status(self.previous_status),
        }
    }

    fn ui_play<B: Backend>(&mut self, frame: &mut Frame<B>, play: Play) {
        let operation_box_height = 3;
        let message_box_height = 3;
        let player_box_width = 8;
        let position_box_width = 10;
        let scroll_box_width = 10;
        let zoom_box_width = 6;
        let debug_box_width = if cfg!(debug_assertions) {
            frame.size().width / 2
        } else {
            0
        };
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(operation_box_height),
                    Constraint::Length(message_box_height),
                    Constraint::Length(
                        frame.size().height - operation_box_height - message_box_height,
                    ),
                ]
                .as_ref(),
            )
            .split(frame.size());
        let chunks_1 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(player_box_width),
                    Constraint::Length(position_box_width),
                    Constraint::Length(scroll_box_width),
                    Constraint::Length(zoom_box_width),
                    Constraint::Length(
                        frame.size().width
                            - player_box_width
                            - position_box_width
                            - scroll_box_width
                            - zoom_box_width,
                    ),
                ]
                .as_ref(),
            )
            .split(chunks[1]);
        let chunks_2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(frame.size().width - debug_box_width),
                    Constraint::Length(debug_box_width),
                ]
                .as_ref(),
            )
            .split(chunks[2]);
        frame.render_widget(
            Paragraph::new(self.key_binding_guidance.as_str())
                .block(Block::default().borders(Borders::ALL)),
            chunks[0],
        );
        self.render_player(frame, play, chunks_1[0]);
        frame.render_widget(
            Paragraph::new(format!(
                "{}, {}",
                self.lattice_board.logic_board().cursor().0,
                self.lattice_board.logic_board().cursor().1,
            ))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Position")),
            chunks_1[1],
        );
        frame.render_widget(
            Paragraph::new(format!("{}, {}", self.board_offset.0, self.board_offset.1))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Scroll")),
            chunks_1[2],
        );
        frame.render_widget(
            Paragraph::new(format!("{}", self.lattice_board.distance()))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Zoom")),
            chunks_1[3],
        );
        frame.render_widget(
            Paragraph::new(Span::styled(
                &self.message,
                Style::default().fg(self.message_color),
            ))
            .block(Block::default().borders(Borders::ALL).title("Message")),
            chunks_1[4],
        );
        self.render_board(frame, play, chunks_2[0]);
        #[cfg(debug_assertions)]
        {
            self.debug_information.clear();
            writeln!(self.debug_information, " Play: {:?}\n", play).unwrap();
            writeln!(
                &mut self.debug_information,
                " {:?}",
                self.lattice_board.logic_board().count()
            )
            .unwrap();
            writeln!(&mut self.debug_information).unwrap();
            for player in PLAYER_LIST {
                writeln!(
                    &mut self.debug_information,
                    " Available position of Player-{}:",
                    self.lattice_board.player_mark().convert(*player)
                )
                .unwrap();
                let mut keys = self
                    .available_list
                    .get(player)
                    .unwrap()
                    .keys()
                    .collect::<Vec<_>>();
                keys.sort();
                for key in keys {
                    writeln!(
                        &mut self.debug_information,
                        " {:?}: {:?}",
                        key,
                        self.available_list.get(player).unwrap().get(key).unwrap()
                    )
                    .unwrap();
                }
                writeln!(&mut self.debug_information).unwrap();
            }
            frame.render_widget(
                Paragraph::new(self.debug_information.as_ref())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("DebugInformation"),
                    )
                    .wrap(Wrap { trim: false }),
                chunks_2[1],
            );
        }
    }

    fn ui_ask_init<B: Backend>(&self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .margin(1)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(50),
            ])
            .split(frame.size());
        frame.render_widget(
            Paragraph::new("Are you sure to initialize?")
                .alignment(Alignment::Center)
                .block(Block::default()),
            chunks[1],
        );
        frame.render_widget(
            Paragraph::new("Y / [n]")
                .alignment(Alignment::Center)
                .block(Block::default()),
            chunks[2],
        );
    }

    fn ui_ask_quit<B: Backend>(&self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .margin(1)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(50),
            ])
            .split(frame.size());
        frame.render_widget(
            Paragraph::new("Are you sure to quit?")
                .alignment(Alignment::Center)
                .block(Block::default()),
            chunks[1],
        );
        frame.render_widget(
            Paragraph::new("Y / [n]")
                .alignment(Alignment::Center)
                .block(Block::default()),
            chunks[2],
        );
    }

    fn render_player<B: Backend>(&self, frame: &mut Frame<B>, play: Play, rect: Rect) {
        frame.render_widget(
            Paragraph::new(Spans::from(
                [Player::One, Player::Two, Player::Three]
                    .iter()
                    .map(|&player| {
                        if player != self.current_player && play != Play::Finished {
                            Span::styled(
                                format!("{}", self.lattice_board.player_mark().convert(player)),
                                Style::default().add_modifier(Modifier::DIM),
                            )
                        } else {
                            Span::raw(format!(
                                "{}",
                                self.lattice_board.player_mark().convert(player)
                            ))
                        }
                    })
                    .collect::<Vec<_>>(),
            ))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Player")),
            rect,
        );
    }

    fn render_board<B: Backend>(&self, frame: &mut Frame<B>, play: Play, rect: Rect) {
        let mut board: Vec<Spans> = Vec::new();
        for _ in 0..cmp::max(
            self.board_offset.1 * self.lattice_board.distance() as i16,
            0,
        ) {
            board.push(Spans::from(vec![Span::raw("")]))
        }
        let lattice_cursor = self.lattice_board.cursor();
        for (y_block, block_row) in self.lattice_board.lattice_board().iter().enumerate() {
            let mut row: Vec<Span> = Vec::new();
            for _ in 0..cmp::max(
                self.board_offset.0 * self.lattice_board.distance() as i16,
                0,
            ) {
                row.push(Span::raw(" "))
            }
            for (x_block, block) in block_row.iter().enumerate() {
                match block {
                    LatticeBlock::Stone(player) => row.push(Span::styled(
                        format!("{}", self.lattice_board.block_to_char(*block)),
                        self.player_style(
                            *player,
                            self.board_offset.0 * self.lattice_board.distance() as i16
                                + x_block as i16
                                >= 0
                                && lattice_cursor == (x_block, y_block),
                        ),
                    )),
                    _ => row.push(Span::raw(format!(
                        "{}",
                        self.lattice_board.block_to_char(*block)
                    ))),
                }
            }
            board.push(Spans::from(row));
        }
        let mut boarder_style_of_board = Style::default();
        if play != Play::Finished {
            boarder_style_of_board =
                boarder_style_of_board.fg(self.color_config.player(self.current_player));
        }
        frame.render_widget(
            Paragraph::new(board)
                .scroll((
                    cmp::max(
                        -self.board_offset.1 * self.lattice_board.distance() as i16,
                        0,
                    ) as u16,
                    cmp::max(
                        -self.board_offset.0 * self.lattice_board.distance() as i16,
                        0,
                    ) as u16,
                ))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Board")
                        .border_style(boarder_style_of_board),
                ),
            rect,
        );
    }
    fn player_style(&self, player: Option<Player>, under_cursor: bool) -> Style {
        let mut style = Style::default();
        if let Some(player) = player {
            style = style.fg(self.color_config.player(player));
            if player == self.current_player {
                style = style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
            }
        }
        if under_cursor {
            style = style.add_modifier(Modifier::REVERSED);
        }
        style
    }
}
