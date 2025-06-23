#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use my_os::{allocator, memory, println};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    println!("Hello World! This is my {}\n", "operating system");
    
    my_os::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    
    use my_os::memory::BootInfoFrameAllocator;
    let mut frame_alloc = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_alloc)
        .expect("Couldn't init kernel heap");

    #[cfg(test)]
    test_main();

    my_os::hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    my_os::hlt_loop();
}

/* #[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
} */