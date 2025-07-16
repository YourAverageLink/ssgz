#![allow(non_snake_case)]
use super::file_manager::{self};
use core::ffi::{c_ushort, c_void};

#[repr(C)]
struct FlagSpace {
    flag_ptr:   *mut u16,
    flag_count: u16,
    pad:        u16,
    call_ptr:   u32,
}

#[repr(C)]
pub struct DungeonflagManager {
    pub should_commit: bool,
    pub flagindex:     c_ushort,
}
#[repr(C)]
pub struct StoryflagManager {
    tobefilled: u32,
}
#[repr(C)]
pub struct SceneflagManager {
    sceneflags:    FlagSpace,
    tempflags:     FlagSpace,
    zoneflags:     FlagSpace,
    flag_helper:   u8,
    field_0x25:    u8,
    scene_idx:     u16,
    should_commit: bool,
    pad:           [u8; 3],
}
#[repr(C)]
pub struct ItemflagManager {
    tobefilled: u32,
}

extern "C" {
    fn FlagManager__setFlagTo1(mgr: *mut c_void, flag: u16);
    fn FlagManager__getFlagOrCounter(mgr: *mut c_void, flag: u16) -> u16;
    fn FlagManager__setFlagOrCounter(mgr: *mut c_void, flag: u16, value: u16);
    fn FlagManger__copyAllFromSave();
    fn setStoryflagToValue(flag: u16, value: u16);
    static STORYFLAG_MANAGER: *mut StoryflagManager;
    static SCENEFLAG_MANAGER: *mut SceneflagManager;
    static ITEMFLAG_MANAGER: *mut ItemflagManager;
    static DUNGEONFLAG_MANAGER: *mut DungeonflagManager;
    static mut STATIC_STORYFLAGS: [c_ushort; 0x80];
    static mut STATIC_ITEMFLAGS: [c_ushort; 0x40];
    static mut STATIC_DUNGEON_FLAGS: [c_ushort; 8usize];
    fn SceneflagManager__setFlagGlobal(mgr: *mut SceneflagManager, scene_index: u16, flag: u16);
    fn SceneflagManager__unsetFlagGlobal(mgr: *mut SceneflagManager, scene_index: u16, flag: u16);
    fn SceneflagManager__checkFlagGlobal(
        mgr: *mut SceneflagManager,
        scene_index: u16,
        flag: u16,
    ) -> bool;
    fn SceneflagManager__setZoneflag(mgr: *mut SceneflagManager, room: u16, flag: u16);
    fn SceneflagManager__unsetZoneflag(mgr: *mut SceneflagManager, room: u16, flag: u16);
    fn SceneflagManager__checkZoneflag(mgr: *mut SceneflagManager, room: u16, flag: u16) -> bool;
    fn DungeonflagManager__setToValue(mgr: *mut DungeonflagManager, flag: u16, value: i32);
    fn StoryflagManager__doCommit(mgr: *mut StoryflagManager);
    fn ItemflagManager__doCommit(mgr: *mut ItemflagManager);
    fn checkStoryflagIsSet(p: *const StoryflagManager, flag: u16) -> bool;
    fn AcItem__checkItemFlag(flag: u16) -> bool;
    fn getCounterByIndex(index: u16) -> u16;
    fn increaseCounter(index: u16, count: u16);
}

impl StoryflagManager {
    pub fn get_static() -> *mut [u16; 0x80] {
        unsafe { &mut STATIC_STORYFLAGS }
    }

    pub fn do_commit() {
        unsafe { StoryflagManager__doCommit(STORYFLAG_MANAGER) };
    }

    pub fn check(flag: u16) -> bool {
        unsafe { checkStoryflagIsSet(core::ptr::null(), flag) }
    }
    pub fn get_value(flag: u16) -> u16 {
        unsafe { FlagManager__getFlagOrCounter(STORYFLAG_MANAGER as _, flag) }
    }
    pub fn set_to_value(flag: u16, value: u16) {
        unsafe { FlagManager__setFlagOrCounter(STORYFLAG_MANAGER as _, flag, value) };
    }
    #[no_mangle]
    pub fn storyflag_set_to_1(flag: u16) {
        unsafe { FlagManager__setFlagTo1(STORYFLAG_MANAGER as _, flag) };
    }
}

