#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vgu_buffer = 0xb8000 as *mut u8;
    
}
