#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use blog_os::allocator;
use blog_os::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use alloc::boxed::Box;
    use blog_os::memory;
    use x86_64::VirtAddr;

    println!("Hello World!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // Initialize the Kernel Heap
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Test the allocator
    let heap_value = Box::new(42);
    println!("heap_value at {heap_value:p}");

    // Create a dynamically sized vector
    // let vec: Vec<_> = (0..500).into_iter().collect();
    // Expanding the loop removes a `with_capacity` optimization, which 
    // resutls in multiple resizes, and a final pointer position that is 
    // different!
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Create a reference counted vector -> will be freed when count reaches 0
    let refcounted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = Rc::clone(&refcounted);
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(refcounted);
    println!("reference count is now {}", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
