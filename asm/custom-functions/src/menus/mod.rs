use self::main_menu::MainMenu;

mod action_menu;
mod cheats_menu;
mod display_menu;
// mod heap_menu;
pub mod main_menu;
mod practice_saves_menu;
mod warp_menu;
mod flag_menu;
mod inventory_menu;

pub trait Menu {
    fn enable();
    fn disable();
    fn input();
    fn display();
    fn is_active() -> bool;
}

pub fn update() {
    MainMenu::enable();
    if MainMenu::is_active() {
        MainMenu::display();
        MainMenu::input();
    }
}

pub fn is_active() -> bool {
    MainMenu::is_active()
}

pub fn do_cheats() {
    cheats_menu::update_cheats();
}

pub fn initialize() {
    practice_saves_menu::initialize_practice_saves();
    inventory_menu::initialize_item_list();
}