// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::game::Action;
use termion::event::Key;

#[cfg(feature = "alternative_key_binding")]
pub use alternative as key;
#[cfg(not(feature = "alternative_key_binding"))]
pub use default as key;

/// View-only commands handled by the TUI layer (not the game core).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ViewAction {
    ScrollLeft,
    ScrollRight,
    ScrollUp,
    ScrollDown,
    ScrollReset,
    ZoomIn,
    ZoomOut,
    ToggleFrame,
}

/// Map a key pressed in an Ask dialog to a game action: `Y` confirms,
/// everything else cancels (preserving the original behavior).
pub fn confirmation_action(key: Key) -> Action {
    if key == Key::Char('Y') {
        Action::Confirm
    } else {
        Action::Cancel
    }
}

/// Map a key to a view-only action, if any.
pub fn view_action(key: Key) -> Option<ViewAction> {
    Some(match key {
        key::SCROLL_LEFT => ViewAction::ScrollLeft,
        key::SCROLL_RIGHT => ViewAction::ScrollRight,
        key::SCROLL_UP => ViewAction::ScrollUp,
        key::SCROLL_DOWN => ViewAction::ScrollDown,
        key::SCROLL_RESET => ViewAction::ScrollReset,
        key::ZOOM_IN => ViewAction::ZoomIn,
        key::ZOOM_OUT => ViewAction::ZoomOut,
        key::FRAME_TOGGLE => ViewAction::ToggleFrame,
        _ => return None,
    })
}

/// Map a key to an abstract game action, if any.
pub fn game_action(key: Key) -> Option<Action> {
    Some(match key {
        key::QUIT => Action::RequestQuit,
        key::INIT => Action::RequestInit,
        key::MOVE_LEFT => Action::MoveLeft,
        key::MOVE_RIGHT => Action::MoveRight,
        key::MOVE_UP => Action::MoveUp,
        key::MOVE_DOWN => Action::MoveDown,
        key::INTO_HISTORY => Action::EnterHistory,
        key::SELECT => Action::Select,
        key::PREV_HISTORY => Action::HistoryPrev,
        key::NEXT_HISTORY => Action::HistoryNext,
        _ => return None,
    })
}

#[cfg(not(feature = "alternative_key_binding"))]
pub mod default {
    use termion::event::Key;
    pub const MOVE_UP: Key = Key::Char('k');
    pub const MOVE_DOWN: Key = Key::Char('j');
    pub const MOVE_LEFT: Key = Key::Char('h');
    pub const MOVE_RIGHT: Key = Key::Char('l');
    pub const SCROLL_UP: Key = Key::Up;
    pub const SCROLL_DOWN: Key = Key::Down;
    pub const SCROLL_LEFT: Key = Key::Left;
    pub const SCROLL_RIGHT: Key = Key::Right;
    pub const SCROLL_RESET: Key = Key::Home;
    pub const FRAME_TOGGLE: Key = Key::Char('f');
    pub const INTO_HISTORY: Key = Key::Char('t');
    pub const PREV_HISTORY: Key = Key::Char('p');
    pub const NEXT_HISTORY: Key = Key::Char('n');
    pub const ZOOM_IN: Key = Key::Char('+');
    pub const ZOOM_OUT: Key = Key::Char('-');
    pub const QUIT: Key = Key::Char('q');
    pub const INIT: Key = Key::Char('0');
    pub const SELECT: Key = Key::Char('\n');
}

