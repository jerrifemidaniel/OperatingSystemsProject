mod constants;
use core::{
    fmt::{self, Write},
    ptr,
};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;


const COLOR_BLUE: [u8; 3] = [255, 0, 0];          // BGR for blue
const COLOR_WHITE: [u8; 3] = [255, 255, 255];     // BGR for white (default color)

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    current_color: [u8; 3],
}

impl FrameBufferWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
            current_color: COLOR_WHITE,
        };
        logger.clear();
        logger
    }

    pub fn print_at(&mut self, x:usize, y:usize, text:&str) {
        let original_x = self.x_pos;
        let original_y = self.y_pos;

        self.x_pos = x;
        self.y_pos = y;

        let _ = write!(self, "{}", text);

        self.x_pos = original_x;
        self.y_pos = original_y;
    }

    fn newline(&mut self) {
        self.y_pos += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
        if self.y_pos >= self.height() {
            self.scroll();
        }
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    
    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            '\t' => {
                for _ in 0..4 { self.write_char(' '); }
            }
            '\\' => {
                
            }
            c => {
                if self.x_pos + font_constants::CHAR_RASTER_WIDTH >= self.width() {
                    self.newline();
                }
                if self.y_pos + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING >= self.height() {
                    self.scroll();
                }
                self.write_rendered_char(get_char_raster(c));
                self.x_pos += font_constants::CHAR_RASTER_WIDTH;
            }
        }
    }

    fn scroll(&mut self) {
        let char_height = CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        let stride_bytes = self.info.stride * self.info.bytes_per_pixel;
        let scroll_bytes = char_height * stride_bytes;
        let height = self.height();

        
        self.framebuffer.copy_within(scroll_bytes.., 0);

    
        let clear_start = (height - char_height) * stride_bytes;
        self.framebuffer[clear_start..].fill(0);

        
        self.y_pos -= char_height;
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        let pixel_offset = y * self.info.stride + x;
        let color = [
            (self.current_color[0] as u16 * intensity as u16 / 255) as u8,
            (self.current_color[1] as u16 * intensity as u16 / 255) as u8,
            (self.current_color[2] as u16 * intensity as u16 / 255) as u8,
        ];
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)].copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }

    #[allow(dead_code)]
    pub fn draw_cursor(&mut self) {
        let cursor_char = '|'; 
        let cursor_raster = get_char_raster(cursor_char);
        self.write_rendered_char(cursor_raster);
    }

    #[allow(dead_code)]
    pub fn clear_cursor(&mut self) {
        let cursor_char = ' '; 
        let cursor_raster = get_char_raster(cursor_char);
        self.write_rendered_char(cursor_raster);
 
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    if let Some(next) = chars.next() {
                        match next {
                            'c' => self.current_color = COLOR_BLUE,  
                            'r' => self.current_color = COLOR_WHITE, 
                            'T' => self.x_pos = self.width() - BORDER_PADDING - font_constants::CHAR_RASTER_WIDTH*26,
                            _ => self.write_char(c),            
                        }
                    }
                }
                _ => self.write_char(c),
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($writer:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($writer, $($arg)*);
    }};
}
