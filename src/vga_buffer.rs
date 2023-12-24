use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color_code: ColorCode,
}

const VGA_BUFFER_COLS: usize = 80;
const VGA_BUFFER_ROWS: usize = 25;
const H_PADDING: usize = 1;
const V_PADDING: usize = 1;


#[repr(transparent)]
struct VgaBuffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_COLS]; VGA_BUFFER_ROWS]
}


pub struct Writer {
    cur_col: usize,
    cur_row: usize,
    has_overflowen: bool,
    current_color_code: ColorCode,
    buffer: &'static mut VgaBuffer,
}


impl Writer {
    pub fn new() -> Writer {
        Writer {
            cur_col: H_PADDING,
            cur_row: V_PADDING,
            has_overflowen: false,
            current_color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) }
        }
    }

    /// write top to bottom
    /// when the data overflows the buffer
    /// just shift them above to have new emply line below
    /// the characters at the top will be clipped
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.cur_col + H_PADDING >= VGA_BUFFER_COLS {
                    self.new_line();
                }

                self.buffer.chars[self.cur_row][self.cur_col].write(ScreenChar {
                    ascii: byte,
                    color_code: self.current_color_code,
                });

                self.cur_col += 1;
            }
        }
    }


    fn new_line(&mut self) {
        self.cur_row += 1;
        self.cur_col = H_PADDING;

        if self.cur_row + V_PADDING >= VGA_BUFFER_ROWS {
            self.has_overflowen = true;
            self.cur_row = VGA_BUFFER_ROWS - 1 - V_PADDING; // last line
        }

        if self.has_overflowen {
            for row in (V_PADDING+1)..(VGA_BUFFER_ROWS - V_PADDING) {
                for col in 0..VGA_BUFFER_COLS {
                    self.buffer.chars[row-1][col].write(self.buffer.chars[row][col].read());
                }
            }
            self.clear_row(self.cur_row);
        }
    }


    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii: b' ',
            color_code: self.current_color_code,
        };
        for col in 0..VGA_BUFFER_COLS {
            self.buffer.chars[row][col].write(blank);
        }
    }


    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
}






impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}





#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}