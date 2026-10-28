// src/vga/writer.rs

use crate::vga::color::{ColorCode, Color};
use core::ptr;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Vga {
    column_pos: usize,
    row_pos: usize,
    color_code: ColorCode,
    buffer: *mut Buffer,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl Vga {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self {
            column_pos: 0,
            row_pos: 0,
            color_code: ColorCode::new(fg, bg),
            buffer: 0xB8000 as *mut Buffer,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_pos;
                let col = self.column_pos;
                let color_code = self.color_code.clone();

                unsafe {
                    let char_ptr = &mut (*self.buffer).chars[row][col] as *mut ScreenChar;
                    char_ptr.write_volatile(ScreenChar {
                            ascii_character: byte,
                            color_code,
                        }
                    );
                }

                self.column_pos += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7E | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0x7E),
            }
        }
    }

    fn new_line(&mut self) {
        self.column_pos = 0;
        if self.row_pos < BUFFER_HEIGHT - 1 {
            self.row_pos += 1;
        } else {
            self.scroll_screen();
        }
    }

    fn scroll_screen(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                unsafe {
                    let src_ptr = &(*self.buffer).chars[row][col] as *const ScreenChar;
                    let dest_ptr = &mut (*self.buffer).chars[row - 1][col] as *mut ScreenChar;
                    
                    let character = ptr::read_volatile(src_ptr);
                    ptr::write_volatile(dest_ptr, character);
                }
            }
        }

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code.clone(),
        };
        for col in 0..BUFFER_WIDTH {
            unsafe {
                let dest_ptr = &mut (*self.buffer).chars[BUFFER_HEIGHT - 1][col] as *mut ScreenChar;
                ptr::write_volatile(dest_ptr, blank.clone());
            }
        }
    }
}