#[cfg(feature = "alternative_key_binding")]
pub mod alternative {
    use termion::event::Key;
    pub const MOVE_UP: Key = Key::Char('i');
    pub const MOVE_DOWN: Key = Key::Char('k');
    pub const MOVE_LEFT: Key = Key::Char('j');
    pub const MOVE_RIGHT: Key = Key::Char('l');
    pub const SCROLL_UP: Key = Key::Up;
    pub const SCROLL_DOWN: Key = Key::Down;
    pub const SCROLL_LEFT: Key = Key::Left;
    pub const SCROLL_RIGHT: Key = Key::Right;
    pub const SCROLL_RESET: Key = Key::Home;
    pub const FRAME_TOGGLE: Key = Key::Char('f');
    pub const INTO_HISTORY: Key = Key::Char('h');
    pub const PREV_HISTORY: Key = Key::Char('p');
    pub const NEXT_HISTORY: Key = Key::Char('n');
    pub const ZOOM_IN: Key = Key::Char('+');
    pub const ZOOM_OUT: Key = Key::Char('-');
    pub const QUIT: Key = Key::Char('q');
    pub const INIT: Key = Key::Char('0');
    pub const SELECT: Key = Key::Char('\n');
}

pub fn make_guidance_in_turn() -> String {
    format!(" Quit [{}], Initialize [{}], History [{}], Frame On/Off [{}], Select [{}]\n Move ◀︎/▼/▲/▶︎ [{}/{}/{}/{}], Scroll ◀︎/▼/▲/▶︎/reset [{}/{}/{}/{}/{}], Zoom In/Out [{}/{}]",
        change_key_to_str(key::QUIT),
        change_key_to_str(key::INIT),
        change_key_to_str(key::INTO_HISTORY),
        change_key_to_str(key::FRAME_TOGGLE),
        change_key_to_str(key::SELECT),
        change_key_to_str(key::MOVE_LEFT),
        change_key_to_str(key::MOVE_DOWN),
        change_key_to_str(key::MOVE_UP),
        change_key_to_str(key::MOVE_RIGHT),
        change_key_to_str(key::SCROLL_LEFT),
        change_key_to_str(key::SCROLL_DOWN),
        change_key_to_str(key::SCROLL_UP),
        change_key_to_str(key::SCROLL_RIGHT),
        change_key_to_str(key::SCROLL_RESET),
        change_key_to_str(key::ZOOM_IN),
        change_key_to_str(key::ZOOM_OUT),
    )
}
pub fn make_guidance_in_history() -> String {
    format!(" Frame On/Off [{}], Select [{}]\n Prev/Next [{}/{}], Scroll ◀︎/▼/▲/▶︎/reset [{}/{}/{}/{}/{}], Zoom In/Out [{}/{}]",
        change_key_to_str(key::FRAME_TOGGLE),
        change_key_to_str(key::SELECT),
        change_key_to_str(key::PREV_HISTORY),
        change_key_to_str(key::NEXT_HISTORY),
        change_key_to_str(key::SCROLL_LEFT),
        change_key_to_str(key::SCROLL_DOWN),
        change_key_to_str(key::SCROLL_UP),
        change_key_to_str(key::SCROLL_RIGHT),
        change_key_to_str(key::SCROLL_RESET),
        change_key_to_str(key::ZOOM_IN),
        change_key_to_str(key::ZOOM_OUT),
        )
}

pub fn change_key_to_str(key: Key) -> String {
    match key {
        Key::Char('\n') => "Enter".into(),
        Key::Char('\t') => "Tab".into(),
        Key::Char(c) => c.into(),
        Key::Alt('\n') => "Alt-Enter".into(),
        Key::Alt('\t') => "Alt-Tab".into(),
        Key::Alt(c) => format!("Alt-{}", c),
        Key::Ctrl('\n') => "Ctrl-Enter".into(),
        Key::Ctrl('\t') => "Ctrl-Tab".into(),
        Key::Ctrl(c) => format!("Ctrl-{}", c),
        Key::F(f) => format!("F{}", f),
        Key::Backspace => "BS".into(),
        Key::Left => "Left".into(),
        Key::Right => "Right".into(),
        Key::Up => "Up".into(),
        Key::Down => "Down".into(),
        Key::Home => "Home".into(),
        Key::End => "End".into(),
        Key::PageUp => "PageUp".into(),
        Key::PageDown => "PageDown".into(),
        Key::BackTab => "BackTab".into(),
        Key::Delete => "Del".into(),
        Key::Insert => "Insert".into(),
        Key::Esc => "Esc".into(),
        _ => unreachable!(),
    }
}
