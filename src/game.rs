// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! TUI/GUI-independent game core of Triversi.
//!
//! [`Game`] is the state machine. It consumes abstract [`Action`]s and exposes
//! plain query getters, so any view layer (TUI, GUI, ...) can drive it without
//! the core depending on a particular input device or rendering backend.

pub mod action;
pub mod message;
pub mod status;

pub use action::Action;
pub use message::{Message, MessageKind};
pub use status::{Play, Status};

use crate::board::{Availables, Board, History, Player, PlayerMap, PLAYERS};
use crate::error::TriversiError;
use getset::{CopyGetters, Getters};
use std::fmt::Write as _;

#[derive(CopyGetters, Getters)]
pub struct Game {
    #[getset(get_copy = "pub")]
    current_player: Player,
    #[getset(get_copy = "pub")]
    current_position: (usize, usize),
    #[getset(get = "pub")]
    board: Board,
    #[getset(get = "pub")]
    availables: Availables,
    #[getset(get = "pub")]
    history: History,
    #[getset(get_copy = "pub")]
    current_status: Status,
    previous_status: Status,
    #[getset(get = "pub")]
    message: Message,
    player_name: PlayerMap<String>,
}

impl Game {
    pub fn try_new(board: Board, player_names_str: &str) -> Result<Self, TriversiError> {
        let names = player_names_str.split(',').collect::<Vec<_>>();
        if names.len() != 3 {
            return Err(TriversiError::InvalidStringForPlayerNames(
                player_names_str.to_owned(),
            ));
        }
        let player_name = PlayerMap::new(
            names[0].to_string(),
            names[1].to_string(),
            names[2].to_string(),
        );
        let mut availables = Availables::default();
        board.update_availables(&mut availables);
        Ok(Self {
            history: History::new(board.clone()),
            current_player: Player::default(),
            current_position: board.initial_position(),
            board,
            message: Message::default(),
            current_status: Status::Play(Play::Turn),
            previous_status: Status::Play(Play::Turn),
            availables,
            player_name,
        })
    }

    pub fn player_name(&self, player: Player) -> &str {
        self.player_name.get(player).as_str()
    }

    pub fn dispatch(&mut self, action: Action) {
        match self.current_status {
            Status::Play(play) => self.play(action, play),
            Status::AskInit => self.ask_init(action),
            Status::AskQuit => self.ask_quit(action),
            Status::Quit => unreachable!(),
        }
    }

    fn init(&mut self) {
        self.board.init();
        self.current_player = Player::default();
        self.message.clear();
        self.current_position = self.board.initial_position();
        self.current_status = Status::Play(Play::Turn);
        self.previous_status = Status::Play(Play::Turn);
        self.update_available_list();
        self.history.init(self.board.clone());
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
            .get(self.current_player)
            .get(&self.current_position)
            .unwrap()
        {
            self.board.set_player(*position, Some(self.current_player));
        }
        self.update_available_list();
    }

    fn play(&mut self, action: Action, play: Play) {
        match play {
            Play::Turn => match action {
                Action::Select => self.select_in_play_turn(),
                Action::SelectAt(position) => {
                    self.current_position = position;
                    self.select_in_play_turn();
                }
                _ => self.handle_common_play_action(action),
            },
            Play::Skipped => match action {
                Action::Select => self.select_in_play_skip(),
                _ => self.handle_common_play_action(action),
            },
            Play::Finished => self.handle_common_play_action(action),
            Play::History => match action {
                Action::RequestQuit => self.update_status(Status::AskQuit),
                Action::RequestInit => self.update_status(Status::AskInit),
                Action::HistoryPrev | Action::HistoryNext => self.history_move(action),
                Action::Select => self.update_status(Status::Play(Play::Turn)),
                _ => {}
            },
        }
    }

    fn handle_common_play_action(&mut self, action: Action) {
        match action {
            Action::RequestQuit => self.update_status(Status::AskQuit),
            Action::RequestInit => self.update_status(Status::AskInit),
            Action::MoveLeft => self.board.move_position_left(&mut self.current_position),
            Action::MoveRight => self.board.move_position_right(&mut self.current_position),
            Action::MoveUp => self.board.move_position_up(&mut self.current_position),
            Action::MoveDown => self.board.move_position_down(&mut self.current_position),
            Action::EnterHistory => self.update_status(Status::Play(Play::History)),
            _ => {}
        }
    }

    fn select_in_play_turn(&mut self) {
        if self
            .availables
            .get(self.current_player)
            .contains_key(&self.current_position)
        {
            self.set_player();
            if self
                .availables
                .values()
                .all(|available| available.is_empty())
            {
                self.history
                    .push(self.current_position, self.board.clone(), self.current_player);
                self.update_status(Status::Play(Play::Finished));
                self.message.clear();
                write!(self.message.text, " Game is finished! Final Score is").unwrap();
                let mut player_iter = PLAYERS.iter().peekable();
                while let Some(player) = player_iter.next() {
                    if player_iter.peek().is_none() {
                        write!(self.message.text, " and").unwrap();
                    }
                    write!(
                        self.message.text,
                        " {} = {}",
                        self.player_name.get(*player),
                        self.board.count().get(*player),
                    )
                    .unwrap();
                    if player_iter.peek().is_none() {
                        write!(self.message.text, ".").unwrap();
                    } else {
                        write!(self.message.text, ",").unwrap();
                    }
                }
            } else {
                self.current_player.advance();
                self.history
                    .push(self.current_position, self.board.clone(), self.current_player);
                self.message.clear();
                if self.availables.get(self.current_player).is_empty() {
                    self.update_status(Status::Play(Play::Skipped));
                    self.message.kind = MessageKind::Error;
                    write!(
                        self.message.text,
                        " Player-{}: Your turn is skipped, you cannot select any position.",
                        self.player_name.get(self.current_player)
                    )
                    .unwrap();
                }
            }
        } else {
            self.message.clear();
            self.message.kind = MessageKind::Error;
            write!(
                self.message.text,
                " Player-{}: You cannot select ({}, {}).",
                self.player_name.get(self.current_player),
                self.current_position.0,
                self.current_position.1
            )
            .unwrap();
        }
    }

    fn select_in_play_skip(&mut self) {
        self.message.clear();
        self.current_player.advance();
        self.history.set_current_player(self.current_player);
        if self.availables.get(self.current_player).is_empty() {
            self.update_status(Status::Play(Play::Skipped));
            self.message.kind = MessageKind::Error;
            write!(
                self.message.text,
                " Player-{}: Your turn is skipped, you cannot select any position.",
                self.player_name.get(self.current_player)
            )
            .unwrap();
        } else {
            self.update_status(Status::Play(Play::Turn));
        }
    }

    fn history_move(&mut self, action: Action) {
        match action {
            Action::HistoryPrev => self.history.go_prev(),
            Action::HistoryNext => self.history.go_next(),
            _ => return,
        }
        self.board = self.history.board().clone();
        self.current_player = self.history.current_player();
        if let Some(position) = self.history.past_position() {
            self.current_position = position;
        }
        self.update_available_list();
    }

    fn ask_quit(&mut self, action: Action) {
        match action {
            Action::Confirm => self.update_status(Status::Quit),
            _ => self.update_status(self.previous_status),
        }
    }

    fn ask_init(&mut self, action: Action) {
        match action {
            Action::Confirm => self.init(),
            _ => self.update_status(self.previous_status),
        }
    }
}
