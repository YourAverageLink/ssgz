mod system;
use crate::system::heap::{Heap, get_dylink_heap};
use core::{ffi::c_char, option::Option};
use alloc::boxed::Box;

#[no_mangle]
#[link_section = "data"]
pub static mut UPDATE_HOOK: Option<fn() -> u32> = Option::None;



const STARTING_COUNTDOWN: u16 = 180;
#[no_mangle]
#[link_section = "data"]
pub static mut COUNTDOWN_TIMER: u16 = STARTING_COUNTDOWN + 1;


// A Common Place where Custom code can be injected to run once per frame
// Returns whether or not to stop (1 == continue)
#[no_mangle]
pub extern "C" fn custom_main_additions() -> u32 {
    unsafe {
        if COUNTDOWN_TIMER <= STARTING_COUNTDOWN {
            COUNTDOWN_TIMER -= 1;
            if COUNTDOWN_TIMER == 0 {
                load_rel("custom\0".as_ptr() as *const c_char);
                COUNTDOWN_TIMER = STARTING_COUNTDOWN + 1;
            }
        }
        // there's only one hook here, but you could imagine this being a list of
        // hooks if we end up having multiple custom rels
        if let Option::Some(hook) = UPDATE_HOOK {
            return hook();
        }
    }

    return 1;
}

#[no_mangle]
pub fn set_hook(func: fn() -> u32) {
    unsafe {
        UPDATE_HOOK = Option::Some(func);
    }
}

#[no_mangle]
pub fn clear_hook() {
    unsafe {
        UPDATE_HOOK = Option::None;
    }
}

#[repr(C)]
pub struct DynamicModuleControl {
    pub dat: [u8; 0x48usize],
}

extern "C" {
    fn __ct__DynamicModuleControl(
        _: *mut DynamicModuleControl,
        path: *const c_char,
        heap: *mut Heap,
    );
    fn DynamicModuleControlBase__link(_: *mut DynamicModuleControl) -> bool;
}

#[no_mangle]
pub fn load_custom_rel() {
    // Start a 10-second countdown to load in the rel
    unsafe { COUNTDOWN_TIMER = STARTING_COUNTDOWN; }
    // load_rel("custom\0".as_ptr() as *const c_char)
}

#[no_mangle]
pub fn load_rel(path: *const c_char) -> bool {
    unsafe {
        let heap = get_dylink_heap();
        let rel = Box::new_in(DynamicModuleControl {
            dat: [0; 0x48usize],
        }, heap);
        
        let rel_ptr = Box::into_raw(rel) as *mut DynamicModuleControl;
        __ct__DynamicModuleControl(rel_ptr, path, core::ptr::null_mut());
        DynamicModuleControlBase__link(rel_ptr)
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}