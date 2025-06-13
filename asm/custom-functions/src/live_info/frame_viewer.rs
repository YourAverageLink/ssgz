use core::fmt::Write;
use crate::utils::console::Console;
use crate::system::button::*;

extern "C" {
    static mut FRAME_COUNT: u32;
    static mut GAME_FRAME: u32;
    static mut EXE_FRAME: u32;
}

pub fn display_frame_count() {
    let count = unsafe {FRAME_COUNT};
    let mut console = Console::with_pos_and_size(0f32, 432f32, 120f32, 60f32);
    console.set_bg_color(0x0000007F);
    console.set_font_color(0xFFFFFFFF);
    console.set_font_size(0.5f32);
    console.set_dynamic_size(true);
    let _ = console.write_fmt(format_args!("Frames since load: {count}"));
    console.draw(false);
}
