pub mod linux;
pub mod windows;

pub use self::linux::*;

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