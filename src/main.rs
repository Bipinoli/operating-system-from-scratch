#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}


static HELLO: &[u8] = b"Hello, this is Bipin";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // character
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color
        }
    }

    loop {}
}
