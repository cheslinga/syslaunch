# Syslaunch
A simple init system designed for Linux. (Still very much a work in progress.)

## Build Instructions
I like to statically link this program, so I'll usually run:
```
cargo build --release --target x86_64-unknown-linux-musl
```
Since targeting musl statically links your Rust binary.

## Usage
The program just launches a program with a hard-coded path right now, but it can also handle shutdown/reboot using kill signals. The program is set to handle the following kill signals:
- SIGINT: Runs a sync and powers off the machine.
- SIGUSR1: Runs a sync and reboots the machine.
- SIGALRM: Just prints something out to the console.
