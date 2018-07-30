Rust Keylogger
==============

This is a keylogger for Linux written in Rust, forked from [original keylogger](https://github.com/gsingh93/keylogger). It works by reading directly from the keyboard device in `/dev/input/`. The keylogger attempts to detect the keyboard device upon startup, but if one cannot be detected or if multiple are detected, you must specify the path to the device file manually.

Only the US keyboard layout is supported. See [input.rs](https://github.com/gsingh93/keylogger/blob/master/src/input.rs) if you are interested in adding mappings for other keyboard layouts.

## Installation
To install Rust, run the following in your terminal, then follow the onscreen instructions.

```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

Clone the repository:

```
$ git clone https://github.com/OpenSpaceCommunity/keylogger
$ cd keylogger
```

Build the code:

```$ cargo build --release```

You can run the code with Cargo or directly from the target directory. Note that the keylogger must be run as the root user:

```
$ sudo ./target/release/keylogger
$ sudo cargo run --release
```

## Usage

```
$ sudo cargo run -- -h

Usage: target/release/keylogger [options]

Options:
    -h --help           prints this help message
    -v --version        prints the version
    -d --device DEVICE  specify the device file
    -f --file FILE      specify the file to log to
```

If the `-f` flag is not specified, the file `YYYY-MM-DD.log` is used.

If you would like to run the keylogger in the background, append an `&` to the end of the command. If you would like to run the keylogger as a daemon or at startup, use init script/service manager that comes with your distro. An example `systemd` file is provided.

## License

[MIT](https://github.com/OpenSpaceCommunity/keylogger/blob/master/LICENSE.txt)
