mod game;
mod live_info;
mod menus;
mod system;
mod utils;

use alloc::boxed::Box;
use crate::system::button;
use crate::system::button::Buttons;
use crate::utils::menu::SimpleMenu;
use crate::menus::Menu;
use crate::menus::main_menu;
use crate::game::reloader::in_reset;
use crate::system::button::ButtonBuffer;
use core::option::Option;

#[no_mangle]
#[link_section = "data"]
static mut SHARED_MENU: Option<SimpleMenu> = Option::None;

// Wait 6 seconds after loading the rel to enable menus,
// to allow time for the main heap to be initialized
#[no_mangle]
#[link_section = "data"]
pub static mut COUNTDOWN_TIMER: u8 = 180;

#[no_mangle]
#[link_section = "data"]
pub static mut INITIALIZED: bool = false;

pub fn reset_menu() -> &'static mut SimpleMenu {
    unsafe {
        SHARED_MENU = Some(SimpleMenu::new());
        SHARED_MENU.as_mut().unwrap_unchecked()
    }
}

// Update menus each frame
#[no_mangle]
pub fn dyn_hook() -> u32 {
    let frame_advance = main_menu::MainMenu::frame_advance_enabled();
    let continue_game = main_menu::MainMenu::should_advance_frame();
    
    if unsafe {INITIALIZED} {
        if frame_advance {
            if continue_game {
                // we should set all buttons currently held down that weren't held last active frame as pressed
                let last_active_down = ButtonBuffer::buttons_down() | button::ONE | button::DPAD_RIGHT;
                let actual_down = button::buttons_down();
                button::add_buttons_pressed(actual_down & !last_active_down);
                ButtonBuffer::update();
            }
        } else {
            ButtonBuffer::update();
        }
        // The game would softlock if the menu were still open during a soft reset
        if in_reset() {
            main_menu::MainMenu::disable();
            menus::force_disable_runtime_tools();
        }
        menus::update();
        if menus::is_active() {
            return 0;
        }
        live_info::display();
        menus::do_global_updates();
    } else {
        unsafe {
            COUNTDOWN_TIMER -= 1;
            if COUNTDOWN_TIMER == 0 {
                menus::initialize();
                crate::system::printf("Menus are ready!\n\0".as_ptr() as *const i8);
                INITIALIZED = true;
            }
        }
    }

    if frame_advance {
        // Disable frame advance-related buttons in frame advance mode
        button::set_buttons_not_down(button::ONE | button::DPAD_RIGHT);
    }

    continue_game as u32
}

extern "C" { 
    #[allow(improper_ctypes)]
    fn set_hook(func: fn() -> u32);
    fn clear_hook();
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _prolog() {
    unsafe {
        crate::system::printf("Loaded customNP.rel, waiting 6 seconds to initialize...\n\0".as_ptr() as *const i8);
        set_hook(dyn_hook);
    }
}

#[no_mangle]
pub extern "C" fn _epilog() {
    menus::force_disable_runtime_tools();
    unsafe {
        clear_hook();
    }
}

#[no_mangle]
pub extern "C" fn _unresolved() {}
