#![no_std]
#![feature(allocator_api)]
#![feature(split_array)]
#![feature(const_trait_impl)]
#![allow(dead_code)]
#![feature(slice_ptr_get)]

#[no_mangle]
#[link_section = "data"]
pub static mut UPDATE_HOOK: Option<fn() -> u32> = None;

// A Common Place where Custom code can be injected to run once per frame
// Returns whether or not to stop (1 == continue)
#[no_mangle]
pub extern "C" fn custom_main_additions() -> u32 {
    if let Some(hook) = UPDATE_HOOK {
        return hook();
    }

    return 1;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn set_hook(func: fn() -> u32) {
    UPDATE_HOOK = Some(func);
}

#[no_mangle]
pub fn clear_hook() {
    UPDATE_HOOK = None;
}