use std::ops::Range;
use std::collections::VecDeque;

use chrono::{DateTime, Local};
use winapi::{um::winuser};
use user32;
use kernel32;

use crate::{Config, system::{PressEvent, Key, WinKey}};

pub fn init() {
//    unsafe { kernel32::FreeConsole() };
}

type VkCode = usize;

#[derive(Debug, Copy, Clone)]
struct Vk {
    code: VkCode,
    index: usize,
}

impl Vk {
    const MAX_KEY_CODE: VkCode = 222;

    fn new(code: VkCode, index: usize) -> Self {
        Self {
            code,
            index,
        }
    }

    fn range() -> Range<VkCode> {
        8..Self::MAX_KEY_CODE
    }

    fn key(&self) -> String {
        match self.code as i32 {
            0x30...0x39 | 0x41...0x5A
            | winuser::VK_MULTIPLY
            | winuser::VK_ADD | winuser::VK_OEM_PLUS
            | winuser::VK_SEPARATOR
            | winuser::VK_SUBTRACT | winuser::VK_OEM_MINUS
            | winuser::VK_DECIMAL | winuser::VK_OEM_PERIOD
            | winuser::VK_DIVIDE
            | winuser::VK_OEM_1
            | winuser::VK_OEM_COMMA
            | winuser::VK_OEM_2
            | winuser::VK_OEM_3
            | winuser::VK_OEM_4
            | winuser::VK_OEM_5
            | winuser::VK_OEM_6
            | winuser::VK_OEM_7 => format!("K{}", self.index),
            _ => match self.code as i32 {
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
                winuser::VK_SLEEP => WinKey::SLEEP,
                _ => Key::UNKNOWN,
            }.to_string(),
        }
    }

    fn is_pressed(code: VkCode) -> bool {
        (unsafe { user32::GetAsyncKeyState(code as i32) } as u16 & 0x8000 != 0)
    }

    fn is_used(code: VkCode) -> bool {
        match code as i32 {
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

struct KeyPressOrder {
    pressed_keys: [bool; Vk::MAX_KEY_CODE],
    order: [VkCode; Vk::MAX_KEY_CODE],
}

impl KeyPressOrder {
    const NONE: VkCode = 0;

    fn new() -> Self {
        Self {
            pressed_keys: [false; Vk::MAX_KEY_CODE],
            order: [Self::NONE; Vk::MAX_KEY_CODE],
        }
    }

    fn press(&mut self, code: VkCode) -> usize {
        self.pressed_keys[code] = true;
        for i in 0..self.order.len() {
            if self.order[i] == Self::NONE {
                self.order[i] = code;
                return i;
            }
        }
        Self::NONE
    }

    fn release(&mut self, code: VkCode) -> usize {
        self.pressed_keys[code] = false;
        for i in 0..self.order.len() {
            if self.order[i] == code {
                self.order[i] = Self::NONE;
                return i;
            }
        }
        Self::NONE
    }

    fn is_already_pressed(&self, code: VkCode) -> bool {
        self.pressed_keys[code]
    }
}

pub struct InputDevice {
    order: KeyPressOrder,
    events: VecDeque<(PressEvent, Vk, DateTime<Local>)>,
    current_key: String,
    sleep_millis: u32,
}

impl InputDevice {
    pub fn new(_config: &Config) -> Self {
        Self {
            order: KeyPressOrder::new(),
            events: VecDeque::new(),
            current_key: String::new(),
            sleep_millis: 1,
        }
    }

    pub fn check_key_event(&mut self) -> Option<(PressEvent, &str, DateTime<Local>)> {
        for code in Vk::range() {
            if Vk::is_used(code) {
                if Vk::is_pressed(code) {
                    if !self.order.is_already_pressed(code) {
                        let index = self.order.press(code);
                        self.events.push_back((PressEvent::Press, Vk::new(code, index), Local::now()));
                    }
                } else {
                    if self.order.is_already_pressed(code) {
                        let index = self.order.release(code);
                        self.events.push_back((PressEvent::Release, Vk::new(code, index), Local::now()));
                    }
                }
            }
        }
        if let Some((event, vk, datetime)) = self.events.pop_front() {
            self.current_key = vk.key();
            Some((event, self.current_key.as_str(), datetime))
        } else {
            None
        }
    }

    pub fn sleep(&self) {
        if self.events.is_empty() {
            unsafe { kernel32::SleepEx(self.sleep_millis, 1); }
        }
    }
}