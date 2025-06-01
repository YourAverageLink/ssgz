use crate::{
    game::file_manager, game::flag_managers::ItemflagManager, game::player, system::button::*,
    utils::menu::SimpleMenu,
};

pub struct Cheat {
    name:   &'static str,
    active: bool,
}

#[no_mangle]
#[link_section = "data"]
pub static mut CHEATS: [Cheat; 6] = [
    Cheat {
        name:   "Infinite Health",
        active: false,
    },
    Cheat {
        name:   "Infinite Stamina",
        active: false,
    },
    Cheat {
        name:   "Infinite Slingshot Seeds",
        active: false,
    },
    Cheat {
        name:   "Infinite Bombs",
        active: false,
    },
    Cheat {
        name:   "Infinite Arrows",
        active: false,
    },
    Cheat {
        name:   "Infinite Rupees",
        active: false,
    },
];

#[derive(PartialEq, Eq)]
enum MenuState {
    Off,
    Main,
}

pub struct CheatsMenu {
    state:  MenuState,
    cursor: u32,
}

#[no_mangle]
#[link_section = "data"]
static mut CHEAT_MENU: CheatsMenu = CheatsMenu {
    state:  MenuState::Off,
    cursor: 0,
};

impl super::Menu for CheatsMenu {
    fn enable() {
        unsafe { CHEAT_MENU.state = MenuState::Main };
    }

    fn disable() {
        unsafe { CHEAT_MENU.state = MenuState::Off };
    }

    fn input() {
        let cheats_menu: &mut CheatsMenu = unsafe { &mut CHEAT_MENU };

        match cheats_menu.state {
            MenuState::Off => {},
            MenuState::Main => {
                if is_pressed(B) {
                    CheatsMenu::disable();
                } else if is_pressed(A) {
                    unsafe {
                        CHEATS[cheats_menu.cursor as usize].active ^= true;
                    }
                }
            },
        }
    }

    fn display() {
        let cheats_menu: &mut CheatsMenu = unsafe { &mut CHEAT_MENU };

        let mut menu = SimpleMenu::<6>::new();
        menu.set_cursor(cheats_menu.cursor);
        menu.set_heading("Cheats");
        for cheat in unsafe { &CHEATS } {
            menu.add_entry_fmt(format_args!(
                "{} [{}]",
                cheat.name,
                if cheat.active { "x" } else { "" }
            ))
        }
        cheats_menu.cursor = menu.move_cursor();
        menu.draw();
    }

    fn is_active() -> bool {
        unsafe { CHEAT_MENU.state != MenuState::Off }
    }
}

pub fn update_cheats() {
    unsafe {
        if CHEATS[0].active {
            // Don't overwrite 0 health (so the Kill Link action still works)
            if file_manager::get_current_file()
                .as_mut()
                .unwrap()
                .current_health
                != 0
            {
                file_manager::get_current_file()
                    .as_mut()
                    .unwrap()
                    .current_health = 80;
            }
        }
        if CHEATS[1].active {
            if let Some(player) = player::as_mut() {
                player.stamina_amount = 1_000_000;
            }
        }
        if CHEATS[2].active {
            if ItemflagManager::get_counter_by_index(4) < 20 {
                ItemflagManager::increase_counter(4, 20);
            }
        }
        if CHEATS[3].active {
            if ItemflagManager::get_counter_by_index(2) < 10 {
                ItemflagManager::increase_counter(2, 10);
            }
        }
        if CHEATS[4].active {
            if ItemflagManager::get_counter_by_index(1) < 20 {
                ItemflagManager::increase_counter(1, 20);
            }
        }
        if CHEATS[5].active {
            if ItemflagManager::get_counter_by_index(0) < 9900 {
                ItemflagManager::increase_counter(0, 9900);
            }
        }
    }
}
