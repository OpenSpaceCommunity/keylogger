// Constants, structs, and arrays derived from /linux/include/linux/input.h
use log::debug;
use crate::system::Key;

const MAX_KEYS: u16 = 112;

const EV_KEY: u16 = 1;

const KEY_RELEASE: i32 = 0;
const KEY_PRESS: i32 = 1;

const KEY_LEFTSHIFT: u16 = 42;
const KEY_RIGHTSHIFT: u16 = 43;

#[derive(Debug)]
#[repr(C)]
pub struct InputEvent {
    pub tv_sec: isize, // from timeval struct
    pub tv_usec: isize, // from timeval struct
    pub type_: u16,
    pub code: u16,
    pub value: i32
}

const KEY_NAMES: [&'static str; MAX_KEYS as usize] = [
    Key::UK, Key::ESC,
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=",
    Key::BACKSPACE, Key::TAB,
    "q", "w", "e", "r", "t", "y", "u", "i", "o", "p",
    "[", "]", Key::ENTER, Key::LCTRL,
    "a", "s", "d", "f", "g", "h", "j", "k", "l", ";",
    "'", "`", Key::LSHIFT,
    "\\", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/",
    Key::RSHIFT,
    Key::KP_STAR,
    Key::LALT, Key::SPACE, Key::CAPS_LOCK,
    Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9, Key::F10,
    Key::NUM_LOCK, Key::SCROL_LOCK,
    Key::KP7, Key::KP8, Key::KP9,
    Key::KP_SUB,
    Key::KP4, Key::KP5, Key::KP6,
    Key::KP_ADD,
    Key::KP1, Key::KP2, Key::KP3, Key::KP0,
    Key::KP_POINT,
    Key::UK, Key::UK, Key::UK,
    Key::F11, Key::F12,
    Key::UK, Key::UK, Key::UK, Key::UK, Key::UK, Key::UK, Key::UK,
    Key::KP_ENTER, Key::RCTRL, Key::KP_DIV, Key::SYS_RQ, Key::RALT, Key::UK,
    Key::HOME, Key::UP, Key::PAGE_UP, Key::LEFT, Key::RIGHT, Key::END, Key::DOWN,
    Key::PAGE_DOWN, Key::INSERT, Key::DELETE
];

const SHIFT_KEY_NAMES: [&'static str; MAX_KEYS as usize] = [
    Key::UK, Key::ESC,
    "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+",
    Key::BACKSPACE, Key::TAB,
    "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P",
    "{", "}", Key::ENTER, Key::LCTRL,
    "A", "S", "D", "F", "G", "H", "J", "K", "L", ":",
    "\"", "~", Key::LSHIFT,
    "|", "Z", "X", "C", "V", "B", "N", "M", "<", ">", "?",
    Key::RSHIFT,
    Key::KP_STAR,
    Key::LALT, Key::SPACE, Key::CAPS_LOCK,
    Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9, Key::F10,
    Key::NUM_LOCK, Key::SCROL_LOCK,
    Key::KP7, Key::KP8, Key::KP9,
    Key::KP_SUB,
    Key::KP4, Key::KP5, Key::KP6,
    Key::KP_ADD,
    Key::KP1, Key::KP2, Key::KP3, Key::KP0,
    Key::KP_POINT,
    Key::UK, Key::UK, Key::UK,
    Key::F11, Key::F12,
    Key::UK, Key::UK, Key::UK, Key::UK, Key::UK, Key::UK, Key::UK,
    Key::KP_ENTER, Key::RCTRL, Key::KP_DIV, Key::SYS_RQ, Key::RALT, Key::UK,
    Key::HOME, Key::UP, Key::PAGE_UP, Key::LEFT, Key::RIGHT, Key::END, Key::DOWN,
    Key::PAGE_DOWN, Key::INSERT, Key::DELETE
];

// Converts a key code to it's ascii representation. Some unprintable keys like escape are printed
// as a name between angled brackets, i.e. <ESC>
pub fn get_key_text(code: u16, shift_pressed: u8) -> &'static str {
    let arr = if shift_pressed != 0 {
        SHIFT_KEY_NAMES
    } else {
        KEY_NAMES
    };

    if code < MAX_KEYS {
        return arr[code as usize];
    } else {
        debug!("Unknown key: {}", code);
        return Key::UK;
    }
}

// Determines whether the given key code is a shift
pub fn is_shift(code: u16) -> bool {
    code == KEY_LEFTSHIFT || code == KEY_RIGHTSHIFT
}

pub fn is_key_event(type_: u16) -> bool {
    type_ == EV_KEY
}

pub fn is_key_press(value: i32) -> bool {
    value == KEY_PRESS
}

pub fn is_key_release(value: i32) -> bool {
    value == KEY_RELEASE
}
