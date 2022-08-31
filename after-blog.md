# Things to do after the Blog series is over

* Implementing a bootloader
    * remove `bootimage`
    * Make it simple
    * Add multiboot support later
* Improve the VGA text mode driver
* Review if the `test` just works with `no_std` with modern versions of Rust
    * Maybe it was an issue of not using `--bin blog_os`
* Writing to ports
    * remove `x86_64` crate
    * Custom IDT type
* Serial (UART 16550) driver
    * remove `uart_16550` crate
