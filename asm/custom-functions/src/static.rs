mod system;
use crate::system::heap::Heap;
use core::{ffi::c_char, option::Option};
use alloc::boxed::Box;

#[no_mangle]
#[link_section = "data"]
pub static mut UPDATE_HOOK: Option<fn() -> u32> = Option::None;
#[no_mangle]
#[link_section = "data"]
pub static mut REL_LOADED: bool = false;

#[no_mangle]
pub extern "C" fn custom_main_additions() -> u32 {
    unsafe {
        if !REL_LOADED {
            REL_LOADED = load_rel("custom\0".as_ptr() as *const c_char);
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
    fn DynamicModuleControl__do_load(_: *mut DynamicModuleControl) -> bool;
    fn DynamicModuleControl__do_link(_: *mut DynamicModuleControl) -> bool;
}

#[no_mangle]
pub fn load_rel(path: *const c_char) -> bool {
    let rel = Box::new(DynamicModuleControl {
        dat: [0; 0x48usize],
    });
    unsafe {
        let rel_ptr = Box::into_raw(rel) as *mut DynamicModuleControl;
        __ct__DynamicModuleControl(rel_ptr, path, core::ptr::null_mut());
        DynamicModuleControl__do_load(rel_ptr) && DynamicModuleControl__do_link(rel_ptr)
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}