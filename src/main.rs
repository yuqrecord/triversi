// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! Terminal (TUI) entry point. The GUI lives in the separate `gui` binary.

#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    triversi::cli::Cli::run()
}

#[cfg(target_arch = "wasm32")]
fn main() {}
