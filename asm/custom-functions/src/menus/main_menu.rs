use super::action_menu::ActionMenu;
use super::cheats_menu::CheatsMenu;
use super::display_menu::DisplayMenu;
use super::practice_saves_menu::PracticeSavesMenu;
use super::flag_menu::FlagMenu;
use super::warp_menu::WarpMenu;
use super::inventory_menu::InventoryMenu;
use super::tricks_menu::TricksMenu;
use core::fmt::Write;
use crate::system::button::*;
use crate::utils::char_writer::{TextWriterBase, CharWriter};
use crate::utils::graphics::draw_rect;
use crate::utils::menu::SimpleMenu;

use wchar::wchz;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MenuState {
    Off,
    MenuSelect,
    DisplayMenu,
    WarpMenu,
    // HeapMenu,
    ActionMenu,
    CheatsMenu,
    PracticeSavesMenu,
    FlagMenu,
    InventoryMenu,
    TricksMenu,
}

const NUM_MENUS: usize = 6;

impl MenuState {
    fn from_u32(num: u32) -> MenuState {
        match num {
            0 => MenuState::DisplayMenu,
            1 => MenuState::WarpMenu,
            // 2 => MenuState::HeapMenu,
            2 => MenuState::ActionMenu,
            3 => MenuState::CheatsMenu,
            4 => MenuState::PracticeSavesMenu,
            5 => MenuState::FlagMenu,
            6 => MenuState::InventoryMenu,
            7 => MenuState::TricksMenu,
            _ => MenuState::MenuSelect,
        }
    }
}

pub struct MainMenu {
    state:       MenuState,
    cursor:      u32,
    force_close: bool,
    description: &'static str,
}

#[link_section = "data"]
#[no_mangle]
pub static mut MAIN_MENU: MainMenu = MainMenu {
    state:       MenuState::Off,
    cursor:      0,
    force_close: false,
    description: "",
};

