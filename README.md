# rust86

This is a partial 8086 emulator, written in Rust.

Much of the 8086's instruction set is implemented, but interrupts, IO, floating point, and segments are not. The subset of the 8086 supported is roughly enough to run `codegolf.asm` from [This stackexchange code golf challenge](http://codegolf.stackexchange.com/questions/4732/emulate-an-intel-8086-cpu).

rust86 is purely a for-fun project, of course. Its main purpose was to keep me up to date with the changing Rust language until 1.0.0-alpha was frozen.

## Build & run

    git clone https://github.com/ianpreston/rust86.git
    cargo build

    nasm -f bin asm/hello.asm -o hello.bin
    ./target/rust86 hello.bin

## License

rust86 is licensed under the [WTFPL](http://www.wtfpl.net/about/).
