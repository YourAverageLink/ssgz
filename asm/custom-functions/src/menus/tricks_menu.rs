use crate::{
    game::file_manager, game::flag_managers::{StoryflagManager, SceneflagManager, ItemflagManager, DungeonflagManager}, game::player, game::reloader, system::button::*,
    utils::menu::SimpleMenu, utils::console::Console, menus::main_menu, game::events::ActorEventFlowMgr
};

use core::fmt::Write;
use core::option::*;

pub struct Trick {
    name:   &'static str,
    description: &'static str,
    associated_enum: ActiveTrick,
    on_select: Option<fn()>,
}

const TRICKS: [Trick; 5] = [
    Trick {
        name:   "Wing Ceremony Cutscene Skip",
        description: "Practice WCCS Save Prompt sidehop (Kills Link for faster reloads).",
        associated_enum: ActiveTrick::WCCS,
        on_select: Some(reload_wccs_prompt),
    },
    Trick {
        name:   "Guay Deathwarp",
        description: "Practice the guay deathwarp after Sky RBW.",
        associated_enum: ActiveTrick::Guay,
        on_select: Some(reload_guay),
    },
    Trick {
        name:   "Keese Yeet",
        description: "Practice in Earth Temple positioned for Keese Yeet.",
        associated_enum: ActiveTrick::KeeseYeet,
        on_select: Some(reload_keese_yeet),
    },
    Trick {
        name:   "Extending Blow",
        description: "Practice the Extending Blow in Deep Woods.",
        associated_enum: ActiveTrick::EB,
        on_select: Some(reload_eb),
    },
    Trick {
        name:   "Scaldera",
        description: "Practice fighting Scaldera in Earth Temple.",
        associated_enum: ActiveTrick::Scaldera,
        on_select: Some(reload_scaldera),
    },
];

#[derive(PartialEq, Eq)]
enum MenuState {
    Off,
    Main,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum ActiveTrick {
    None,
    WCCS,
    Guay,
    KeeseYeet,
    EB,
    Scaldera,
}

pub struct TricksMenu {
    state:  MenuState,
    cursor: u32,
    active_trick: ActiveTrick,
}

#[no_mangle]
#[link_section = "data"]
static mut TRICKS_MENU: TricksMenu = TricksMenu {
    state:  MenuState::Off,
    cursor: 0,
    active_trick: ActiveTrick::None,
};

impl super::Menu for TricksMenu {
    fn enable() {
        unsafe { TRICKS_MENU.state = MenuState::Main };
    }

    fn disable() {
        unsafe { TRICKS_MENU.state = MenuState::Off };
    }

    fn input() {
        let tricks_menu: &mut TricksMenu = unsafe { &mut TRICKS_MENU };

        match tricks_menu.state {
            MenuState::Off => {},
            MenuState::Main => {
                if is_pressed(B) {
                    TricksMenu::disable();
                } else if is_pressed(A) {
                    let trick = &TRICKS[tricks_menu.cursor as usize];
                    if tricks_menu.active_trick == trick.associated_enum {
                        tricks_menu.active_trick = ActiveTrick::None;
                    } else {
                        tricks_menu.active_trick = trick.associated_enum;
                        match trick.on_select {
                            None => {},
                            Some(f) => {
                                (f)();
                                TricksMenu::disable();
                                main_menu::MainMenu::disable();
                            }
                        }
                    }
                }
            },
        }
    }

