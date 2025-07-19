use crate::{
    game::file_manager, game::flag_managers::ItemflagManager, game::player, system::button::*,
    utils::menu::SimpleMenu,
};

pub struct Cheat {
    name:   &'static str,
    description: &'static str,
    active: bool,
}

extern "C" {
    fn set_instant_text(active: bool);
}

#[no_mangle]
#[link_section = "data"]
pub static mut CHEATS: [Cheat; 8] = [
    Cheat {
        name:   "Infinite Health",
        description: "Constantly refills health to 20 hearts (unless you're already dead).",
        active: false,
    },
    Cheat {
        name:   "Infinite Stamina",
        description: "Constantly refills stamina and underwater air to full.",
        active: false,
    },
    Cheat {
        name:   "Infinite Slingshot Seeds",
        description: "Constantly refills Slingshot Seeds to full.",
        active: false,
    },
    Cheat {
        name:   "Infinite Bombs",
        description: "Constantly refills Bombs to full.",
        active: false,
    },
    Cheat {
        name:   "Infinite Arrows",
        description: "Constantly refills Arrows to full.",
        active: false,
    },
    Cheat {
        name:   "Infinite Rupees",
        description: "Constantly refills Rupees to 9900.",
        active: false,
    },
    Cheat {
        name:   "Moon Jump while holding D-Pad Right",
        description: "Applies an upward velocity of 56 units to Link while holding D-Pad Right.",
        active: false,
    },
    //Cheat {
    //    name:   "Super Speed",
    //    active: false,
    //},
    Cheat {
        name:   "Instant Text",
        description: "Instantly fills in all text in text boxes.",
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

        let menu = crate::reset_menu();
        menu.set_heading("Cheats");
        for cheat in unsafe { &CHEATS } {
            menu.add_entry_fmt(format_args!(
                "{} [{}]",
                cheat.name,
                if cheat.active { "x" } else { "" }
            ), cheat.description)
        }
        menu.set_cursor(cheats_menu.cursor);
        menu.draw();
        cheats_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        unsafe { CHEAT_MENU.state != MenuState::Off }
    }
}

pub fn update_cheats() {
    unsafe {
        if CHEATS[0].active {
            // Don't overwrite 0 health (so the Kill Link action still works)
            if file_manager::get_current_health() != 0 {
                file_manager::set_current_health(80);
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
        if CHEATS[6].active && ButtonBuffer::check_combo_down_up(DPAD_RIGHT, C) {
            if let Some(player) = player::as_mut() {
                player.velocity.y = 56f32; // Minimum amount for consistent liftoff on the ground
            }
        }
        set_instant_text(CHEATS[7].active);
    }
}
