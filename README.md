# BlogOS

Rust implementation of Phill Opp's BlogOS.
Based on his series of [blog posts](https://os.phil-opp.com/).

## Installing

The steps required to install and run this project for development are this would be:

* Install `qemu`
* Install `rust`
* Enable nightly rust version
    * `rustup override set nightly`
* Add the `rust-src` toolchain component
    * `rustup component add rust-src`
* Add the `llvm-tools-preview` toolchain component
    * `rustup component add llvm-tools-preview`
* Install the `bootimage` binary
    * `cargo install bootimage`

With this you should be able to run with `cargo run`, which should open a
qemu window with the emulated target running the OS.
