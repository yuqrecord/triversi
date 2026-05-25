// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! egui/eframe GUI frontend for Triversi.
//!
//! This layer drives the TUI-independent [`crate::game::Game`] core through
//! abstract [`Action`]s and renders its state with egui. It builds for both
//! native (via [`run_native`]) and WebAssembly (via [`run_wasm`]) targets.

use crate::board::{Board, Player, PLAYERS};
use crate::game::{Action, Game, MessageKind, Play, Status};
use eframe::egui;
use std::collections::HashSet;

const DEFAULT_RANGE: usize = 14;
const DEFAULT_PLAYER_NAMES: &str = "1,2,3";

pub struct TriversiApp {
    game: Game,
    show_legal: bool,
}

impl TriversiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self {
            game: new_game(),
            show_legal: false,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn reset(&mut self) {
        self.game = new_game();
    }

    fn ui_top(&mut self, ui: &mut egui::Ui) {
        ui.heading("Triversi");
        ui.horizontal(|ui| {
            ui.label("Turn:");
            let player = self.game.current_player();
            let finished = self.game.current_status() == Status::Play(Play::Finished);
            if finished {
                ui.label("game finished");
            } else {
                ui.colored_label(player_color(player), self.game.player_name(player));
            }
        });
        ui.horizontal(|ui| {
            for &player in PLAYERS {
                let score = self.game.board().count().get(player);
                ui.colored_label(
                    player_color(player),
                    format!("Player-{}: {}", self.game.player_name(player), score),
                );
                ui.separator();
            }
        });
    }

    fn ui_bottom(&mut self, ui: &mut egui::Ui) {
        let kind = self.game.message().kind;
        let text = self.game.message().text.clone();
        let color = match kind {
            MessageKind::Normal => ui.visuals().text_color(),
            MessageKind::Error => egui::Color32::from_rgb(230, 80, 80),
        };
        ui.colored_label(color, text.trim());
        ui.separator();
        self.ui_controls(ui);
    }

    fn ui_controls(&mut self, ui: &mut egui::Ui) {
        let Status::Play(play) = self.game.current_status() else {
            return;
        };
        ui.horizontal(|ui| {
            ui.toggle_value(&mut self.show_legal, "Show legal moves");
            ui.separator();
            if ui.button("Initialize").clicked() {
                self.game.dispatch(Action::RequestInit);
            }
            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("Quit").clicked() {
                self.game.dispatch(Action::RequestQuit);
            }
            match play {
                Play::History => {
                    if ui.button("◀ Prev").clicked() {
                        self.game.dispatch(Action::HistoryPrev);
                    }
                    if ui.button("Next ▶").clicked() {
                        self.game.dispatch(Action::HistoryNext);
                    }
                    if ui.button("Back to game").clicked() {
                        self.game.dispatch(Action::Select);
                    }
                }
                Play::Skipped => {
                    if ui.button("Continue (skipped)").clicked() {
                        self.game.dispatch(Action::Select);
                    }
                    if ui.button("History").clicked() {
                        self.game.dispatch(Action::EnterHistory);
                    }
                }
                _ => {
                    if ui.button("History").clicked() {
                        self.game.dispatch(Action::EnterHistory);
                    }
                }
            }
        });
    }

    fn ui_board(&mut self, ui: &mut egui::Ui) {
        let range = self.game.board().range();
        let current_player = self.game.current_player();
        let status = self.game.current_status();
        let legal: HashSet<(usize, usize)> = if self.show_legal && status == Status::Play(Play::Turn) {
            self.game.availables().get(current_player).keys().copied().collect()
        } else {
            HashSet::new()
        };

        const BOARD_MARGIN: f32 = 20.0;
        ui.add_space(BOARD_MARGIN);
        let size = egui::vec2(ui.available_width(), ui.available_height() - BOARD_MARGIN);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click());
        let rect = response.rect;
        // Shrink the layout rect by the cursor stroke half-width so the stroke
        // is never clipped by the painter boundary.
        let layout = CellLayout::new(rect.shrink(3.0), range);

        // Cell currently under the mouse pointer.
        let hovered_cell = response
            .hover_pos()
            .and_then(|pointer| layout.cell_at(pointer, range));

