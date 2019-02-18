#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub use self::unix::*;

#[cfg(windows)]
pub use self::windows::*;


#[derive(Copy, Clone)]
pub enum PressEvent {
    Press,
    Release,
}

impl PressEvent {
    pub fn as_mark(&self) -> &str {
        match self {
            PressEvent::Press => "PR",
            PressEvent::Release => "RE",
        }
    }
}

pub struct Key;

impl Key {
    // Unknown key string
    pub const UK: &'static str = "<UK>";

    pub const ESC: &'static str = "<ESC>";
    pub const BACKSPACE: &'static str = "<Backspace>";
    pub const TAB: &'static str = "<Tab>";
    pub const ENTER: &'static str = "<Enter>";
    pub const LCTRL: &'static str = "<LCtrl>";
    pub const RCTRL: &'static str = "<RCtrl>";
    pub const LSHIFT: &'static str = "<LShift>";
    pub const RSHIFT: &'static str = "<RShift>";
    pub const LALT: &'static str = "<LAlt>";
    pub const RALT: &'static str = "<RAlt>";
    pub const SPACE: &'static str = "<Space>";
    pub const F1: &'static str = "<F1>";
    pub const F2: &'static str = "<F2>";
    pub const F3: &'static str = "<F3>";
    pub const F4: &'static str = "<F4>";
    pub const F5: &'static str = "<F5>";
    pub const F6: &'static str = "<F6>";
    pub const F7: &'static str = "<F7>";
    pub const F8: &'static str = "<F8>";
    pub const F9: &'static str = "<F9>";
    pub const F10: &'static str = "<F10>";
    pub const F11: &'static str = "<F11>";
    pub const F12: &'static str = "<F12>";
    pub const UP: &'static str = "<Up>";
    pub const LEFT: &'static str = "<Left>";
    pub const RIGHT: &'static str = "<Right>";
    pub const DOWN: &'static str = "<Down>";
    pub const HOME: &'static str = "<Home>";
    pub const END: &'static str = "<End>";
    pub const PAGE_UP: &'static str = "<PageUp>";
    pub const PAGE_DOWN: &'static str = "<PageDown>";
    pub const INSERT: &'static str = "<Insert>";
    pub const DELETE: &'static str = "<Delete>";
    pub const SYS_RQ: &'static str = "<SysRq>";
    pub const CAPS_LOCK: &'static str = "<CapsLock>";
    pub const SCROL_LOCK: &'static str = "<ScrollLock>";
    pub const NUM_LOCK: &'static str = "<NumLock>";
    pub const KP0: &'static str = "<KP0>";
    pub const KP1: &'static str = "<KP1>";
    pub const KP2: &'static str = "<KP2>";
    pub const KP3: &'static str = "<KP3>";
    pub const KP4: &'static str = "<KP4>";
    pub const KP5: &'static str = "<KP5>";
    pub const KP6: &'static str = "<KP6>";
    pub const KP7: &'static str = "<KP7>";
    pub const KP8: &'static str = "<KP8>";
    pub const KP9: &'static str = "<KP9>";
    pub const KP_STAR: &'static str = "<KP*>";
    pub const KP_DIV: &'static str = "<KP/>";
    pub const KP_ADD: &'static str = "<KP+>";
    pub const KP_SUB: &'static str = "<KP->";
    pub const KP_POINT: &'static str = "<KP.>";
    pub const KP_ENTER: &'static str = "<KPEnter>";
}