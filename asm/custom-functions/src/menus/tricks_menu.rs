use crate::{
    game::file_manager, game::flag_managers::StoryflagManager, game::player, game::reloader, system::button::*,
    utils::menu::SimpleMenu, utils::console::Console, menus::main_menu,
};

use core::fmt::Write;
use core::option::*;

pub struct Trick {
    name:   &'static str,
    description: &'static str,
    associated_enum: ActiveTrick,
    on_select: Option<fn()>,
}

const TRICKS: [Trick; 2] = [
    Trick {
        name:   "Wing Ceremony Cutscene Skip",
        description: "Reload WCCS Save Prompt with D-Pad Left. (Kills Link for faster reloads).",
        associated_enum: ActiveTrick::WCCS,
        on_select: Some(reload_wccs_prompt),
    },
    Trick {
        name:   "Guay Deathwarp",
        description: "Reload the guay deathwarp after Sky RBW with D-Pad Left.",
        associated_enum: ActiveTrick::Guay,
        on_select: Some(reload_guay),
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
        menu.set_heading("Activate a trick to practice it (see description).");
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

fn eval_wccs(buffer: u8) {
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
    let buf = unsafe {WCCS_INPUT_BUFFER};
    // kinda hacky but prevents eye-blinding reloads from the display
    if count >= THREE_FRAMES_LATE && count & 0x80000000 == 0 {
        eval_wccs(buf);
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
    unsafe {
        file_manager::get_current_file()
            .as_mut()
            .unwrap()
            .current_health = 0;
    }
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
    unsafe {
        file_manager::get_current_file()
            .as_mut()
            .unwrap()
            .current_health = 24;
    }
}

pub fn update_tricks() {
    let tricks_menu: &mut TricksMenu = unsafe { &mut TRICKS_MENU };

    match tricks_menu.active_trick {
        ActiveTrick::None => {},
        ActiveTrick::WCCS => {
            check_wccs();
            if is_pressed(DPAD_LEFT) {
                reload_wccs_prompt();
            }
        },
        ActiveTrick::Guay => {
            if is_pressed(DPAD_LEFT) {
                reload_guay();
            }
        }
    }
}
