use core::ptr::Unique;
use core::fmt::Write;
use spin::Mutex;

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::vga_buffer::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
                                 

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}

#[repr(u8)]
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 3,
    Cyan        = 4,
    Red         = 5,
    Magenta     = 6,
    Brown       = 7,
    LightGray   = 8,
    DarkGrag    = 9,
    LightBlue   = 10,
    LightGreen  = 11,
    LightCyan   = 12,
    LightRed    = 13,
    LightMagenta = 14,
    Yellow      = 15,
    White       = 16,
}

#[derive(Copy, Clone)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    fn new(foreground: Color, background: Color) -> Writer {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(foreground, background),
            buffer: unsafe { Unique::new(0xb8000 as *mut _) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            },
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                self.buffer().chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: ColorCode::new(Color::LightGreen, Color::Black),
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT-1) {
            let buffer = self.buffer();
            buffer.chars[row] = buffer.chars[row + 1]
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        self.buffer().chars[row] = [blank; BUFFER_WIDTH];
    }
}

impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});