        for y in 0..range {
            for x in 0..=y {
                let center = layout.cell_center(x, y);
                let cell_player = self.game.board().player((x, y));
                painter.circle_filled(center, layout.radius, egui::Color32::from_gray(45));
                if let Some(player) = cell_player {
                    painter.circle_filled(center, layout.radius * 0.8, player_color(player));
                }
                if legal.contains(&(x, y)) {
                    painter.circle_stroke(
                        center,
                        layout.radius * 0.92,
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(90, 220, 90)),
                    );
                }
                if hovered_cell == Some((x, y)) {
                    painter.circle_stroke(
                        center,
                        layout.radius,
                        egui::Stroke::new(2.5, egui::Color32::WHITE),
                    );
                }
            }
        }

        if status == Status::Play(Play::Turn)
            && response.clicked()
            && let Some(pointer) = response.interact_pointer_pos()
            && let Some(position) = layout.cell_at(pointer, range)
        {
            self.game.dispatch(Action::SelectAt(position));
        }
    }

    fn ui_ask(&mut self, ctx: &egui::Context) {
        let prompt = match self.game.current_status() {
            Status::AskInit => "Initialize the game?",
            Status::AskQuit => "Quit the game?",
            _ => return,
        };
        egui::Window::new("Confirm")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.label(prompt);
                ui.horizontal(|ui| {
                    if ui.button("Yes").clicked() {
                        self.game.dispatch(Action::Confirm);
                    }
                    if ui.button("No").clicked() {
                        self.game.dispatch(Action::Cancel);
                    }
                });
            });
    }
}

impl eframe::App for TriversiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        egui::Panel::top("triversi_top").show_inside(ui, |ui| self.ui_top(ui));
        egui::Panel::bottom("triversi_bottom").show_inside(ui, |ui| self.ui_bottom(ui));
        egui::CentralPanel::default().show_inside(ui, |ui| self.ui_board(ui));
        self.ui_ask(&ctx);

        if self.game.current_status() == Status::Quit {
            #[cfg(not(target_arch = "wasm32"))]
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            #[cfg(target_arch = "wasm32")]
            self.reset();
        }
    }
}

/// Screen geometry for the triangular board.
struct CellLayout {
    origin: egui::Pos2,
    dx: f32,
    dy: f32,
    radius: f32,
}

impl CellLayout {
    fn new(rect: egui::Rect, range: usize) -> Self {
        // Triangular grid with near-equilateral spacing (dy = dx * sin(60deg)).
        // Fit the whole triangle into `rect`, leaving room for the stone radius.
        const ASPECT: f32 = 0.866; // sin(60deg)
        const RADIUS_RATIO: f32 = 0.42;
        let rows = (range.max(1) - 1) as f32; // gaps between rows
        let pad = 2.0 * RADIUS_RATIO; // stone diameter, in units of dx
        let dx_by_width = rect.width() / (rows + pad);
        let dx_by_height = rect.height() / (rows * ASPECT + pad);
        let dx = dx_by_width.min(dx_by_height).max(1.0);
        let dy = dx * ASPECT;
        let radius = dx * RADIUS_RATIO;
        // Center the triangle within the available rectangle.
        let origin = egui::pos2(rect.center().x, rect.center().y - rows * dy / 2.0);
        Self {
            origin,
            dx,
            dy,
            radius,
        }
    }

    fn cell_center(&self, x: usize, y: usize) -> egui::Pos2 {
        egui::pos2(
            self.origin.x + (x as f32 - y as f32 / 2.0) * self.dx,
            self.origin.y + y as f32 * self.dy,
        )
    }

    fn cell_at(&self, pointer: egui::Pos2, range: usize) -> Option<(usize, usize)> {
        for y in 0..range {
            for x in 0..=y {
                if self.cell_center(x, y).distance(pointer) <= self.radius {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

fn player_color(player: Player) -> egui::Color32 {
    match player {
        Player::Zero => egui::Color32::from_rgb(0, 200, 200),
        Player::One => egui::Color32::from_rgb(220, 80, 220),
        Player::Two => egui::Color32::from_rgb(220, 200, 40),
    }
}

fn new_game() -> Game {
    let board = Board::try_new(DEFAULT_RANGE).expect("default board range is valid");
    Game::try_new(board, DEFAULT_PLAYER_NAMES).expect("default player names are valid")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Triversi",
        options,
        Box::new(|cc| Ok(Box::new(TriversiApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn run_wasm() {
    use eframe::wasm_bindgen::JsCast as _;

    console_error_panic_hook::set_once();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let document = eframe::web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");
        let canvas = document
            .get_element_by_id("triversi_canvas")
            .expect("missing #triversi_canvas element")
            .dyn_into::<eframe::web_sys::HtmlCanvasElement>()
            .expect("#triversi_canvas is not a canvas");
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(TriversiApp::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}
