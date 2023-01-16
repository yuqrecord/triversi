// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use termion::event::Key;

#[cfg(feature = "alternative_key_binding")]
pub use alternative as key;
#[cfg(not(feature = "alternative_key_binding"))]
pub use default as key;

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
    pub const ZOOM_IN: Key = Key::Char('+');
    pub const ZOOM_OUT: Key = Key::Char('-');
    pub const QUIT: Key = Key::Char('q');
    pub const INIT: Key = Key::Char('0');
    pub const SELECT: Key = Key::Char('\n');
}

pub fn make_guidance_in_turn() -> String {
    format!(" Quit [{}], Initialize [{}], Select [{}], Move ◀︎/▼/▲/▶︎ [{}/{}/{}/{}], Scroll ◀︎/▼/▲/▶︎/reset [{}/{}/{}/{}/{}], Zoom In/Out [{}/{}], Frame On/Off [{}]",
        change_key_to_str(key::QUIT),
        change_key_to_str(key::INIT),
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
        change_key_to_str(key::FRAME_TOGGLE),
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
