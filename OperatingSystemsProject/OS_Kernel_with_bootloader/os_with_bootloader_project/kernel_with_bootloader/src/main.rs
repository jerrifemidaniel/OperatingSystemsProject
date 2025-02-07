#![no_std]
#![no_main]

mod writer;
use writer::FrameBufferWriter;

use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;
use core::fmt::Write;

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

static mut FRAME_BUFFER_WRITER: Option<FrameBufferWriter> = None;

macro_rules! print {
    ($fmt:expr) => {
        unsafe {
            if let Some(ref mut writer) = FRAME_BUFFER_WRITER {
                write!(writer, "{}", $fmt).unwrap();
            }
        }
    };
}
fn dynamic_print(x:usize, y:usize, text: &str) {
    unsafe {
        if let Some(ref mut writer) = FRAME_BUFFER_WRITER {
            writer.print_at(x, y, text);
        }
    }
}

fn my_entry_point(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = _boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = _boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);


    unsafe {
        FRAME_BUFFER_WRITER = Some(frame_buffer_writer);
    }


    //dynamic_print(990, 20, "Hello World!");
    


    print!("\\cHello,world. ThisisNathan'scode.\\r \nHello world  ");
    loop {
        hlt(); 
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt(); // Halt in case of panic
    }
}