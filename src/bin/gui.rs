// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! egui/eframe GUI entry point, for both native and WebAssembly targets.

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    triversi::gui_app::run_native()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    triversi::gui_app::run_wasm();
}
