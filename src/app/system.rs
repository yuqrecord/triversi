// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::app::board_display::BoardDisplay;
use crate::app::key_binding;
use crate::app::ColorConfig;
use crate::board::{Availables, Board, History, Player, PLAYERS};
use crate::error::TriversiError;
use getset::CopyGetters;
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
use unicode_width::UnicodeWidthStr;

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
    History,
    Skipped,
    Finished,
}

#[derive(CopyGetters)]
pub struct System<D: BoardDisplay> {
    current_player: Player,
    current_position: (usize, usize),
    board: Board,
    board_display: D,
    availables: Availables,
    history: History,
    #[getset(get_copy = "pub")]
    current_status: Status,
    previous_status: Status,
    message: String,
    message_color: Color,
    color_config: ColorConfig,
    #[cfg(debug_assertions)]
    debug_information: String,
}

impl<D: BoardDisplay> System<D> {
    pub fn try_new(board: Board, board_display: D) -> Result<Self, TriversiError> {
        let mut availables = Availables::default();
        board.update_availables(&mut availables);
        Ok(Self {
            history: History::new(board.clone()),
            board_display,
            current_player: Player::default(),
            current_position: board.initial_position(),
            board,
            message: String::new(),
            message_color: Color::Reset,
            current_status: Status::Play(Play::Turn),
            previous_status: Status::Play(Play::Turn),
            color_config: ColorConfig::default(),
            availables,
            #[cfg(debug_assertions)]
            debug_information: String::new(),
        })
    }

    fn init(&mut self) {
        self.board.init();
        self.current_player = Player::default();
        self.clear_message();
        self.current_position = self.board.initial_position();
        self.current_status = Status::Play(Play::Turn);
        self.previous_status = Status::Play(Play::Turn);
        self.update_available_list();
        self.history.init(self.board.clone());
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
        self.board.update_availables(&mut self.availables);
    }

