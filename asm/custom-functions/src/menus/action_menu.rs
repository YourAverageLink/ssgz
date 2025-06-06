use crate::game::{file_manager, flag_managers, item, player, reloader};
use crate::system::button::*;
use crate::utils::menu::SimpleMenu;
use crate::utils::practice_saves::load_practice_save;

use super::main_menu;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActionMenuState {
    Off,
    Main,
    Item,
    PracticeSave,
    Category,
}

pub struct ActionMenu {
    state:           ActionMenuState,
    cursor:          u32,
    item_cursor:     u16,
    category_cursor: u32,
    cat_save_cursor: u8,
}

#[no_mangle]
#[link_section = "data"]
static mut ACTION_MENU: ActionMenu = ActionMenu {
    state:           ActionMenuState::Off,
    cursor:          0,
    item_cursor:     0,
    category_cursor: 0,
    cat_save_cursor: 0,
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

struct SpeedrunCategory {
    name:      &'static str,
    base_path: &'static str,
    saves:     [&'static str; 25usize],
    num_saves: u8,
}

const ANY: SpeedrunCategory = SpeedrunCategory {
    name:      "Any%",
    base_path: "/saves/Any",
    saves:     [
        "Start",
        "First BiT",
        "Copy After Cave",
        "Sky RBW",
        "Skyview RBW",
        "F3 in Skyview",
        "Ghirahim 1",
        "Goddess Statue RBW",
        "Eldin RBW",
        "Eldin OoB",
        "ET Door RBM",
        "ET Bridge RBM",
        "F1 Keese Yeet F2 Scaldera",
        "Lanayru Pillar RBM",
        "Lanayru Mine BiTWarp",
        "Rock RBM",
        "Machi RBM",
        "Gorge BiTWarp",
        "2x20 Crystal RBM",
        "3 in 1 - G3 Escape, Statue, Demise",
        "",
        "",
        "",
        "",
        "",
    ],
    num_saves: 20,
};

// TODO - AD Saves for NTSC
const AD: SpeedrunCategory = SpeedrunCategory {
    name:      "All Dungeons",
    base_path: "/saves/All Dungeons",
    saves:     [
        "Start",
        "After Waterfall Cave",
        "Sealed Grounds",
        "Faron Woods Entry",
        "Skyview",
        "After Skyview",
        "Volcano Ascent",
        "Earth Temple",
        "After ET",
        "AC CSWW",
        "Ancient Cistern",
        "After Cistern",
        "Stone Cache",
        "Raise LMF",
        "Sand Sea Skip",
        "Sandship",
        "Lanayru Mining Facility",
        "After LMF",
        "Sky Keep",
        "After Sky Keep",
        "Eldin Trial RBM",
        "After Eldin Trial",
        "Fire Sanctuary",
        "Gate of Time Skip",
        "Horde",
    ],
    num_saves: 25,
};

// const HUNDO: SpeedrunCategory = SpeedrunCategory {
// name:      "100%",
// base_path: "/saves/100",
// saves:     [""],
// num_saves: 1,
// };

const CATEGORIES: [SpeedrunCategory; 2usize] = [ANY, AD]; // HUNDO];

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
        const PRAC_SAVE: u32 = 5;

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
                        PRAC_SAVE => {
                            action_menu.state = ActionMenuState::PracticeSave;
                            // load_practice_save("/saves/All Dungeons/Gate of
                            // Time Skip");
                            // action_menu.state = ActionMenuState::Off;
                            // main_menu::MainMenu::disable();
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
            ActionMenuState::PracticeSave => {
                if is_pressed(B) {
                    action_menu.state = ActionMenuState::Main;
                } else if is_pressed(A) {
                    action_menu.state = ActionMenuState::Category;
                }
            },
            ActionMenuState::Category => {
                let category = &CATEGORIES[action_menu.category_cursor as usize];
                if is_pressed(B) {
                    action_menu.state = ActionMenuState::PracticeSave;
                } else if is_pressed(A) {
                    let save = category.saves[action_menu.cat_save_cursor as usize];
                    load_practice_save(format!("{0}/{save}", category.base_path).as_str());
                    action_menu.state = ActionMenuState::Off;
                    main_menu::MainMenu::disable();
                } else if is_pressed(DPAD_RIGHT) {
                    action_menu.cat_save_cursor =
                        if action_menu.cat_save_cursor == category.num_saves - 1 {
                            0
                        } else {
                            action_menu.cat_save_cursor + 1
                        };
                } else if is_pressed(DPAD_LEFT) {
                    action_menu.cat_save_cursor = if action_menu.cat_save_cursor == 0 {
                        category.num_saves - 1
                    } else {
                        action_menu.cat_save_cursor - 1
                    };
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
                menu.add_entry("Load Practice Save");
                menu.draw();
                action_menu.cursor = menu.move_cursor();
            },
            ActionMenuState::Item => {
                let mut menu: SimpleMenu<1> = SimpleMenu::new();
                menu.set_heading("Give Item");
                menu.add_entry_fmt(format_args!("Id: {}", action_menu.item_cursor));
                menu.draw();
            },
            ActionMenuState::PracticeSave => {
                let mut menu: SimpleMenu<2> = SimpleMenu::new();
                menu.set_heading("Choose a Category");
                menu.set_cursor(action_menu.category_cursor);
                for n in 0..2 {
                    menu.add_entry_fmt(format_args!("{}", CATEGORIES[n].name));
                }
                menu.draw();
                action_menu.category_cursor = menu.move_cursor();
            },
            ActionMenuState::Category => {
                let category = &CATEGORIES[action_menu.category_cursor as usize];
                let mut menu: SimpleMenu<1> = SimpleMenu::new();
                menu.set_heading("Choose a Practice Save");
                menu.add_entry_fmt(format_args!(
                    "{}: {}",
                    action_menu.cat_save_cursor,
                    category.saves[action_menu.cat_save_cursor as usize]
                ));
                menu.draw();
            },
        }
    }

    fn is_active() -> bool {
        let action_menu = unsafe { &mut ACTION_MENU };
        action_menu.state != ActionMenuState::Off
    }
}
