# Things to do after the Blog series is over

* Implementing a bootloader
    * remove `bootimage`
    * Make it simple
    * Add multiboot support later
* Implementing a spinlock Mutex
    * remove `spin` crate
* Improve the VGA text mode driver
* Review if the `test` just works with `no_std` with modern versions of Rust
    * Maybe it was an issue of not using `--bin blog_os`
* Writing to ports
    * remove `x86_64` crate
    * Custom IDT type
    * Custom ISF type
    * Custom TSS type
* Serial (UART 16550) driver
    * remove `uart_16550` crate
* Custom PIC8259 driver
    * remove `pic8259` crate
* USB support
* Custom keyboard scancodes parsing
    * remove `pc-keyboard` crate
* Custom page table interaction implementation
    * Do not use `x86_64`
    * Implement also TLB handling
* Custom bootloader
    * Implement initial memory paging setup
        * remove `bootloader` crate
