use crate::game::{actor::ActorID::LAST, collision::get_ground_height, enemy::AcEnBase, player};
use core::fmt::Write;

use crate::utils::console::Console;

// haven't yet run into issues if the last targeted enemy dies but worth considering if that's a problem
static mut LAST_TARGETED: Option<&AcEnBase> = None;

pub fn display_actor_info() {
    if let Some(player) = player::as_mut() {
        let mut targeting_something = false;
        if let Some(enemy) = player.get_targeted_actor() {
            unsafe { LAST_TARGETED = Some(enemy); }
            targeting_something = true;
        }
        if let Some(enemy) = unsafe { LAST_TARGETED } {
            let pos = enemy.obj_base.ac_base.position;
            let (x, y, z) = (pos.x, pos.y, pos.z);
            let mut console = Console::with_pos_and_size(0f32, 176f32, 120f32, 85f32);
            console.set_bg_color(0x0000007F);
            console.set_font_color(0xFFC0C0FF);
            console.set_font_size(0.25f32);
            console.set_dynamic_size(true);
            let _ = console.write_fmt(format_args!("{} actor:\nx:{x:>9.2}\ny:{y:>9.2}\nz:{z:>9.2}", if targeting_something { "targeted" } else { "last targeted" }));
            console.draw(true);
        }
    } else {
        unsafe { LAST_TARGETED = None; }
    }
}
