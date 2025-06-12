#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my_os::println;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World! This is my {}\n", "operating system");
    
    my_os::init();

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