    fn set_player(&mut self) {
        for position in self
            .availables
            .get(&self.current_player)
            .unwrap()
            .get(&self.current_position)
            .unwrap()
        {
            self.board.set_player(*position, Some(self.current_player));
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
                key_binding::key::FRAME_TOGGLE => self.board_display.toggle_frame_visibility(),
                key_binding::key::MOVE_LEFT => {
                    self.board.move_position_left(&mut self.current_position)
                }
                key_binding::key::MOVE_RIGHT => {
                    self.board.move_position_right(&mut self.current_position)
                }
                key_binding::key::MOVE_UP => {
                    self.board.move_position_up(&mut self.current_position)
                }
                key_binding::key::MOVE_DOWN => {
                    self.board.move_position_down(&mut self.current_position)
                }
                key_binding::key::SCROLL_LEFT => self.board_display.scroll_left(),
                key_binding::key::SCROLL_RIGHT => self.board_display.scroll_right(),
                key_binding::key::SCROLL_UP => self.board_display.scroll_up(),
                key_binding::key::SCROLL_DOWN => self.board_display.scroll_down(),
                key_binding::key::SCROLL_RESET => self.board_display.scroll_reset(),
                key_binding::key::ZOOM_IN => self.board_display.zoom_in(),
                key_binding::key::ZOOM_OUT => self.board_display.zoom_out(),
                key_binding::key::INTO_HISTORY => self.update_status(Status::Play(Play::History)),
                key_binding::key::SELECT => self.select_in_play_turn(),
                _ => (),
            },
            Play::History => match key {
                key_binding::key::QUIT => self.update_status(Status::AskQuit),
                key_binding::key::INIT => self.update_status(Status::AskInit),
                key_binding::key::PREV_HISTORY | key_binding::key::NEXT_HISTORY => {
                    self.history_move(key)
                }
                key_binding::key::SCROLL_LEFT => self.board_display.scroll_left(),
                key_binding::key::SCROLL_RIGHT => self.board_display.scroll_right(),
                key_binding::key::SCROLL_UP => self.board_display.scroll_up(),
                key_binding::key::SCROLL_DOWN => self.board_display.scroll_down(),
                key_binding::key::SCROLL_RESET => self.board_display.scroll_reset(),
                key_binding::key::ZOOM_IN => self.board_display.zoom_in(),
                key_binding::key::ZOOM_OUT => self.board_display.zoom_out(),
                key_binding::key::SELECT => self.update_status(Status::Play(Play::Turn)),
                _ => (),
            },
            Play::Skipped => match key {
                key_binding::key::QUIT => self.update_status(Status::AskQuit),
                key_binding::key::INIT => self.update_status(Status::AskInit),
                key_binding::key::FRAME_TOGGLE => self.board_display.toggle_frame_visibility(),
                key_binding::key::MOVE_LEFT => {
                    self.board.move_position_left(&mut self.current_position)
                }
                key_binding::key::MOVE_RIGHT => {
                    self.board.move_position_right(&mut self.current_position)
                }
                key_binding::key::MOVE_UP => {
                    self.board.move_position_up(&mut self.current_position)
                }
                key_binding::key::MOVE_DOWN => {
                    self.board.move_position_down(&mut self.current_position)
                }
                key_binding::key::SCROLL_LEFT => self.board_display.scroll_left(),
                key_binding::key::SCROLL_RIGHT => self.board_display.scroll_right(),
                key_binding::key::SCROLL_UP => self.board_display.scroll_up(),
                key_binding::key::SCROLL_DOWN => self.board_display.scroll_down(),
                key_binding::key::SCROLL_RESET => self.board_display.scroll_reset(),
                key_binding::key::ZOOM_IN => self.board_display.zoom_in(),
                key_binding::key::ZOOM_OUT => self.board_display.zoom_out(),
                key_binding::key::INTO_HISTORY => self.update_status(Status::Play(Play::History)),
                key_binding::key::SELECT => self.select_in_play_skip(),
                _ => (),
            },
            Play::Finished => match key {
                key_binding::key::QUIT => self.update_status(Status::AskQuit),
                key_binding::key::INIT => self.update_status(Status::AskInit),
                key_binding::key::FRAME_TOGGLE => self.board_display.toggle_frame_visibility(),
                key_binding::key::MOVE_LEFT => {
                    self.board.move_position_left(&mut self.current_position)
                }
                key_binding::key::MOVE_RIGHT => {
                    self.board.move_position_right(&mut self.current_position)
                }
                key_binding::key::MOVE_UP => {
                    self.board.move_position_up(&mut self.current_position)
                }
                key_binding::key::MOVE_DOWN => {
                    self.board.move_position_down(&mut self.current_position)
                }
                key_binding::key::SCROLL_LEFT => self.board_display.scroll_left(),
                key_binding::key::SCROLL_RIGHT => self.board_display.scroll_right(),
                key_binding::key::SCROLL_UP => self.board_display.scroll_up(),
                key_binding::key::SCROLL_DOWN => self.board_display.scroll_down(),
                key_binding::key::SCROLL_RESET => self.board_display.scroll_reset(),
                key_binding::key::ZOOM_IN => self.board_display.zoom_in(),
                key_binding::key::ZOOM_OUT => self.board_display.zoom_out(),
                key_binding::key::INTO_HISTORY => self.update_status(Status::Play(Play::History)),
                _ => (),
            },
        }
    }

    fn select_in_play_turn(&mut self) {
        if self
            .availables
            .get(&self.current_player)
            .unwrap()
            .contains_key(&self.current_position)
        {
            self.set_player();
            if self
                .availables
                .values()
                .all(|available| available.is_empty())
            {
                self.history.push(
                    (self.current_player, self.current_position),
                    self.board.clone(),
                );
                self.update_status(Status::Play(Play::Finished));
                self.clear_message();
                self.board.count();
                write!(self.message, " Game is finished! Final Score is").unwrap();
                let mut player_iter = PLAYERS.iter().peekable();
                while let Some(player) = player_iter.next() {
                    if player_iter.peek().is_none() {
                        write!(self.message, " and").unwrap();
                    }
                    write!(
                        self.message,
                        " {} = {}",
                        self.board_display.player_name(*player),
                        self.board.count().get(player).unwrap(),
                    )
                    .unwrap();
                    if player_iter.peek().is_none() {
                        write!(self.message, ".").unwrap();
                    } else {
                        write!(self.message, ",").unwrap();
                    }
                }
            } else {
                self.history.push(
                    (self.current_player, self.current_position),
                    self.board.clone(),
                );
                self.current_player.advance();
                self.clear_message();
                if self
                    .availables
                    .get(&self.current_player)
                    .unwrap()
                    .is_empty()
                {
                    self.update_status(Status::Play(Play::Skipped));
                    self.message_color = Color::Red;
                    write!(self.message, " Player-{}: Your turn is skipped, you cannot select any position. Pless [{}].",
                        self.board_display.player_name(self.current_player),
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
                self.board_display.player_name(self.current_player),
                self.current_position.0,
                self.current_position.1
            )
            .unwrap();
        }
    }

    fn select_in_play_skip(&mut self) {
        self.clear_message();
        self.current_player.advance();
        if self
            .availables
            .get(&self.current_player)
            .unwrap()
            .is_empty()
        {
            self.update_status(Status::Play(Play::Skipped));
            self.message_color = Color::Red;
            write!(
                self.message,
                " Player-{}: Your turn is skipped, you cannot select any position. Pless [{}].",
                self.board_display.player_name(self.current_player),
                key_binding::change_key_to_str(key_binding::key::SELECT)
            )
            .unwrap();
        } else {
            self.update_status(Status::Play(Play::Turn));
        }
    }

    fn history_move(&mut self, key: Key) {
        if key == key_binding::key::PREV_HISTORY {
            self.history.go_prev();
        } else {
            self.history.go_next();
        }
        self.board = self.history.board().clone();
        if self.history.past_player().is_some() {
            self.current_player = self.history.past_player().unwrap();
        }
        if self.history.past_position().is_some() {
            self.current_position = self.history.past_position().unwrap();
        }
        self.update_available_list();
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
        let guidance_box_height = 4;
        let message_box_height = 3;
        let player_box_width = 6 + PLAYERS
            .iter()
            .map(|player| self.board_display.player_name(*player).width_cjk())
            .sum::<usize>() as u16;
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
                    Constraint::Length(guidance_box_height),
                    Constraint::Length(message_box_height),
                    Constraint::Length(
                        frame.size().height - guidance_box_height - message_box_height,
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
        let guidance = if play == Play::History {
            key_binding::make_guidance_in_history()
        } else {
            key_binding::make_guidance_in_turn()
        };
        self.render_guidance_block(frame, chunks[0], guidance);
        self.render_player_block(frame, chunks_1[0], play);
        self.render_position_block(frame, chunks_1[1]);
        self.board_display.render_scroll_block(frame, chunks_1[2]);
        self.board_display.render_zoom_block(frame, chunks_1[3]);
        self.render_message_block(frame, chunks_1[4]);
        self.board_display.render_board_block(
            frame,
            chunks_2[0],
            &self.board,
            self.color_config,
            play,
            self.current_player,
            self.current_position,
        );
        #[cfg(debug_assertions)]
        {
            // self.write_debug_info_of_available_position(play);
            self.write_debug_info_of_history();
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

    fn render_guidance_block<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        rect: Rect,
        guidance: String,
    ) {
        frame.render_widget(
            Paragraph::new(guidance).block(Block::default().borders(Borders::ALL)),
            rect,
        );
    }

    fn render_player_block<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect, play: Play) {
        let mut player_names: Vec<Span> = Vec::new();
        let mut players_iter = PLAYERS.iter().peekable();
        while let Some(player) = players_iter.next() {
            if player != &self.current_player && play != Play::Finished {
                player_names.push(Span::styled(
                    if players_iter.peek().is_none() {
                        self.board_display.player_name(*player).to_owned()
                    } else {
                        format!("{} ", self.board_display.player_name(*player))
                    },
                    Style::default().add_modifier(Modifier::DIM),
                ))
            } else {
                player_names.push(Span::raw(if players_iter.peek().is_none() {
                    self.board_display.player_name(*player).to_owned()
                } else {
                    format!("{} ", self.board_display.player_name(*player))
                }))
            }
        }
        frame.render_widget(
            Paragraph::new(Spans::from(
                PLAYERS
                    .iter()
                    .map(|&player| {
                        if player != self.current_player && play != Play::Finished {
                            Span::styled(
                                format!(" {}", self.board_display.player_name(player)),
                                Style::default().add_modifier(Modifier::DIM),
                            )
                        } else {
                            Span::raw(format!(" {}", self.board_display.player_name(player)))
                        }
                    })
                    .collect::<Vec<_>>(),
            ))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Player")),
            rect,
        );
    }

    fn render_position_block<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        frame.render_widget(
            Paragraph::new(format!(
                "{}, {}",
                self.current_position.0, self.current_position.1,
            ))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Position")),
            rect,
        );
    }

    fn render_message_block<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        frame.render_widget(
            Paragraph::new(Span::styled(
                &self.message,
                Style::default().fg(self.message_color),
            ))
            .block(Block::default().borders(Borders::ALL).title("Message")),
            rect,
        );
    }

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn write_debug_info_of_history(&mut self) {
        self.debug_information.clear();
        writeln!(
            self.debug_information,
            " Turn {}",
            self.history.current_turn(),
        )
        .unwrap();
        for player_putting in self.history.record().player_positions() {
            writeln!(self.debug_information, " {:?}", player_putting).unwrap();
        }
    }

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn write_debug_info_of_available_position(&mut self, play: Play) {
        self.debug_information.clear();
        writeln!(self.debug_information, " Play: {:?}\n", play).unwrap();
        writeln!(&mut self.debug_information, " {:?}", self.board.count()).unwrap();
        writeln!(&mut self.debug_information).unwrap();
        for player in PLAYERS {
            writeln!(
                &mut self.debug_information,
                " Available position of Player-{}:",
                self.board_display.player_name(*player)
            )
            .unwrap();
            let mut keys = self
                .availables
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
                    self.availables.get(player).unwrap().get(key).unwrap()
                )
                .unwrap();
            }
            writeln!(&mut self.debug_information).unwrap();
        }
    }
}