    fn display() {
        let tricks_menu: &mut TricksMenu = unsafe { &mut TRICKS_MENU };

        let menu = crate::reset_menu();
        menu.set_heading("Practice a trick (reloads on success or pressing D-Pad Left).");
        for trick in &TRICKS {
            menu.add_entry_fmt(format_args!(
                "{} [{}]",
                trick.name,
                if trick.associated_enum == tricks_menu.active_trick { "x" } else { "" }
            ), trick.description);
        }
        menu.set_cursor(tricks_menu.cursor);
        menu.draw();
        tricks_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        unsafe { TRICKS_MENU.state != MenuState::Off }
    }
}

extern "C" {
    static mut FRAME_COUNT: u32;
    // static mut SCALDERA_CONTEXT_MAYBE: u16;
}

// The buffer will stop accepting A presses on the frame that is 3 frames too late
#[link_section = "data"]
pub static mut WCCS_INPUT_BUFFER: u8 = 0;

// Frames "-2" and "-1" are the good frames, but there is a 3 frame input delay
// So frame 5 is actually 3 frames late, and frames 1 and 2 are the good ones
const THREE_FRAMES_LATE: u32 = 5;

pub fn update_buffer() {
    // The buffer's bits store whether or not A was pressed in the last 8 frames
    unsafe {
        WCCS_INPUT_BUFFER <<= 1;
        if is_pressed(A) {
            WCCS_INPUT_BUFFER += 1;
        }
    }
}

fn eval_wccs() {
    let buffer = unsafe {WCCS_INPUT_BUFFER};
    let mut console = Console::with_pos_and_size(0f32, 378f32, 120f32, 60f32);
    console.set_bg_color(0x0000007F);
    console.set_font_size(0.5f32);
    console.set_dynamic_size(true);
    // We're checking inputs 3 frames after the window closed
    // TODO - console color doesn't seem to work
    if buffer & 0x10 != 0 {
        // 4 frames ago
        console.set_font_color(0x00FF00FF);
        let _ = console.write_fmt(format_args!("got it (first frame)"));
    }
    else if buffer & 0x08 != 0 {
        // 3 frames ago
        console.set_font_color(0x00FF00FF);
        let _ =console.write_fmt(format_args!("got it (second frame)"));
    }
    else if buffer & 0x20 != 0 {
        // 5 frames ago
        console.set_font_color(0xFFFF00FF);
        let _ =console.write_fmt(format_args!("1 frame early"));
    }
    else if buffer & 0x04 != 0 {
        // 2 frames ago
        console.set_font_color(0xFFFF00FF);
        let _ = console.write_fmt(format_args!("1 frame late"));
    }
    else if buffer & 0x40 != 0 {
        // 6 frames ago
        console.set_font_color(0xFFC000FF);
        let _ = console.write_fmt(format_args!("2 frames early"));
    }
    else if buffer & 0x02 != 0 {
        // 1 frame ago
        console.set_font_color(0xFFC000FF);
        let _ = console.write_fmt(format_args!("2 frames late"));
    }
    else if buffer & 0x80 != 0 {
        // 7 frames ago
        console.set_font_color(0xFF4000FF);
        let _ = console.write_fmt(format_args!("3 frames early"));
    }
    else if buffer & 0x01 != 0 {
        // this frame
        console.set_font_color(0xFF4000FF);
        let _ = console.write_fmt(format_args!("3 frames late"));
    } else {
        console.set_font_color(0xFF0000FF);
        let _ = console.write_fmt(format_args!("more than 3 frames off"));
    }
    let _ = console.write_fmt(format_args!("\nTry again by pressing D-Pad Left."));
    console.draw(false);
}

fn check_wccs() {
    let count = unsafe {FRAME_COUNT};
    if count < THREE_FRAMES_LATE {
        update_buffer();
    }
    // kinda hacky but prevents eye-blinding reloads from the display
    if count >= THREE_FRAMES_LATE && count & 0x80000000 == 0 {
        eval_wccs();
        // Kill Link for faster reloads
        file_manager::set_current_health(0);
    }
}

fn reload_wccs_prompt() {
    // kinda hacky but prevents eye-blinding reloads from the display
    unsafe { FRAME_COUNT = 0x80000000; }
    reloader::set_save_prompt_flag();
    reloader::trigger_entrance(
        b"F000\0".as_ptr(),
        0,
        3, // Layer 3
        0,
        2,
        2,
        1,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(8);
}

fn reload_guay() {
    // Flag 24 is having seen the Fi text near Faron Pillar, must be unset
    StoryflagManager::set_to_value(24, 0);
    StoryflagManager::do_commit();
    reloader::trigger_entrance(
        b"F020\0".as_ptr(),
        0,
        2, // Layer 2
        20, // Entrance 20
        2,
        2,
        1,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(24);
}

fn reload_keese_yeet() {
    SceneflagManager::unset_global(14, 29); // ET keese yeet rope cut
    SceneflagManager::unset_global(14, 24); // ET drawbridge down
    set_sword_to_goddess();
    let current_file = file_manager::get_file_A();
    // Positioned for Keese Yeet
    current_file.pos_t1.x = 512.0;
    current_file.pos_t1.y = 0.0;
    current_file.pos_t1.z = 6600.0;
    current_file.angle_t1 = 0;
    reloader::trigger_entrance(
        b"D200\0".as_ptr(),
        1,
        0,
        2, // Entrance 2 (for no entrance animation)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reloader_type(1);
    reloader::set_reload_trigger(5);
}

fn reload_scaldera() {
    // DungeonflagManager::set_to_value(3, 0); // Unset boss beaten dungeonflag
    SceneflagManager::set_global(14, 47); // Boulder rolling cutscene
    SceneflagManager::set_global(14, 37); // Fi Text in Room
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    // StoryflagManager::set_to_value(7, 0); // Unset ET Beaten
    // StoryflagManager::set_to_value(189, 0); // Unset flag after Scaldera CS
    StoryflagManager::set_to_value(686, 0); // Unset something???
    // StoryflagManager::set_to_value(1102, 0); // Unset something???
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(92, 1); // Give Bomb Bag
    ItemflagManager::increase_counter(2, 10); // Refill Bombs
    set_sword_to_goddess();
    let current_file = file_manager::get_file_A();
    // Positioned for Scaldera
    current_file.pos_t1.x = 407.0;
    current_file.pos_t1.y = 7440.0;
    current_file.pos_t1.z = -21166.0;
    current_file.angle_t1 = 16384;
    // current_file.dungeon_flags[5][2] &= 0xFFFF - 1024;
    reloader::trigger_entrance(
        b"B200\0".as_ptr(),
        10, // Room 10 (actual boss area)
        2, // Layer 2
        1, // Entrance 1 (for no entrance animation)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reloader_type(1);
    reloader::set_reload_trigger(5);
}

fn set_sword_to_goddess() {
    ItemflagManager::set_to_value(11, 1); // Give Goddess Sword
    // Remove higher-level swords
    ItemflagManager::set_to_value(12, 0);
    ItemflagManager::set_to_value(9, 0);
    ItemflagManager::set_to_value(13, 0);
    ItemflagManager::set_to_value(14, 0);
    ItemflagManager::do_commit();
}

fn reload_eb() {
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(52, 1); // Give Slingshot
    ItemflagManager::increase_counter(4, 20); // Refill Seeds
    set_sword_to_goddess();
    let current_file = file_manager::get_file_A();
    // Positioned for EB
    current_file.pos_t1.x = -450.0;
    current_file.pos_t1.y = 2405.0;
    current_file.pos_t1.z = 15000.0;
    current_file.angle_t1 = 32000;
    reloader::trigger_entrance(
        b"F101\0".as_ptr(),
        0,
        1,
        2, // Entrance 2 (for no entrance animation)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reloader_type(1);
    reloader::set_reload_trigger(5);
}

pub fn update_tricks() {
    let tricks_menu: &mut TricksMenu = unsafe { &mut TRICKS_MENU };

    match tricks_menu.active_trick {
        ActiveTrick::None => {},
        ActiveTrick::WCCS => {
            check_wccs();
            if is_pressed(DPAD_LEFT) {
                reload_wccs_prompt();
            } else if let Some(link) = player::as_mut() {
                if link.pos.z < 5205f32 {
                    // Link dies and falls over after successful WCCS, reload
                    link.pos.z = 5300f32;
                    reload_wccs_prompt();
                }
            }
        },
        ActiveTrick::Guay => {
            let health = file_manager::get_current_health();
            // Auto-reload on successful deathwarp
            if is_pressed(DPAD_LEFT) || health == 0 {
                reload_guay();
            }
        },
        ActiveTrick::KeeseYeet => {
            // Auto-reload on successful Keese Yeet
            if is_pressed(DPAD_LEFT) || SceneflagManager::check_global(14, 29) {
                reload_keese_yeet();
            } else if let Some(link) = player::as_mut() {
                if link.pos.x >= 4999f32 && link.pos.z <= 3451f32 && link.angle.y == -16384 {
                    // Position failed to load somehow, so reload again
                    link.pos.x = 4900f32;
                    reload_keese_yeet();
                }
            }
        },
        ActiveTrick::EB => {
            if is_pressed(DPAD_LEFT) {
                reload_eb();
            } else if let Some(link) = player::as_mut() {
                if link.pos.z < 2500f32 {
                    // Successfully got EB
                    link.pos.z = 4000f32;
                    reload_eb();
                }
            }
        },
        ActiveTrick::Scaldera => {
            if is_pressed(DPAD_LEFT) {
                reload_scaldera();
            }
            
            if let Some(link) = player::as_mut() {
                // No idea why, but setting these zoneflags allows skipping Ghirahim's text
                SceneflagManager::set_zone_flag(10, 193, true);
                SceneflagManager::set_zone_flag(10, 194, true);
                SceneflagManager::set_zone_flag(10, 195, true);
                // HACKY
                if link.pos.x > 0f32 && link.pos.y > 7400f32 && link.pos.z < -20000f32 && !StoryflagManager::check(686) {
                    ActorEventFlowMgr::trigger_entry_point(301, 9); // Scaldera trigger
                    eval_wccs();
                }
            }
            // DungeonflagManager::set_to_value(3, 0); // Unset boss beaten dungeonflag
        }
    }
}
