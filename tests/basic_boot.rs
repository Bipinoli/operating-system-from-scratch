#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(operating_system_from_scratch::test_runner)]

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    operating_system_from_scratch::test_panic_handler(info)
}

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


use operating_system_from_scratch::println;

#[test_case]
fn test_println() {
    println!("test_println output");
}