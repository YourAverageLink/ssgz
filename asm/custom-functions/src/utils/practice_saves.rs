use crate::game::file_manager::{get_saved_save_files, get_skip_dat, initialize_write_save};
use crate::game::save_file::{SavedSaveFiles, SkipDatArr};
use crate::system::printf;
use core::ffi::{c_char, c_void};
use core::mem::size_of;
use alloc::vec;
// #[repr(C)]
// pub struct DVD_Command {
// pub vtable:           u32,
// pub next:             *mut c_void, // actually dvd_command*
// pub status:           u8,
// pub mount_direction:  u8,
// pub compression_type: u8,
// }
//
// #[repr(C)]
// pub struct DVD_ToMainRam_Normal {
// pub base_command:       DVD_Command,
// to main ram base members
// pub data_ptr:           *mut c_void,
// pub amount_read:        i32,
// pub file_size:          u32,
// pub heap:               *mut Heap,
// own members
// pub compression_type_2: u8,
// pub entry_num:          i32,
// }
//
// #[repr(C)]
// pub struct Loader {
// pub vtable:  u32,
// pub size:    u32,
// pub command: *mut DVD_ToMainRam_Normal,
// pub heap:    *mut Heap,
// pub buffer:  *mut c_void,
// }
//
// impl DVD_ToMainRam_Normal {
// fn get_base_ptr(&mut self) -> *mut DVD_Command {
// (&mut self.base_command) as *mut DVD_Command
// }
// }

extern "C" {
    static reload_color_fader: *mut ReloadColorFader;
    // fn requestFileLoadFromDiskOrDie(
    // path: *const c_char,
    // mount_direction: i32,
    // heap: *mut Heap,
    // ) -> *mut DVD_ToMainRam_Normal;
    // fn requestFileLoadFromDisk(
    // path: *const c_char,
    // mount_direction: i32,
    // heap: *mut Heap,
    // ) -> *mut DVD_ToMainRam_Normal;
    // fn Loader__request(
    // loader: *mut Loader,
    // path: *const c_char,
    // mount_direction: i32,
    // heap: *mut Heap,
    // ) -> *mut c_void;
    // fn __ct__Loader(loader: *mut Loader);
    // fn DVD_Command__waitUntilDone(_: *mut DVD_Command);
    // fn loadToMainRAM(
    // path: *const c_char,
    // dest: *mut u8,
    // heap: *mut Heap,
    // alloc_dir: EAllocDirection,
    // offset: u32,
    // p_read: *mut u32,
    // p_file_size: *mut u32,
    // );
    // fn IsExistPath(path: *const c_char) -> bool;
    fn DVDOpen(path: *const c_char, info: *mut c_void) -> bool;
    fn DVDClose(info: *mut c_void);
    fn DVDReadPrio(info: *mut c_void, dest: *mut c_void, size: i32, offset: i32, prio: i32) -> i32;
    // fn allocOnCurrentHeap(count: usize) -> *mut c_void;
    fn doSoftResetMaybe(fader: *mut ReloadColorFader);
}

#[repr(C)]
pub struct ReloadColorFader {
    pub _0:             [u8; 0x14],
    pub current_state:  u32,
    pub unk:            u32,
    pub previous_state: u32,
    pub _1:             [u8; 0x65],
    pub other_state:    u8,
}

fn soft_reset() {
    unsafe {
        (*reload_color_fader).other_state = 1;
        (*reload_color_fader).previous_state = (*reload_color_fader).current_state;
        (*reload_color_fader).current_state = 1;

        doSoftResetMaybe(reload_color_fader);
    }
}

#[no_mangle]
pub fn load_practice_save(dir: &str) {
    unsafe {
        let mut dvd_info = vec![0u8; 60usize];
        let info_ptr = dvd_info.as_mut_ptr() as *mut c_void;
        let sav_path = format!("{dir}/wiiking2.sav\0");
        let skip_path = format!("{dir}/skip.dat\0");
        if DVDOpen(sav_path.as_ptr() as *const c_char, info_ptr) {
            let save_size = size_of::<SavedSaveFiles>();
            let save_buf = get_saved_save_files();
            DVDReadPrio(info_ptr, save_buf as *mut c_void, save_size as i32, 0, 2);
            DVDClose(info_ptr);
            initialize_write_save();
            printf("Successfully loaded wii2king.sav!\n\0".as_ptr() as *const i8);
        } else {
            return;
        }
        if DVDOpen(skip_path.as_ptr() as *const c_char, info_ptr) {
            let skip_size = size_of::<SkipDatArr>();
            let skip_buf = get_skip_dat();
            DVDReadPrio(info_ptr, skip_buf as *mut c_void, skip_size as i32, 0, 2);
            DVDClose(info_ptr);
            printf("Successfully loaded skip.dat!\n\0".as_ptr() as *const i8);
        } else {
            return;
        }

        initialize_write_save();
        soft_reset();
    }
}
