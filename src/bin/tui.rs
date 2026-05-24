// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

//! Terminal UI entry point.

fn main() -> anyhow::Result<()> {
    triversi::cli::Cli::run()
}
