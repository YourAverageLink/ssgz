use core::fmt::Write;

use crate::game::flag_managers::SceneflagManager;

use crate::utils::console::Console;

pub fn disp_scene_flags() {
    let mut console = Console::with_pos(0f32, 0f32);
    console.set_bg_color(0x000000CF);
    console.set_font_color(0xFFFFFFFF);
    console.set_font_size(0.3f32);
    let flags = unsafe { &*SceneflagManager::get_scene_flags() };
    let _ = console.write_str("Scene:");
    for flag_bytes in flags {
        let val_hi = (*flag_bytes >> 8) as u8;
        let val_lo = (*flag_bytes >> 0) as u8;
        let _ = console.write_fmt(format_args!(" {val_hi:02X} {val_lo:02X}"));
    }
    let flags = unsafe { &*SceneflagManager::get_temp_flags() };
    let _ = console.write_str("\n Temp:");
    for flag_bytes in flags {
        let val_hi = (*flag_bytes >> 8) as u8;
        let val_lo = (*flag_bytes >> 0) as u8;
        let _ = console.write_fmt(format_args!(" {val_hi:02X} {val_lo:02X}"));
    }
    // let flags = unsafe { &*SceneflagManager::get_zone_flags() };
    // let _ = console.write_str("\nZone Flags:");
    // for flag_bytes in flags {
    //     let val_hi = (*flag_bytes >> 8) as u8;
    //     let val_lo = (*flag_bytes >> 0) as u8;
    //     let _ = console.write_fmt(format_args!(" {val_hi:02X} {val_lo:02X}"));
    // }
    console.draw(true);
}
