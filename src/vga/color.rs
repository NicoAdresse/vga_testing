// src/vga/color.rs

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Color {
    Black       = 0x0,
    Blue        = 0x1,
    Green       = 0x2,
    Cyan        = 0x3,
    Red         = 0x4,
    Magenta     = 0x5,
    Brown       = 0x6,
    Silver      = 0x7,
    Charcoal    = 0x8,
    LightBlue   = 0x9,
    Lime        = 0xA,
    Aqua        = 0xB,
    LightRed    = 0xC,
    Fuchsia     = 0xD,
    Yellow      = 0xE,
    White       = 0xF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}