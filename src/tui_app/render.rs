// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! Rendering of the [`Game`] state onto a `tui` frame. These functions read
//! the game core through its query getters plus the view-only display and
//! color configuration; they never mutate the game.

use crate::board::PLAYERS;
use crate::game::{Game, MessageKind, Play, Status};
use crate::tui_app::board_display::BoardDisplay;
use crate::tui_app::key_binding;
use crate::tui_app::ColorConfig;
use std::fmt::Write as _;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
#[cfg(debug_assertions)]
use tui::widgets::Wrap;
use tui::widgets::{Block, Borders, Paragraph};
use unicode_width::UnicodeWidthStr;

pub fn render<B: Backend, D: BoardDisplay>(
    frame: &mut Frame<B>,
    game: &Game,
    display: &D,
    color_config: ColorConfig,
) {
    match game.current_status() {
        Status::Play(play) => ui_play(frame, game, display, color_config, play),
        Status::AskInit => ui_ask_init(frame),
        Status::AskQuit => ui_ask_quit(frame),
        Status::Quit => unreachable!(),
    }
}

fn ui_play<B: Backend, D: BoardDisplay>(
    frame: &mut Frame<B>,
    game: &Game,
    display: &D,
    color_config: ColorConfig,
    play: Play,
) {
    let guidance_box_height = 4;
    let message_box_height = 3;
    let player_box_width = 6 + PLAYERS
        .iter()
        .map(|player| game.player_name(*player).width_cjk())
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
                Constraint::Length(frame.size().height - guidance_box_height - message_box_height),
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
    render_guidance_block(frame, chunks[0], guidance);
    render_player_block(frame, game, color_config, chunks_1[0], play);
    render_position_block(frame, game, chunks_1[1]);
    display.render_scroll_block(frame, chunks_1[2]);
    display.render_zoom_block(frame, chunks_1[3]);
    render_message_block(frame, game, chunks_1[4]);
    display.render_board_block(
        frame,
        chunks_2[0],
        game.board(),
        color_config,
        play,
        game.current_player(),
        game.current_position(),
    );
    #[cfg(debug_assertions)]
    {
        let mut debug_information = String::new();
        writeln!(debug_information, " Turn {}", game.history().current_turn()).unwrap();
        for player_putting in game.history().moves() {
            writeln!(debug_information, " {:?}", player_putting).unwrap();
        }
        frame.render_widget(
            Paragraph::new(debug_information)
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

fn ui_ask_init<B: Backend>(frame: &mut Frame<B>) {
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

fn ui_ask_quit<B: Backend>(frame: &mut Frame<B>) {
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

fn render_guidance_block<B: Backend>(frame: &mut Frame<B>, rect: Rect, guidance: String) {
    frame.render_widget(
        Paragraph::new(guidance).block(Block::default().borders(Borders::ALL)),
        rect,
    );
}

fn render_player_block<B: Backend>(
    frame: &mut Frame<B>,
    game: &Game,
    color_config: ColorConfig,
    rect: Rect,
    play: Play,
) {
    let mut player_names: Vec<Span> = Vec::new();
    let mut players_iter = PLAYERS.iter().peekable();
    while let Some(player) = players_iter.next() {
        let name = if players_iter.peek().is_none() {
            game.player_name(*player).to_owned()
        } else {
            format!("{} ", game.player_name(*player))
        };
        let style = if *player != game.current_player() && play != Play::Finished {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default().fg(color_config.player(*player))
        };
        player_names.push(Span::styled(name, style));
    }
    frame.render_widget(
        Paragraph::new(Spans::from(player_names))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Player")),
        rect,
    );
}

fn render_position_block<B: Backend>(frame: &mut Frame<B>, game: &Game, rect: Rect) {
    let (x, y) = game.current_position();
    frame.render_widget(
        Paragraph::new(format!("{}, {}", x, y))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Position")),
        rect,
    );
}

fn render_message_block<B: Backend>(frame: &mut Frame<B>, game: &Game, rect: Rect) {
    let mut text = game.message().text.clone();
    if game.current_status() == Status::Play(Play::Skipped) {
        write!(
            text,
            " Press [{}].",
            key_binding::change_key_to_str(key_binding::key::SELECT)
        )
        .unwrap();
    }
    let color = match game.message().kind {
        MessageKind::Normal => Color::Reset,
        MessageKind::Error => Color::Red,
    };
    frame.render_widget(
        Paragraph::new(Span::styled(text, Style::default().fg(color)))
            .block(Block::default().borders(Borders::ALL).title("Message")),
        rect,
    );
}
