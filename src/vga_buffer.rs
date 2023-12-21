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


#[repr(transparent)]
struct VGA_Buffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_COLS]; VGA_BUFFER_ROWS]
}


pub struct Writer {
    current_col: usize,
    current_color_code: ColorCode,
    buffer: &'static mut VGA_Buffer,
}


impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.current_col >= VGA_BUFFER_COLS {
                    self.new_line();
                }

                let row = VGA_BUFFER_ROWS - 1;
                let col = self.current_col;

                let color_code = self.current_color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii: byte,
                    color_code,
                });
                self.current_col += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_BUFFER_ROWS {
            for col in 0..VGA_BUFFER_COLS {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(VGA_BUFFER_ROWS - 1);
        self.current_col = 0;
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

pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        current_col: 0,
        current_color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VGA_Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        current_col: 0,
        current_color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VGA_Buffer) },
    });
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