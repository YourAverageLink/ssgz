#![allow(non_snake_case)]
use core::ffi::c_void;

#[repr(C)]
pub struct SpawnStruct {
    pub name:                   [u8; 32], // 0x00
    pub transition_fade_frames: u16,      // 0x20
    pub room:                   u8,       // 0x22
    pub layer:                  u8,       // 0x23
    pub entrance:               u8,       // 0x24
    pub night:                  u8,       // 0x25
    pub trial:                  u8,       // 0x26
    pub transition_type:        u8,       // 0x27
    pub field8_0x28:            u8,       // 0x28
    pub field9_0x29:            u8,       // 0x29
    pub field10_0x2a:           u8,       // 0x2A
    pub field11_0x2b:           u8,       // 0x2B
}

#[repr(C)]
pub struct Reloader {
    _0:                        [u8; 0x290],
    initial_speed:             f32,
    stamina_amount:            u32,
    item_to_use_on_reload:     u8,
    beedle_shop_spawn_state:   u8,
    spawn_state:               i16, // actionIndex
    last_area_type:            u32,
    type_0_pos_flag:           u8,
    unk:                       u8,
    save_prompt_flag:          u8,
    prevent_save_respawn_info: bool,
}

extern "C" {
    static mut SPAWN_MASTER: SpawnStruct;
    static mut SPAWN_SLAVE: SpawnStruct;
    static mut RELOADER_PTR: *mut Reloader;
    static mut RELOADER_TYPE: u8;
    fn RoomManager__getRoomByIndex(room_mgr: *mut c_void, room_number: u32);
    fn Reloader__setReloadTrigger(reloader: *mut Reloader, trigger: u8);
    fn actuallyTriggerEntrance(
        stage_name: *const u8,
        room: u8,
        layer: u8,
        entrance: u8,
        forced_night: u8,
        forced_trial: u8,
        transition_type: u8,
        transition_fade_frames: u8,
        param_9: u8,
    );
}

pub fn get_ptr() -> *mut Reloader {
    unsafe { RELOADER_PTR }
}

pub fn get_spawn_master() -> &'static mut SpawnStruct {
    return unsafe { &mut SPAWN_MASTER };
}

pub fn get_spawn_slave() -> &'static mut SpawnStruct {
    return unsafe { &mut SPAWN_SLAVE };
}

pub fn set_reload_trigger(trigger: u8) {
    unsafe { Reloader__setReloadTrigger(RELOADER_PTR, trigger) };
}

pub fn set_reloader_type(val: u8) {
    unsafe { RELOADER_TYPE = val };
}

pub fn set_save_prompt_flag() {
    unsafe { (*get_ptr()).save_prompt_flag = 1; }
}

pub fn trigger_entrance(
    stage_name: *const u8,
    room: u8,
    layer: u8,
    entrance: u8,
    forced_night: u8,
    forced_trial: u8,
    transition_type: u8,
    transition_fade_frames: u8,
    param_9: u8,
) {
    unsafe {
        actuallyTriggerEntrance(
            stage_name,
            room,
            layer,
            entrance,
            forced_night,
            forced_trial,
            transition_type,
            transition_fade_frames,
            param_9,
        )
    };
}
