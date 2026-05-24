// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

pub mod board;
pub mod error;
pub mod game;
pub mod gui_app;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
#[cfg(not(target_arch = "wasm32"))]
pub mod tui_app;
