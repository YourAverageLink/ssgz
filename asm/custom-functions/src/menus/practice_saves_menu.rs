use crate::system::button::*;
use crate::utils::menu::SimpleMenu;
use crate::utils::practice_saves::load_practice_save;

use super::main_menu;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PracticeSavesMenuState {
    Off,
    Main,
    Category,
}

pub struct PracticeSavesMenu {
    state:       PracticeSavesMenuState,
    cursor:      u32,
    save_cursor: u8,
}

#[no_mangle]
#[link_section = "data"]
static mut PRACTICE_SAVES_MENU: PracticeSavesMenu = PracticeSavesMenu {
    state:       PracticeSavesMenuState::Off,
    cursor:      0,
    save_cursor: 0,
};

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

impl super::Menu for PracticeSavesMenu {
    fn enable() {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };
        ps_menu.state = PracticeSavesMenuState::Main;
    }

    fn disable() {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };
        ps_menu.state = PracticeSavesMenuState::Off;
    }
    fn input() {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };

        const SAVE_FILE: u32 = 0;
        const LOAD_FILE: u32 = 1;
        const LOAD_FILE_DIRECT: u32 = 2;
        const GIVE_ITEM: u32 = 3;
        const KILL_LINK: u32 = 4;
        const PRAC_SAVE: u32 = 5;

        match ps_menu.state {
            PracticeSavesMenuState::Off => {},
            PracticeSavesMenuState::Main => {
                if is_pressed(B) {
                    ps_menu.state = PracticeSavesMenuState::Off;
                } else if is_pressed(A) {
                    ps_menu.state = PracticeSavesMenuState::Category;
                    let category = &CATEGORIES[ps_menu.cursor as usize];
                    if ps_menu.save_cursor >= category.num_saves {
                        ps_menu.save_cursor = 0;
                    }
                }
            },
            PracticeSavesMenuState::Category => {
                let category = &CATEGORIES[ps_menu.cursor as usize];
                if is_pressed(B) {
                    ps_menu.state = PracticeSavesMenuState::Main;
                } else if is_pressed(A) {
                    let save = category.saves[ps_menu.save_cursor as usize];
                    load_practice_save(format!("{0}/{save}", category.base_path).as_str());
                    ps_menu.state = PracticeSavesMenuState::Off;
                    main_menu::MainMenu::disable();
                } else if is_pressed(DPAD_RIGHT) {
                    ps_menu.save_cursor = if ps_menu.save_cursor == category.num_saves - 1 {
                        0
                    } else {
                        ps_menu.save_cursor + 1
                    };
                } else if is_pressed(DPAD_LEFT) {
                    ps_menu.save_cursor = if ps_menu.save_cursor == 0 {
                        category.num_saves - 1
                    } else {
                        ps_menu.save_cursor - 1
                    };
                }
            },
        }
    }
    fn display() {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };

        match ps_menu.state {
            PracticeSavesMenuState::Off => {},
            PracticeSavesMenuState::Main => {
                let menu = crate::reset_menu();
                menu.set_heading("Choose a Category");
                for category in &CATEGORIES {
                    menu.add_entry_fmt(format_args!("{}", category.name));
                }

                menu.set_cursor(ps_menu.cursor);
                menu.draw();
                ps_menu.cursor = menu.move_cursor();
            },
            PracticeSavesMenuState::Category => {
                let category = &CATEGORIES[ps_menu.cursor as usize];
                let menu = crate::reset_menu();
                menu.set_heading("Choose a Practice Save");
                menu.add_entry_fmt(format_args!(
                    "{}: {}",
                    ps_menu.save_cursor, category.saves[ps_menu.save_cursor as usize]
                ));
                menu.draw();
            },
        }
    }

    fn is_active() -> bool {
        let ps_menu = unsafe { &mut PRACTICE_SAVES_MENU };
        ps_menu.state != PracticeSavesMenuState::Off
    }
}
