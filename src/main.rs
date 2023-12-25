#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(operating_system_from_scratch::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use operating_system_from_scratch::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    operating_system_from_scratch::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    operating_system_from_scratch::test_panic_handler(info)
}