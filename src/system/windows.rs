use std::ops::Range;
use std::collections::VecDeque;

use chrono::{DateTime, Local};
use winapi::{um::winuser};
use user32;
use kernel32;

use crate::{Config, system::{PressEvent, Key, WinKey}};

pub fn init() {}

type VkCode = usize;

#[derive(Debug, Copy, Clone)]
struct Vk(VkCode);

impl Vk {
    const MAX_KEY_CODE: VkCode = 222;

    fn range() -> Range<VkCode> {
        8..Self::MAX_KEY_CODE
    }

    fn key(&self) -> &'static str {
        match self.0 as i32 {
            winuser::VK_ESCAPE => Key::ESC,
            winuser::VK_BACK => Key::BACKSPACE,
            winuser::VK_TAB => Key::TAB,
            winuser::VK_RETURN => Key::ENTER,
            winuser::VK_LCONTROL => Key::LCTRL,
            winuser::VK_RCONTROL => Key::RCTRL,
            winuser::VK_LSHIFT => Key::LSHIFT,
            winuser::VK_RSHIFT => Key::RSHIFT,
            winuser::VK_LMENU => Key::LALT,
            winuser::VK_RMENU => Key::RALT,
            winuser::VK_SPACE => Key::SPACE,
            winuser::VK_F1 => Key::F1,
            winuser::VK_F2 => Key::F2,
            winuser::VK_F3 => Key::F3,
            winuser::VK_F4 => Key::F4,
            winuser::VK_F5 => Key::F5,
            winuser::VK_F6 => Key::F6,
            winuser::VK_F7 => Key::F7,
            winuser::VK_F8 => Key::F8,
            winuser::VK_F9 => Key::F9,
            winuser::VK_F10 => Key::F10,
            winuser::VK_F11 => Key::F11,
            winuser::VK_F12 => Key::F12,
            winuser::VK_F13 => WinKey::F13,
            winuser::VK_F14 => WinKey::F14,
            winuser::VK_F15 => WinKey::F15,
            winuser::VK_F16 => WinKey::F16,
            winuser::VK_F17 => WinKey::F17,
            winuser::VK_F18 => WinKey::F18,
            winuser::VK_F19 => WinKey::F19,
            winuser::VK_F20 => WinKey::F20,
            winuser::VK_F21 => WinKey::F21,
            winuser::VK_F22 => WinKey::F22,
            winuser::VK_F23 => WinKey::F23,
            winuser::VK_F24 => WinKey::F24,
            winuser::VK_UP => Key::UP,
            winuser::VK_LEFT => Key::LEFT,
            winuser::VK_RIGHT => Key::RIGHT,
            winuser::VK_DOWN => Key::DOWN,
            winuser::VK_HOME => Key::HOME,
            winuser::VK_END => Key::END,
            winuser::VK_PRIOR => Key::PAGE_UP,
            winuser::VK_NEXT => Key::PAGE_DOWN,
            winuser::VK_INSERT => Key::INSERT,
            winuser::VK_DELETE => Key::DELETE,
            winuser::VK_SNAPSHOT => WinKey::PRINT_SCREEN,
            winuser::VK_PAUSE => WinKey::PAUSE,
            winuser::VK_CAPITAL => Key::CAPS_LOCK,
            winuser::VK_SCROLL => Key::SCROL_LOCK,
            winuser::VK_NUMLOCK => Key::NUM_LOCK,
            winuser::VK_NUMPAD0 => Key::KP0,
            winuser::VK_NUMPAD1 => Key::KP1,
            winuser::VK_NUMPAD2 => Key::KP2,
            winuser::VK_NUMPAD3 => Key::KP3,
            winuser::VK_NUMPAD4 => Key::KP4,
            winuser::VK_NUMPAD5 => Key::KP5,
            winuser::VK_NUMPAD6 => Key::KP6,
            winuser::VK_NUMPAD7 => Key::KP7,
            winuser::VK_NUMPAD8 => Key::KP8,
            winuser::VK_NUMPAD9 => Key::KP9,
            winuser::VK_LWIN => WinKey::LWIN,
            winuser::VK_RWIN => WinKey::RWIN,
            winuser::VK_CLEAR => WinKey::CLEAR,
            0x30 => "0",
            0x31 => "1",
            0x32 => "2",
            0x33 => "3",
            0x34 => "4",
            0x35 => "5",
            0x36 => "6",
            0x37 => "7",
            0x38 => "8",
            0x39 => "9",
            0x41 => "A",
            0x42 => "B",
            0x43 => "C",
            0x44 => "D",
            0x45 => "E",
            0x46 => "F",
            0x47 => "G",
            0x48 => "H",
            0x49 => "I",
            0x4A => "J",
            0x4B => "K",
            0x4C => "L",
            0x4D => "M",
            0x4E => "N",
            0x4F => "O",
            0x50 => "P",
            0x51 => "Q",
            0x52 => "R",
            0x53 => "S",
            0x54 => "T",
            0x55 => "U",
            0x56 => "V",
            0x57 => "W",
            0x58 => "X",
            0x59 => "Y",
            0x5A => "Z",
            winuser::VK_SLEEP => WinKey::SLEEP,
            winuser::VK_MULTIPLY => "*",
            winuser::VK_ADD | winuser::VK_OEM_PLUS => "+",
            winuser::VK_SEPARATOR => "\\",
            winuser::VK_SUBTRACT | winuser::VK_OEM_MINUS => "-",
            winuser::VK_DECIMAL | winuser::VK_OEM_PERIOD => ".",
            winuser::VK_DIVIDE => "/",
            winuser::VK_OEM_1 => ":;",
            winuser::VK_OEM_COMMA => ",",
            winuser::VK_OEM_2 => "/?",
            winuser::VK_OEM_3 => "`~",
            winuser::VK_OEM_4 => "[{",
            winuser::VK_OEM_5 => "\\|",
            winuser::VK_OEM_6 => "]}",
            winuser::VK_OEM_7 => "'\"",
            _ => Key::UNKNOWN,
        }
    }

    fn is_pressed(&self) -> bool {
        (unsafe { user32::GetAsyncKeyState(self.0 as i32) } as u16 & 0x8000 != 0)
    }

    fn is_used(&self) -> bool {
        match self.0 as i32 {
            0...7 => false,
            // Because used VK_L../VK_R.. versions
            winuser::VK_CONTROL
            | winuser::VK_SHIFT
            | winuser::VK_MENU => false,
            0xA6...0xB9 => false,
            _ => true,
        }
    }
}

pub struct InputDevice {
    pressed_keys: [bool; Vk::MAX_KEY_CODE],
    events: VecDeque<(PressEvent, Vk, DateTime<Local>)>,
}

impl InputDevice {
    pub fn new(_config: &Config) -> Self {
        Self {
            pressed_keys: [false; Vk::MAX_KEY_CODE],
            events: VecDeque::new(),
        }
    }

    pub fn check_key_event(&mut self) -> Option<(PressEvent, &str, DateTime<Local>)> {
        for i in Vk::range() {
            let vk = Vk(i);
            if vk.is_used() {
                if vk.is_pressed() {
                    if !self.pressed_keys[i] {
                        self.pressed_keys[i] = true;
                        self.events.push_back((PressEvent::Press, vk, Local::now()));
                    }
                } else {
                    if self.pressed_keys[i] {
                        self.pressed_keys[i] = false;
                        self.events.push_back((PressEvent::Release, vk, Local::now()));
                    }
                }
            }
        }
        if let Some((event, vk, datetime)) = self.events.pop_front() {
            Some((event, vk.key(), datetime))
        } else {
            None
        }
    }

    pub fn sleep(&self) {
        if self.events.is_empty() {
            unsafe { kernel32::SleepEx(1, 1); }
        }
    }
}