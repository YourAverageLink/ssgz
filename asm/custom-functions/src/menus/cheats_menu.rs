use crate::{system::button::*, utils::menu::SimpleMenu};

pub struct Cheat {
    name:   &'static str,
    active: bool,
}

#[no_mangle]
#[link_section = "data"]
pub static mut CHEATS: [Cheat; 5] = [
    Cheat {
        name:   "Infinite Health",
        active: false,
    },
    Cheat {
        name:   "Infinite Stamina",
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
                }
            },
        }
    }

    fn display() {
        let cheats_menu: &mut CheatsMenu = unsafe { &mut CHEAT_MENU };

        let mut menu = SimpleMenu::<5>::new();
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
