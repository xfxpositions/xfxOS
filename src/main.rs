#![no_std]
#![no_main]
mod vga_writer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
static TEXT:&[u8;12] = b"Hello world!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_writer::print_test();
    loop {}
}