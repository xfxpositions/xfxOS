#![no_std]
#![no_main]
mod vga_print;

use core::{panic::PanicInfo};

use vga_print::{VGACOLORS, Buffer};


static HELLO: &[u8] = b"Hello World!";
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    let vga_buffer = unsafe{ &mut *(0xb8000 as *mut Buffer) };
    let mut printer = vga_print::Printer { vga_buffer: vga_buffer, row_position: 0,col_position:0,default_bg:VGACOLORS::BLACK,default_fg:VGACOLORS::MAGENTA};
    printer.print_chr('\n' as u8,VGACOLORS::MAGENTA,VGACOLORS::BLACK);
    printer.print_chr('H' as u8, VGACOLORS::MAGENTA, VGACOLORS::BLACK);
    printer.print_chr('e' as u8, VGACOLORS::MAGENTA, VGACOLORS::BLACK);
    printer.print_chr('l' as u8, VGACOLORS::MAGENTA, VGACOLORS::BLACK);
    printer.println("Deneme");
    printer.println("Deneme");
    printer.println("Deneme");
    printer.println("Deneme");
    printer.println_colored("Deneme ",VGACOLORS::CYAN,VGACOLORS::BLACK);
    printer.print("Deneme");
    




    
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    loop {}
}