impl super::Menu for MainMenu {
    fn enable() {
        if MainMenu::is_active() {
            return;
        };

        if is_down(Buttons::Z | Buttons::C) {
            unsafe { MAIN_MENU.state = MenuState::MenuSelect };
        }
    }
    fn disable() {
        unsafe { MAIN_MENU.force_close = true };
        // Removes possible interaction with the game
        set_buttons_not_pressed(Buttons::B | Buttons::A);
    }
    fn input() {
        let main_menu = unsafe { &mut MAIN_MENU };
        match main_menu.state {
            // MenuState::Off => {},
            MenuState::MenuSelect => {
                if is_pressed(B) {
                    main_menu.state = MenuState::Off;
                    set_buttons_not_pressed(B);
                } else if is_pressed(A) {
                    main_menu.state = MenuState::from_u32(main_menu.cursor);
                    match main_menu.state {
                        MenuState::DisplayMenu => DisplayMenu::enable(),
                        MenuState::WarpMenu => WarpMenu::enable(),
                        // MenuState::HeapMenu => HeapMenu::enable(),
                        MenuState::ActionMenu => ActionMenu::enable(),
                        MenuState::CheatsMenu => CheatsMenu::enable(),
                        MenuState::PracticeSavesMenu => PracticeSavesMenu::enable(),
                        MenuState::FlagMenu => FlagMenu::enable(),
                        MenuState::InventoryMenu => InventoryMenu::enable(),
                        MenuState::TricksMenu => TricksMenu::enable(),
                        _ => {},
                    };
                }
            },
            MenuState::DisplayMenu => DisplayMenu::input(),
            MenuState::WarpMenu => WarpMenu::input(),
            // MenuState::HeapMenu => HeapMenu::input(),
            MenuState::ActionMenu => ActionMenu::input(),
            MenuState::CheatsMenu => CheatsMenu::input(),
            MenuState::PracticeSavesMenu => PracticeSavesMenu::input(),
            MenuState::FlagMenu => FlagMenu::input(),
            MenuState::InventoryMenu => InventoryMenu::input(),
            MenuState::TricksMenu => TricksMenu::input(),
            _ => {},
        }
    }
    fn display() {
        let main_menu = unsafe { &mut MAIN_MENU };

        let mut writer = TextWriterBase::new();

        // Draw the input Guide
        if MainMenu::is_active() {
            draw_rect(0f32, 0f32, 640f32, 480f32, 0.0f32, 0x000000C0);
            writer.set_font_color(0xFFFFFFFF, 0xFFFFFFFF);
            writer.set_position(10f32, 420f32);
            writer.print_symbol(wchz!(u16, "\x20"));
            writer.print(wchz!(u16, "Select\t"));
            writer.print_symbol(wchz!(u16, "\x21"));
            writer.print(wchz!(u16, "Back\t"));
            writer.print_symbol(wchz!(u16, "\x2F\x30"));
            writer.print(wchz!(u16, "Up/Down\t"));
            writer.print_symbol(wchz!(u16, "\x31\x32"));
            writer.print(wchz!(u16, "Change Value"));
        }

        match main_menu.state {
            MenuState::Off => {
                write_description("");
            },
            MenuState::MenuSelect => {
                let menu = crate::reset_menu();
                // TODO - automate versioning here
                menu.set_heading("SSGZ 0.1.3 - Select a Menu");
                menu.add_entry("Display Menu", "Passively display info on-screen.");
                menu.add_entry("Warp Menu", "Warp to a given stage, room, layer, and entrance.");
                // menu.add_entry("Heap Menu");
                menu.add_entry("Action Menu", "Perform miscellaneous actions.");
                menu.add_entry("Cheats Menu", "Activate useful cheats.");
                menu.add_entry("Practice Saves Menu", "Load speedrun practice saves.");
                menu.add_entry("Flag Menu", "Set/unset relevant progress flags.");
                menu.add_entry("Inventory Menu", "Set/unset notable items in inventory.");
                menu.add_entry("Tricks Menu", "Practice certain specific tricks.");
                menu.set_cursor(main_menu.cursor);
                menu.draw();

                main_menu.cursor = menu.move_cursor();
            },
            MenuState::DisplayMenu => {
                DisplayMenu::display();
                if !DisplayMenu::is_active() {
                    main_menu.state = MenuState::MenuSelect;
                }
            },
            MenuState::WarpMenu => {
                WarpMenu::display();
                if !WarpMenu::is_active() {
                    main_menu.state = MenuState::MenuSelect;
                }
            },
            // MenuState::HeapMenu => {
            // HeapMenu::display();
            // if !HeapMenu::is_active() {
            // main_menu.state = MenuState::MenuSelect;
            // }
            // },
            MenuState::ActionMenu => {
                ActionMenu::display();
                if !ActionMenu::is_active() {
                    main_menu.state = MenuState::MenuSelect;
                }
            },
            MenuState::CheatsMenu => {
                CheatsMenu::display();
                if !CheatsMenu::is_active() {
                    main_menu.state = MenuState::MenuSelect;
                }
            },
            MenuState::PracticeSavesMenu => {
                PracticeSavesMenu::display();
                if !PracticeSavesMenu::is_active() {
                    main_menu.state = MenuState::MenuSelect;
                }
            },
            MenuState::FlagMenu => {
                FlagMenu::display();
                if !FlagMenu::is_active() {
                    main_menu.state = MenuState::MenuSelect;
                }
            },
            MenuState::InventoryMenu => {
                InventoryMenu::display();
                if !InventoryMenu::is_active() {
                    main_menu.state = MenuState::MenuSelect;
                }
            }
            MenuState::TricksMenu => {
                TricksMenu::display();
                if !TricksMenu::is_active() {
                    main_menu.state = MenuState::MenuSelect;
                }
            }
        }

        if MainMenu::is_active() && !main_menu.description.is_empty() {
            writer.set_position(10f32, 400f32);
            let mut line = CharWriter::new();
            let _ = line.write_str(main_menu.description);
            line.draw(&mut writer);
        }

        if main_menu.force_close {
            main_menu.force_close = false;
            main_menu.state = MenuState::Off;
        }
    }
    fn is_active() -> bool {
        unsafe { MAIN_MENU.state != MenuState::Off }
    }
}

pub fn write_description(text: &'static str) {
    unsafe { MAIN_MENU.description = text; }
}