#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::{PanicInfo};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> !
{
    use my_os::memory;
    use my_os::allocator;
    use x86_64::VirtAddr;
    use my_os::memory::BootInfoFrameAllocator;

    my_os::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_alloc = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_alloc)
        .expect("Couldn't init kernel heap");

    test_main();

    my_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}


use alloc::boxed::Box;
#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(42);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 42);
}

use alloc::vec::Vec;
#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n-1)*n/2);
}

use my_os::allocator::HEAP_SIZE;
#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
