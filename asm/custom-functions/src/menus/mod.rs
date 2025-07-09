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
mod tricks_menu;

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

pub fn do_global_updates() {
    cheats_menu::update_cheats();
    tricks_menu::update_tricks();
    if main_menu::check_extra_hotkey_pressed(crate::system::button::DPAD_RIGHT) {
        action_menu::action_save_file();
        // Show guardian potion running out to indicate save
        let current_file = crate::game::file_manager::get_file_A();
        current_file.guardianPotionTimer = 1;
    }
    else if main_menu::check_extra_hotkey_pressed(crate::system::button::DPAD_LEFT) {
        action_menu::action_load_position();
    }
    else if main_menu::check_extra_hotkey_pressed(crate::system::button::DPAD_UP) {
        action_menu::action_load_file_direct();
    }
    else if main_menu::check_extra_hotkey_pressed(crate::system::button::DPAD_DOWN) {
        action_menu::action_load_file();
    }
}

pub fn initialize() {
    practice_saves_menu::initialize_practice_saves();
    inventory_menu::initialize_item_list();
}