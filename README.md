# rust86

This is an 8086 emulator, written in Rust. The goal is to eventually support everything the actual chips did back in the day. Currently, only a small subset of the original 8086 functionality is supported; roughly enough to print hello world.

rust86 is a purely a for-fun project, of course. The only practical application is to keep me up to date with the changing Rust spec.

## Build & run

    git clone https://github.com/ianpreston/rust86.git
    cargo build
    ./target/rust86

## License

rust86 is licensed under the [WTFPL](http://www.wtfpl.net/about/).
