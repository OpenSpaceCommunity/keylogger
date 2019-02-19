mod system;

use std::process::exit;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::env;

use getopts::Options;
use chrono::{DateTime, Datelike, Timelike, Local};
use log::debug;
use system::InputDevice;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Config {
    pub device_file: Option<String>,
    pub log_file: Option<String>
}

impl Config {
    fn new(device_file: Option<String>, log_file: Option<String>) -> Self {
        Config {
            device_file,
            log_file
        }
    }
}

fn main() {
    system::init();
    env_logger::init();

    let config = parse_args();
    debug!("Config: {:?}", config);

    let log_name = log_name_from(&Local::now());
    let mut log_file = if let Some(ref name) = config.log_file {
        open_log_file(name)
    } else {
        open_log_file(&log_name)
    };

    let mut input = InputDevice::new(&config);
    loop {
        if let Some((press, key, datetime)) = input.check_key_event() {
            let time = datetime.hour() * 60 * 60 * 1000 + datetime.minute() * 60 * 1000 + datetime.second() * 1000 + datetime.nanosecond() / 1_000_000;
            let text = format!(
                "{:08} {} {}\n",
                time, press.as_mark(), key
            );

            if config.log_file.is_none() {
                let current_log_name = log_name_from(&datetime);
                if log_name != current_log_name {
                    log_file = open_log_file(current_log_name);
                }
            }

            let num_bytes = log_file.write(text.as_bytes())
                .expect("Can't write to log file");
            log_file.flush()
                .expect("Can't flush to log file");
            if num_bytes != text.len() {
                panic!("Error while writing to log file");
            }

            input.sleep();
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

fn log_name_from(datetime: &DateTime<Local>) -> String {
    format!("{:04}-{:02}-{:02}.log", datetime.year(), datetime.month(), datetime.day())
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
    opts.optopt("d", "device", "specify the device file (for Linux systems only)", "DEVICE");
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

    let device_file = matches.opt_str("d");
    let log_file = matches.opt_str("f");

    Config::new(device_file, log_file)
}