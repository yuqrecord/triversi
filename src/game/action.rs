// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

/// An abstract game command, independent of any input device or UI backend.
/// A view layer (TUI, GUI, ...) translates its own input events into these.
///
/// View-only concerns such as scrolling, zooming, and frame toggling are
/// deliberately not represented here: they belong to the view layer, not the
/// game state machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Select,
    /// Select a specific board position directly (e.g. a mouse click on a
    /// cell), without first moving the cursor there step by step.
    SelectAt((usize, usize)),
    RequestInit,
    RequestQuit,
    Confirm,
    Cancel,
    EnterHistory,
    HistoryPrev,
    HistoryNext,
}
