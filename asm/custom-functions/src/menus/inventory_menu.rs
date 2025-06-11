use crate::{
    game::file_manager, game::flag_managers::{ItemflagManager, SceneflagManager, StoryflagManager}, game::player, system::button::*,
    utils::menu::SimpleMenu,
};

use alloc::vec::Vec;
use alloc::vec;

pub struct ItemEntry {
    names:  Vec<&'static str>,
    flags: Vec<u16>,
    description: &'static str,
}

impl ItemEntry {
    fn number_owned(&self) -> usize {
        for i in (0..self.flags.len()).rev() {
            if ItemflagManager::check(self.flags[i]) {
                return i + 1;
            }
        }

        0
    }

    fn set_number_owned(&mut self, owned: usize) {
        for i in 0..self.flags.len() {
            if i < owned {
                ItemflagManager::set_to_value(self.flags[i], 1);
            } else {
                ItemflagManager::set_to_value(self.flags[i], 0);
            }
        }
        ItemflagManager::do_commit();
    }
}

#[derive(PartialEq, Eq)]
enum MenuState {
    Off,
    Main,
}

pub struct InventoryMenu {
    state:  MenuState,
    cursor: u32,
    items: Vec<ItemEntry>
}

#[no_mangle]
#[link_section = "data"]
static mut INVENTORY_MENU: InventoryMenu = InventoryMenu {
    state:  MenuState::Off,
    cursor: 0,
    items: Vec::new(),
};

impl super::Menu for InventoryMenu {
    fn enable() {
        unsafe { INVENTORY_MENU.state = MenuState::Main };
    }

    fn disable() {
        unsafe { INVENTORY_MENU.state = MenuState::Off };
    }

    fn input() {
        let inventory_menu: &mut InventoryMenu = unsafe { &mut INVENTORY_MENU };

        match inventory_menu.state {
            MenuState::Off => {},
            MenuState::Main => {
                let item = &mut inventory_menu.items[inventory_menu.cursor as usize];
                if is_pressed(B) {
                    InventoryMenu::disable();
                } else if is_pressed(DPAD_LEFT) {
                    let mut new_count = item.number_owned();
                    if new_count == 0 {
                        new_count = item.flags.len();
                    } else {
                        new_count -= 1;
                    }
                    item.set_number_owned(new_count);
                } else if is_pressed(DPAD_RIGHT) {
                    let mut new_count = item.number_owned();
                    if new_count == item.flags.len() {
                        new_count = 0;
                    } else {
                        new_count += 1;
                    }
                    item.set_number_owned(new_count);
                }
            },
        }
    }

    fn display() {
        let inventory_menu: &mut InventoryMenu = unsafe { &mut INVENTORY_MENU };

        let menu = crate::reset_menu();
        menu.set_heading("Change Item Flags with Left/Right");
        for item_entry in &inventory_menu.items {
            let count = item_entry.number_owned();
            menu.add_entry_fmt(format_args!(
                "{}",
                item_entry.names[count],
            ), item_entry.description);
        }
        menu.set_cursor(inventory_menu.cursor);
        menu.draw();
        inventory_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        unsafe { INVENTORY_MENU.state != MenuState::Off }
    }
}

pub fn initialize_item_list() {
    let inventory_menu: &mut InventoryMenu = unsafe { &mut INVENTORY_MENU };
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Sword", "Practice Sword", "Goddess Sword", "Goddess Longsword", "Goddess White Sword", "Master Sword", "True Master Sword"],
            flags: vec![10, 11, 12, 9, 13, 14],
            description: "What type of sword you have",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Mitts", "Digging Mitts", "Mogma Mitts"],
            flags: vec![56, 99],
            description: "What type of mitts you have",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Harp", "Goddess's Harp"],
            flags: vec![16],
            description: "Whether or not you have the Goddess's Harp",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Scale", "Water Dragon's Scale"],
            flags: vec![68],
            description: "Whether or not you have the Water Dragon's Scale",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Earrings", "Fireshield Earrings"],
            flags: vec![138],
            description: "Whether or not you have the Fireshield Earrings",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Slingshot", "Slingshot", "Scattershot"],
            flags: vec![52, 105],
            description: "What type of slingshot you have",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Beetle", "Beetle", "Hook Beetle", "Quick Beetle", "Tough Beetle"],
            flags: vec![53, 75, 76, 77],
            description: "What type of beetle you have",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Bomb Bag", "Bomb Bag"],
            flags: vec![92],
            description: "Whether or not you have the Bomb Bag",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Gust Bellows", "Gust Bellows"],
            flags: vec![49],
            description: "Whether or not you have the Gust Bellows",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Whip", "Whip"],
            flags: vec![137],
            description: "Whether or not you have the Whip",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Clawshots", "Clawshots"],
            flags: vec![20],
            description: "Whether or not you have the Clawshots",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Bow", "Bow", "Iron Bow", "Sacred Bow"],
            flags: vec![19, 90, 91],
            description: "What type of bow you have",
        },
    );
    inventory_menu.items.push(
        ItemEntry {
            names: vec!["No Bug Net", "Bug Net", "Big Bug Net"],
            flags: vec![71, 140],
            description: "What type of bug net you have",
        },
    );
}
