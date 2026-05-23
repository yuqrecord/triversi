// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

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
