mod input;

use std::process::Command;
use std::fs::File;
use std::io::Read;
use std::mem;

use libc;
use chrono::{NaiveDateTime, DateTime, Local, Utc};
use log::debug;

use crate::{Config, system::PressEvent};
use self::input::{is_key_event, is_key_press, is_key_release, is_shift, get_key_text, InputEvent};

pub fn init() {
    root_check();
}

fn root_check() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("Must run as root user");
    }
}

pub struct InputDevice {
    device_file: File,
    // TODO: use the sizeof function (not available yet) instead of hard-coding 24.
    buf: Option<[u8; 24]>,
    // We use a u8 here instead of a bool to handle the rare case when both shift keys are pressed
    // and then one is released
    shift_pressed: u8,
}

impl InputDevice {
    pub fn new(config: &Config) -> Self {
        let device_file = config.device_file.clone().unwrap_or_else(|| get_default_device());
        Self {
            device_file: File::open(&device_file)
                .expect("Can't open device file"),
            buf: Some(unsafe { mem::zeroed() }),
            shift_pressed: 0,
        }
    }

    pub fn check_key_event(&mut self) -> Option<(PressEvent, &str, DateTime<Local>)> {
        let mut buf = self.buf.take().unwrap_or_else(|| unsafe { mem::zeroed() });
        let num_bytes = self.device_file.read(&mut buf)
            .expect("Can't read from device file");
        if num_bytes != mem::size_of::<InputEvent>() {
            panic!("Error while reading from device file");
        }
        let event: InputEvent = unsafe { mem::transmute(buf) };
        let result = if is_key_event(event.type_) {
            let press = if is_key_press(event.value) {
                if is_shift(event.code) {
                    self.shift_pressed += 1;
                }
                Some(PressEvent::Press)
            } else if is_key_release(event.value) {
                if is_shift(event.code) {
                    self.shift_pressed -= 1;
                }
                Some(PressEvent::Release)
            } else {
                None
            };

            press.map(|press| {
                let key = get_key_text(event.code, self.shift_pressed);
                (press, key, convert_time(event.tv_sec, event.tv_usec))
            })
        } else {
            None
        };
        self.buf.replace(buf);
        result
    }
}

fn convert_time(secs: isize, micros: isize) -> DateTime<Local> {
    let naive_datetime = NaiveDateTime::from_timestamp(secs as i64, micros as u32 * 1000);
    DateTime::<Utc>::from_utc(naive_datetime, Utc).with_timezone(&Local)
}

fn get_default_device() -> String {
    let mut filenames = get_keyboard_device_filenames();
    debug!("Detected devices: {:?}", filenames);

    if filenames.len() == 1 {
        filenames.swap_remove(0)
    } else {
        panic!("The following keyboard devices were detected: {:?}. Please select one using \
                the `-d` flag", filenames);
    }
}

// Detects and returns the name of the keyboard device file. This function uses
// the fact that all device information is shown in /proc/bus/input/devices and
// the keyboard device file should always have an EV of 120013
fn get_keyboard_device_filenames() -> Vec<String> {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

    let res = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
        .expect("Can't get keyboard device filenames");
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }
    filenames
}