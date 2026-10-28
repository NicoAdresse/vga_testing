// src/main.rs

#![no_std]
#![no_main]

mod vga;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use vga::color::Color;
    use vga::writer::Vga;

    let mut vga = Vga::new(Color::Aqua, Color::Black);
    vga.write_string("Welcome! Hello, World!\n");
    vga.write_string("Big boom!");

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}