impl ItemflagManager {
    pub fn get_static() -> *mut [u16; 0x40] {
        unsafe { &mut STATIC_ITEMFLAGS }
    }

    pub fn do_commit() {
        unsafe { ItemflagManager__doCommit(ITEMFLAG_MANAGER) };
    }

    pub fn check(flag: u16) -> bool {
        unsafe { AcItem__checkItemFlag(flag) }
    }

    pub fn set_to_value(flag: u16, value: u16) {
        unsafe { FlagManager__setFlagOrCounter(ITEMFLAG_MANAGER as _, flag, value) };
    }

    pub fn get_counter_by_index(index: u16) -> u16 {
        unsafe { getCounterByIndex(index) }
    }

    pub fn increase_counter(index: u16, count: u16) {
        unsafe {
            increaseCounter(index, count);
        }
    }
}

impl SceneflagManager {
    pub fn check_global(scn_idx: u16, flag: u16) -> bool {
        unsafe { SceneflagManager__checkFlagGlobal(SCENEFLAG_MANAGER, scn_idx, flag) }
    }
    pub fn set_global(scn_idx: u16, flag: u16) {
        unsafe { SceneflagManager__setFlagGlobal(SCENEFLAG_MANAGER, scn_idx, flag) };
    }
    pub fn set_local(flag: u16) {
        SceneflagManager::set_global(SceneflagManager::get_scene_idx(), flag);
    }
    pub fn unset_global(scn_idx: u16, flag: u16) {
        unsafe { SceneflagManager__unsetFlagGlobal(SCENEFLAG_MANAGER, scn_idx, flag) };
    }
    pub fn get_scene_flags() -> *const [u16] {
        let t = unsafe { SCENEFLAG_MANAGER.as_ref().unwrap() };
        return core::ptr::slice_from_raw_parts::<u16>(
            t.sceneflags.flag_ptr,
            t.sceneflags.flag_count.into(),
        );
    }
    pub fn get_temp_flags() -> *const [u16] {
        let t = unsafe { SCENEFLAG_MANAGER.as_ref().unwrap() };
        return core::ptr::slice_from_raw_parts::<u16>(
            t.tempflags.flag_ptr,
            t.tempflags.flag_count.into(),
        );
    }
    pub fn get_zone_flags() -> *const [u16] {
        let t = unsafe { SCENEFLAG_MANAGER.as_ref().unwrap() };
        return core::ptr::slice_from_raw_parts::<u16>(
            t.zoneflags.flag_ptr,
            t.zoneflags.flag_count.into(),
        );
    }
    pub fn set_zone_flag(room: u16, flag: u16, set: bool) {
        if set {
            unsafe { SceneflagManager__setZoneflag(SCENEFLAG_MANAGER, room, flag) };
        } else {
            unsafe { SceneflagManager__unsetZoneflag(SCENEFLAG_MANAGER, room, flag) };
        }
    }
    pub fn check_zone_flag(room: u16, flag: u16) -> bool {
        unsafe { SceneflagManager__checkZoneflag(SCENEFLAG_MANAGER, room, flag) }
    }
    pub fn get_scene_idx() -> u16 {
        unsafe { (*SCENEFLAG_MANAGER).scene_idx }
    }
}

impl DungeonflagManager {
    pub fn get_ptr() -> *mut DungeonflagManager {
        unsafe { DUNGEONFLAG_MANAGER }
    }
    /// returns the pointer to the static dungeonflags, those for the current
    /// sceneflagindex
    pub fn get_local() -> *mut [u16; 8] {
        unsafe { &mut STATIC_DUNGEON_FLAGS }
    }
    pub fn get_global(scn_idx: u16) -> *mut [u16; 8] {
        unsafe {
            (*file_manager::get_dungeon_flags())
                .as_mut_ptr()
                .add(scn_idx as usize)
        }
    }
    pub fn set_to_value(flag: u16, value: i32) {
        unsafe { DungeonflagManager__setToValue(DUNGEONFLAG_MANAGER, flag, value) }
    }
    pub fn get_global_key_count(scn_idx: u16) -> u16 {
        unsafe { (*Self::get_global(scn_idx))[1] & 0xF }
    }
}

pub fn copy_all_managers_from_save() {
    unsafe { FlagManger__copyAllFromSave() };
}
