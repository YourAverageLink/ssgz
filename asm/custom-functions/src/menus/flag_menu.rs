use crate::{
    game::file_manager, game::flag_managers::{ItemflagManager, SceneflagManager, StoryflagManager}, game::player, system::button::*,
    utils::menu::SimpleMenu,
};

enum Flag {
    SceneFlag {scene_idx: u16, flag: u16},
    StoryFlag {flag: u16},
}

pub struct FlagEntry {
    name:   &'static str,
    associated_flag: Flag,
}

impl FlagEntry {
    fn is_set(&self) -> bool {
        match self.associated_flag {
            Flag::SceneFlag { scene_idx, flag } => {
                SceneflagManager::check_global(scene_idx, flag)
            }
            Flag::StoryFlag { flag } => {
                StoryflagManager::check(flag)
            }
        }
    }

    fn toggle(&mut self) {
        match self.associated_flag {
            Flag::SceneFlag { scene_idx, flag } => {
                if self.is_set() {
                    SceneflagManager::set_global(scene_idx, flag);
                } else {
                    SceneflagManager::unset_global(scene_idx, flag);
                }
            }
            Flag::StoryFlag { flag } => {
                if self.is_set() {
                    StoryflagManager::set_to_value(flag, 0);
                } else {
                    StoryflagManager::set_to_value(flag, 1);
                }
                StoryflagManager::do_commit();
            }
        }
    }
}

const NUM_FLAGS: usize = 10;

#[no_mangle]
#[link_section = "data"]
pub static mut RELEVANT_FLAGS: [FlagEntry; NUM_FLAGS] = [
    FlagEntry {
        name:   "B-Wheel",
        associated_flag: Flag::StoryFlag {
            flag: 58,
        }
    },
    FlagEntry {
        name:   "Tunic",
        associated_flag: Flag::StoryFlag {
            flag: 36,
        }
    },
    FlagEntry {
        name:   "Loftwing Saved",
        associated_flag: Flag::StoryFlag {
            flag: 27,
        }
    },
    FlagEntry {
        name:   "Can Dive Off Loftwing",
        associated_flag: Flag::StoryFlag {
            flag: 198,
        }
    },
    FlagEntry {
        name:   "Faron Pillar",
        associated_flag: Flag::StoryFlag {
            flag: 46,
        }
    },
    FlagEntry {
        name:   "Eldin Pillar",
        associated_flag: Flag::StoryFlag {
            flag: 47,
        }
    },
    FlagEntry {
        name:   "Lanayru Pillar",
        associated_flag: Flag::StoryFlag {
            flag: 48,
        }
    },
    FlagEntry {
        name:   "Seen SG Intro Cutscene",
        associated_flag: Flag::StoryFlag {
            flag: 137,
        }
    },
    FlagEntry {
        name:   "Raised Gate of Time",
        associated_flag: Flag::StoryFlag {
            flag: 340,
        }
    },
    FlagEntry {
        name:   "Opened Gate of Time",
        associated_flag: Flag::StoryFlag {
            flag: 341,
        }
    },
];

#[derive(PartialEq, Eq)]
enum MenuState {
    Off,
    Main,
}

pub struct FlagMenu {
    state:  MenuState,
    cursor: u32,
}

#[no_mangle]
#[link_section = "data"]
static mut FLAG_MENU: FlagMenu = FlagMenu {
    state:  MenuState::Off,
    cursor: 0,
};

impl super::Menu for FlagMenu {
    fn enable() {
        unsafe { FLAG_MENU.state = MenuState::Main };
    }

    fn disable() {
        unsafe { FLAG_MENU.state = MenuState::Off };
    }

    fn input() {
        let flag_menu: &mut FlagMenu = unsafe { &mut FLAG_MENU };

        match flag_menu.state {
            MenuState::Off => {},
            MenuState::Main => {
                if is_pressed(B) {
                    FlagMenu::disable();
                } else if is_pressed(A) {
                    unsafe {
                        RELEVANT_FLAGS[flag_menu.cursor as usize].toggle();
                    }
                }
            },
        }
    }

    fn display() {
        let flag_menu: &mut FlagMenu = unsafe { &mut FLAG_MENU };

        let mut menu = SimpleMenu::new();
        menu.set_heading("Set/Unset Relevant Flags");
        for flag_entry in unsafe { &RELEVANT_FLAGS } {
            menu.add_entry_fmt(format_args!(
                "{} [{}]",
                flag_entry.name,
                if flag_entry.is_set() { "x" } else { "" }
            ))
        }
        menu.set_cursor(flag_menu.cursor);
        menu.draw();
        flag_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        unsafe { FLAG_MENU.state != MenuState::Off }
    }
}