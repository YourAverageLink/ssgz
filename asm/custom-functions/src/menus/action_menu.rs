use crate::game::{file_manager, flag_managers, item, player, reloader};
use crate::system::button::*;
use crate::utils::menu::SimpleMenu;

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
        const GIVE_ITEM: u32 = 3;
        const KILL_LINK: u32 = 4;
        const SCENE_FLAG: u32 = 5;

        match action_menu.state {
            ActionMenuState::Off => {},
            ActionMenuState::Main => {
                if is_pressed(B) {
                    action_menu.state = ActionMenuState::Off;
                } else if is_pressed(A) {
                    match action_menu.cursor {
                        SAVE_FILE => {
                            save_file();
                            action_menu.state = ActionMenuState::Off;
                            main_menu::MainMenu::disable();
                        },
                        LOAD_FILE => {
                            load_file(false);
                            action_menu.state = ActionMenuState::Off;
                            main_menu::MainMenu::disable();
                        },
                        LOAD_FILE_DIRECT => {
                            load_file(true);
                            action_menu.state = ActionMenuState::Off;
                            main_menu::MainMenu::disable();
                        },
                        GIVE_ITEM => {
                            action_menu.state = ActionMenuState::Item;
                        },
                        KILL_LINK => {
                            unsafe {
                                file_manager::get_current_file()
                                    .as_mut()
                                    .unwrap()
                                    .current_health = 0;
                            }
                            action_menu.state = ActionMenuState::Off;
                            main_menu::MainMenu::disable();
                        },
                        SCENE_FLAG => {
                            action_menu.state = ActionMenuState::SceneFlag;
                        },
                        _ => {},
                    }
                }
            },
            ActionMenuState::Item => {
                if is_pressed(B) {
                    action_menu.state = ActionMenuState::Main;
                } else if is_pressed(A) {
                    item::give_item(action_menu.item_cursor, u32::MAX, 1);
                    action_menu.state = ActionMenuState::Off;
                    main_menu::MainMenu::disable();
                } else if is_pressed(DPAD_RIGHT) {
                    action_menu.item_cursor = if action_menu.item_cursor == 0x1FE {
                        0
                    } else {
                        action_menu.item_cursor + 1
                    };
                } else if is_pressed(DPAD_LEFT) {
                    action_menu.item_cursor = if action_menu.item_cursor == 0 {
                        0x1FE
                    } else {
                        action_menu.item_cursor - 1
                    };
                }
            },
            ActionMenuState::SceneFlag => {
                if is_pressed(B) {
                    action_menu.state = ActionMenuState::Main;
                } else if is_pressed(A) {
                    flag_managers::SceneflagManager::set_local(calc_sceneflag_num(
                        &action_menu.flag_cursor,
                    ));
                    action_menu.state = ActionMenuState::Off;
                    main_menu::MainMenu::disable();
                } else if is_pressed(DPAD_RIGHT) {
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
                } else if is_pressed(DPAD_LEFT) {
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
                let mut menu: SimpleMenu<6> = SimpleMenu::new();
                menu.set_heading("Action Menu");
                menu.set_cursor(action_menu.cursor);
                menu.add_entry("Save File");
                menu.add_entry("Load File");
                menu.add_entry("Direct Load File");
                menu.add_entry("Give Item");
                menu.add_entry("Kill Link");
                menu.add_entry("RBM Scene Flag");
                menu.draw();
                action_menu.cursor = menu.move_cursor();
            },
            ActionMenuState::Item => {
                let mut menu: SimpleMenu<1> = SimpleMenu::new();
                menu.set_heading("Give Item");
                menu.add_entry_fmt(format_args!("Id: {}", action_menu.item_cursor));
                menu.draw();
            },
            ActionMenuState::SceneFlag => {
                let mut menu: SimpleMenu<2> = SimpleMenu::new();
                let flag_cursor = &mut action_menu.flag_cursor;
                let byte_str = BYTESTRS[flag_cursor.byte_cursor as usize];
                let bit_str = BITSTRS[flag_cursor.bit_cursor as usize];
                menu.set_cursor(flag_cursor.menu_cursor);
                menu.set_heading_fmt(format_args!("RBM Scene Flag ({}x{})", byte_str, bit_str,));
                menu.add_entry_fmt(format_args!("Byte: {}", byte_str));
                menu.add_entry_fmt(format_args!("Bit: {}", bit_str));
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
