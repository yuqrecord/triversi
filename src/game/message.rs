// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

/// Semantic severity of a message, decoupled from any concrete color.
/// The view layer maps each kind to its own presentation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MessageKind {
    #[default]
    Normal,
    Error,
}

/// A user-facing message produced by the game core.
#[derive(Clone, Debug, Default)]
pub struct Message {
    pub text: String,
    pub kind: MessageKind,
}

impl Message {
    pub fn clear(&mut self) {
        self.text.clear();
        self.kind = MessageKind::Normal;
    }
}
