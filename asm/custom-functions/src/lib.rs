#![no_std]
#![feature(allocator_api)]
#![feature(split_array)]
#![feature(const_trait_impl)]
#![allow(dead_code)]
#![feature(slice_ptr_get)]

mod game;
mod live_info;
mod menus;
mod system;
mod utils;

// A Common Place where Custom code can be injected to run once per frame
// Returns whether or not to stop (1 == continue)
#[no_mangle]
fn custom_main_additions() -> u32 {
    menus::update();
    if menus::is_active() {
        return 0;
    }
    live_info::display();
    menus::do_cheats();

    return 1;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
