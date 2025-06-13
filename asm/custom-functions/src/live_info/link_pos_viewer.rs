use crate::game::player;
use core::fmt::Write;

use crate::utils::console::Console;

pub fn display_pos() {
    if let Some(player) = player::as_mut() {
        let (x, y, z) = (player.pos.x, player.pos.y, player.pos.z);
        let angle = player.angle.y;
        let mut console = Console::with_pos_and_size(0f32, 120f32, 120f32, 85f32);
        console.set_bg_color(0x0000007F);
        console.set_font_color(0xFFFFFFFF);
        console.set_font_size(0.25f32);
        console.set_dynamic_size(true);
        let _ = console.write_fmt(format_args!("pos:\nx:{x:>9.2}\ny:{y:>9.2}\nz:{z:>9.2}\n"));
        let _ = console.write_fmt(format_args!("angle: {angle}"));
        console.draw(true);
    }
}
