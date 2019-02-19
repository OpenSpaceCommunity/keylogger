use std::ops::Range;
use std::collections::VecDeque;

use chrono::{NaiveDateTime, DateTime, Local, Utc};
use log::debug;
use user32::{self, winuser};
use kernel32;

use crate::{Config, system::{PressEvent, Key}};

pub fn init() {}

type VkCode = usize;

#[derive(Debug, Copy, Clone)]
struct Vk(VkCode);

impl Vk {
    const MAX_KEY_CODE: VkCode = 190;

    fn range() -> Range<VkCode> {
        8..Self::MAX_KEY_CODE
    }

    fn key(&self) -> &str {
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
            _ => Key::UNKNOWN,
        }
    }

    fn is_pressed(&self) -> bool {
        unsafe { user32::GetAsyncKeyState(self.0 as i32) } == -32767
    }
}

pub struct InputDevice {
    pressed_keys: [bool; Vk::MAX_KEY_CODE],
    events: VecDeque<(PressEvent, Vk, DateTime<Local>)>,
}

impl InputDevice {
    pub fn new(config: &Config) -> Self {
        Self {
            pressed_keys: [false; Vk::MAX_KEY_CODE],
            events: VecDeque::new(),
        }
    }

    pub fn check_key_event(&mut self) -> Option<(PressEvent, &str, DateTime<Local>)> {
        for i in Vk::range() {
            let vk = Vk(i);
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