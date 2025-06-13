mod game;
mod live_info;
mod menus;
mod system;
mod utils;

use alloc::boxed::Box;
use crate::utils::menu::SimpleMenu;
use core::option::Option;

#[no_mangle]
#[link_section = "data"]
static mut SHARED_MENU: Option<SimpleMenu> = Option::None;

pub fn reset_menu() -> &'static mut SimpleMenu {
    unsafe {
        SHARED_MENU = Some(SimpleMenu::new());
        SHARED_MENU.as_mut().unwrap_unchecked()
    }
}

// Update menus each frame
#[no_mangle]
pub fn dyn_hook() -> u32 {
    menus::update();
    if menus::is_active() {
        return 0;
    }
    live_info::display();
    menus::do_global_updates();

    return 1;
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
        menus::initialize();
        crate::system::printf("Successfully loaded this rel file!\n\0".as_ptr() as *const i8);
        set_hook(dyn_hook);
    }
}

#[no_mangle]
pub extern "C" fn _epilog() {
    unsafe {
        clear_hook();
    }
}

#[no_mangle]
pub extern "C" fn _unresolved() {}
