use crate::game::{file_manager, flag_managers, item, player, reloader};
use crate::system::button::*;
use crate::system::math::Vec3s;
use crate::utils::menu::SimpleMenu;
use crate::menus::Menu;

use super::main_menu;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActionMenuState {
    Off,
    Main,
    Item,
    SceneFlag,
}

const BYTESTRS: [&'static str; 16usize] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];
const BITSTRS: [&'static str; 8usize] = ["01", "02", "04", "08", "10", "20", "40", "80"];

struct SceneflagCursor {
    menu_cursor: u32,
    byte_cursor: u8,
    bit_cursor:  u8,
}

fn calc_sceneflag_num(cursor: &SceneflagCursor) -> u16 {
    let byte_offset = if cursor.byte_cursor & 1 == 1 {
        cursor.byte_cursor * 8 - 8
    } else {
        cursor.byte_cursor * 8 + 8
    };

    (byte_offset + cursor.bit_cursor) as u16
}

pub struct ActionMenu {
    state:       ActionMenuState,
    cursor:      u32,
    item_cursor: u16,
    flag_cursor: SceneflagCursor,
}

#[no_mangle]
#[link_section = "data"]
static mut ACTION_MENU: ActionMenu = ActionMenu {
    state:       ActionMenuState::Off,
    cursor:      0,
    item_cursor: 0,
    flag_cursor: SceneflagCursor {
        menu_cursor: 0,
        byte_cursor: 0,
        bit_cursor:  0,
    },
};

struct SavedInfo {
    stage:      [u8; 32],
    room:       u8,
    layer:      u8,
    entrance:   u8,
    night:      u8,
    trial:      u8,
    saved_data: bool,
}

#[no_mangle]
#[link_section = "data"]
static mut SAVE_INFO: SavedInfo = SavedInfo {
    stage:      [0; 32],
    room:       0,
    layer:      0,
    entrance:   0,
    night:      0,
    trial:      0,
    saved_data: false,
};

fn save_file() {
    // Implementaion of the old Practice codes by shoutplenty
    let current_file = file_manager::get_file_A();

    if let Some(link) = player::as_mut() {
        current_file.pos_t1 = link.pos;
        current_file.angle_t1 = link.angle.y;
    }

    file_manager::save_A_to_selected();

    // Save Link position + angle to FA and then -> FS
    let spawn_master = reloader::get_spawn_master();
    let save_info = unsafe { &mut SAVE_INFO };
    save_info.saved_data = true;
    save_info.stage = spawn_master.name;
    save_info.room = spawn_master.room;
    save_info.layer = spawn_master.layer;
    save_info.entrance = spawn_master.entrance;
    save_info.night = spawn_master.night;
    save_info.trial = spawn_master.trial;
}

fn load_file(direct: bool) {
    // Implementaion of the old Practice codes by shoutplenty
    file_manager::load_selected_to_A();
    flag_managers::copy_all_managers_from_save();

    let spawn_master = reloader::get_spawn_master();
    let save_info = unsafe { &mut SAVE_INFO };

    spawn_master.name = save_info.stage;
    spawn_master.room = save_info.room;
    spawn_master.layer = save_info.layer;
    spawn_master.entrance = save_info.entrance;
    spawn_master.night = save_info.night;
    spawn_master.trial = save_info.trial;

    if direct {
        reloader::set_reloader_type(1);
    }
    reloader::set_reload_trigger(5);
}

fn load_position() {
    if let Some(link) = player::as_mut() {
        let current_file = file_manager::get_file_A();
        let mut angle = link.angle;
        angle.y = current_file.angle_t1;
        player::force_set_link_pos_rot(&current_file.pos_t1, &angle);
    }
}

pub fn action_save_file() {
    let action_menu = unsafe { &mut ACTION_MENU };
    save_file();
    action_menu.state = ActionMenuState::Off;
    main_menu::MainMenu::disable();
}

pub fn action_load_file() {
    let action_menu = unsafe { &mut ACTION_MENU };
    if unsafe { SAVE_INFO.saved_data } {
        load_file(false);
        action_menu.state = ActionMenuState::Off;
        main_menu::MainMenu::disable();
    }
}

pub fn action_load_file_direct() {
    let action_menu = unsafe { &mut ACTION_MENU };
    if unsafe { SAVE_INFO.saved_data } {
        load_file(true);
        action_menu.state = ActionMenuState::Off;
        main_menu::MainMenu::disable();
    }
}

pub fn action_load_position() {
    let action_menu = unsafe { &mut ACTION_MENU };
    if unsafe { SAVE_INFO.saved_data } {
        load_position();
        action_menu.state = ActionMenuState::Off;
        main_menu::MainMenu::disable();
    }
}

pub fn enter_bit() {
    reloader::set_reloader_type(3);
    reloader::soft_reset();
}

fn give_item() {}

impl super::Menu for ActionMenu {
    fn enable() {
        let action_menu = unsafe { &mut ACTION_MENU };
        action_menu.state = ActionMenuState::Main;
    }

    fn disable() {
        let action_menu = unsafe { &mut ACTION_MENU };
        action_menu.state = ActionMenuState::Off;
    }
    fn input() {
        let action_menu = unsafe { &mut ACTION_MENU };

        const SAVE_FILE: u32 = 0;
        const LOAD_FILE: u32 = 1;
        const LOAD_FILE_DIRECT: u32 = 2;
        const LOAD_POS: u32 = 3;
        const KILL_LINK: u32 = 4;
        const SCENE_FLAG: u32 = 5;
        const ENTER_BIT: u32 = 6;

        #[cfg(feature = "debug_dyn")]
        const GIVE_ITEM: u32 = 7;
        #[cfg(feature = "debug_dyn")]
        const DEBUG_SAVE: u32 = 8;

        let b_pressed = is_pressed(B);
        let a_pressed = is_pressed(A);
        let right_pressed = is_pressed(DPAD_RIGHT) || should_scroll(DPAD_RIGHT);
        let left_pressed = is_pressed(DPAD_LEFT) || should_scroll(DPAD_LEFT);

        match action_menu.state {
            ActionMenuState::Off => {},
            ActionMenuState::Main => {
                if b_pressed {
                    action_menu.state = ActionMenuState::Off;
                } else if a_pressed {
                    match action_menu.cursor {
                        SAVE_FILE => {
                            action_save_file();
                        },
                        LOAD_FILE => {
                            action_load_file();
                        },
                        LOAD_FILE_DIRECT => {
                            action_load_file_direct();
                        },
                        LOAD_POS => {
                            action_load_position();
                        },
                        KILL_LINK => {
                            file_manager::set_current_health(0);
                            action_menu.state = ActionMenuState::Off;
                            main_menu::MainMenu::disable();
                        },
                        SCENE_FLAG => {
                            action_menu.state = ActionMenuState::SceneFlag;
                        },
                        ENTER_BIT => {
                            enter_bit();
                            action_menu.state = ActionMenuState::Off;
                            main_menu::MainMenu::disable();
                        }
                        #[cfg(feature = "debug_dyn")]
                        GIVE_ITEM => {
                            action_menu.state = ActionMenuState::Item;
                        },
                        #[cfg(feature = "debug_dyn")]
                        DEBUG_SAVE => {
                            file_manager::trigger_save();
                            action_menu.state = ActionMenuState::Off;
                            main_menu::MainMenu::disable();
                        }
                        _ => {},
                    }
                }
            },
            ActionMenuState::Item => {
                if b_pressed {
                    action_menu.state = ActionMenuState::Main;
                } else if a_pressed {
                    item::give_item(action_menu.item_cursor, u32::MAX, 1);
                    action_menu.state = ActionMenuState::Off;
                    main_menu::MainMenu::disable();
                } else if right_pressed {
                    action_menu.item_cursor = if action_menu.item_cursor == 0x1FE {
                        0
                    } else {
                        action_menu.item_cursor + 1
                    };
                } else if left_pressed {
                    action_menu.item_cursor = if action_menu.item_cursor == 0 {
                        0x1FE
                    } else {
                        action_menu.item_cursor - 1
                    };
                }
            },
            ActionMenuState::SceneFlag => {
                if b_pressed {
                    action_menu.state = ActionMenuState::Main;
                } else if a_pressed {
                    flag_managers::SceneflagManager::set_local(calc_sceneflag_num(
                        &action_menu.flag_cursor,
                    ));
                    action_menu.state = ActionMenuState::Off;
                    main_menu::MainMenu::disable();
                } else if right_pressed {
                    match action_menu.flag_cursor.menu_cursor {
                        0 => {
                            action_menu.flag_cursor.byte_cursor =
                                if action_menu.flag_cursor.byte_cursor == 15 {
                                    0
                                } else {
                                    action_menu.flag_cursor.byte_cursor + 1
                                };
                        },
                        _ => {
                            action_menu.flag_cursor.bit_cursor =
                                if action_menu.flag_cursor.bit_cursor == 7 {
                                    0
                                } else {
                                    action_menu.flag_cursor.bit_cursor + 1
                                };
                        },
                    }
                } else if left_pressed {
                    match action_menu.flag_cursor.menu_cursor {
                        0 => {
                            action_menu.flag_cursor.byte_cursor =
                                if action_menu.flag_cursor.byte_cursor == 0 {
                                    15
                                } else {
                                    action_menu.flag_cursor.byte_cursor - 1
                                };
                        },
                        _ => {
                            action_menu.flag_cursor.bit_cursor =
                                if action_menu.flag_cursor.bit_cursor == 0 {
                                    7
                                } else {
                                    action_menu.flag_cursor.bit_cursor - 1
                                };
                        },
                    }
                }
            },
        }
    }
    fn display() {
        let action_menu = unsafe { &mut ACTION_MENU };

        match action_menu.state {
            ActionMenuState::Off => {},
            ActionMenuState::Main => {
                let menu = crate::reset_menu();
                let can_load = unsafe { SAVE_INFO.saved_data };
                menu.set_heading("Action Menu");
                menu.add_entry("Save File", "Save Link's current map, position, and status.");
                if can_load {
                    menu.add_entry("Load File", "Load saved file at saved entrance.");
                    menu.add_entry("Direct Load File", "Load saved file at saved position.");
                    menu.add_entry("Load Position", "Set Link's position & rotation to that of the saved file.");
                } else {
                    menu.add_entry("Load File", "You must save a file in this menu first to use this.");
                    menu.add_entry("Direct Load File", "You must save a file in this menu first to use this.");
                    menu.add_entry("Load Position", "You must save a file in this menu first to use this.");
                }
                menu.add_entry("Kill Link", "Kills Link (even with Infinite Health enabled).");
                menu.add_entry("RBM Scene Flag", "RBMs and commits a chosen scene flag in this area.");
                menu.add_entry("Enter BiT", "Enter into Back in Time on Skyloft.");

                #[cfg(feature = "debug_dyn")]
                menu.add_entry("Debug: Give Item", "Trigger an item get for an item id (risky, may cause crashes).");
                #[cfg(feature = "debug_dyn")]
                menu.add_entry("Debug: Create Save", "Initiates a save as though you saved at a statue.");

                menu.set_cursor(action_menu.cursor);
                menu.draw();
                action_menu.cursor = menu.move_cursor();
            },
            ActionMenuState::Item => {
                let menu = crate::reset_menu();
                menu.set_heading("Give Item");
                menu.add_entry_fmt(format_args!("Id: {}", action_menu.item_cursor), "Give this item.");
                menu.draw();
            },
            ActionMenuState::SceneFlag => {
                let menu = crate::reset_menu();
                let flag_cursor = &mut action_menu.flag_cursor;
                let byte_str = BYTESTRS[flag_cursor.byte_cursor as usize];
                let bit_str = BITSTRS[flag_cursor.bit_cursor as usize];
                menu.set_heading_fmt(format_args!("RBM Scene Flag ({}x{})", byte_str, bit_str,));
                menu.add_entry_fmt(format_args!("Byte: {}", byte_str), "Which byte (0x0 through 0xF) in the flag sheet.");
                menu.add_entry_fmt(format_args!("Bit: {}", bit_str), "Which bit (0x01 through 0x80) in the flag sheet.");
                menu.set_cursor(flag_cursor.menu_cursor);
                menu.draw();
                flag_cursor.menu_cursor = menu.move_cursor();
            },
        }
    }

    fn is_active() -> bool {
        let action_menu = unsafe { &mut ACTION_MENU };
        action_menu.state != ActionMenuState::Off
    }
}
