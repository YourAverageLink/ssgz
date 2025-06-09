mod game;
mod live_info;
mod menus;
mod system;
mod utils;

// A Common Place where Custom code can be injected to run once per frame
// Returns whether or not to stop (1 == continue)
#[no_mangle]
pub fn dyn_hook() -> u32 {
    menus::update();
    if menus::is_active() {
        return 0;
    }
    live_info::display();
    menus::do_cheats();

    return 1;
}

extern "C" {
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
