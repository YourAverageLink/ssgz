use crate::system::{
    button::{
        buttons_down, buttons_pressed, get_stick_pos, is_down, set_buttons_down,
        set_buttons_pressed, set_stick_pos, A, B, Buttons, C, DPAD_DOWN, DPAD_LEFT, DPAD_RIGHT,
        MINUS, Z,
    },
    math::{self, Vec3f, Vec3s},
};
use crate::game::{is_valid_game_ptr, player};
use core::f32::consts::{FRAC_PI_2, TAU};

extern "C" {
    static mut SCN_ROOT_PTR: *mut u8;
    fn dScGame_c__getCamera(idx: i32) -> *mut DCamera;
    static mut EVENT_MANAGER_INSTANCE: *mut EventManager;
}

const SCNROOT_CURRENT_CAMERA_ID_OFFSET: usize = 0xF4;
const SCNROOT_CAMERA_ARRAY_OFFSET: usize = 0xF8;
const CAMERA_DATA_SIZE: usize = 0x218;
const MAX_CAMERA_COUNT: usize = 16;

const STICK_MOVE_SCALE: f32 = 72.0;
const STICK_ROT_SCALE: f32 = 59.0;
const FREE_CAM_ROTATION_SPEED: f32 = 0.002;
const FREE_CAM_SPEED: f32 = 0.2;
const FREE_CAM_FAST_SPEED: f32 = FREE_CAM_SPEED * 5.0;
const FREE_CAM_VERY_FAST_SPEED: f32 = FREE_CAM_SPEED * 25.0;
const FREE_CAM_VERTICAL_SCALE: f32 = 75.0;
const FREE_CAM_FOV_STEP: f32 = 0.8;
const FREE_CAM_FOV_FAST_MULT: f32 = 5.0;
const FREE_CAM_MIN_FOV: f32 = 1.0;
const FREE_CAM_MAX_FOV: f32 = 180.0;
const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.1;

#[repr(C)]
#[derive(Clone, Copy)]
struct CamView {
    pos:    Vec3f,
    target: Vec3f,
    fov:    f32,
    tilt:   f32,
}

#[repr(C)]
struct DCamera {
    _pad0:          [u8; 0x6C],
    view:           CamView,        // 0x6C
    _pad1:          [u8; 0xD98 - 0x8C],
    camera_objects: [*mut u8; 4],   // 0xD98
    active_camera:  i32,            // 0xDA8
    _pad2:          [u8; 0xDCC - 0xDAC],
    override_once:  u8,             // 0xDCC
    _pad3:          [u8; 3],
    override_view:  CamView,        // 0xDD0
}

#[repr(C)]
struct CameraData {
    _camera_mtx: [u8; 0x74],
    pos:         Vec3f,
    _up:         Vec3f,
    target:      Vec3f,
}

#[repr(C)]
struct EventManager {
    _pad0: [u8; 0x184],
    state: i32,
}

#[link_section = "data"]
#[no_mangle]
static mut FREE_CAM_INITIALIZED: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut FREE_CAM_PITCH: f32 = 0.0;

#[link_section = "data"]
#[no_mangle]
static mut FREE_CAM_YAW: f32 = 0.0;

#[link_section = "data"]
#[no_mangle]
static mut FREEZE_CAMERA_INITIALIZED: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut FREE_CAM_LINK_LOCKED: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut FREE_CAM_LINK_POS: Vec3f = Vec3f::zero();

#[link_section = "data"]
#[no_mangle]
static mut FREE_CAM_LINK_ROT: Vec3s = Vec3s { x: 0, y: 0, z: 0 };

#[link_section = "data"]
#[no_mangle]
static mut FREE_CAM_EVENT_STATE_OWNED: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut FREE_CAM_EVENT_STATE_ORIGINAL: i32 = 0;

#[link_section = "data"]
#[no_mangle]
static mut FROZEN_CAMERA_POS: Vec3f = Vec3f::zero();

#[link_section = "data"]
#[no_mangle]
static mut FROZEN_CAMERA_TARGET: Vec3f = Vec3f::zero();

#[link_section = "data"]
#[no_mangle]
static mut FROZEN_CAMERA_FOV: f32 = 50.0;

#[link_section = "data"]
#[no_mangle]
static mut FROZEN_CAMERA_TILT: f32 = 0.0;

#[link_section = "data"]
#[no_mangle]
static mut EXTERNAL_CAMERA_OVERRIDE_ACTIVE: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut EXTERNAL_CAMERA_POS: Vec3f = Vec3f::zero();

#[link_section = "data"]
#[no_mangle]
static mut EXTERNAL_CAMERA_TARGET: Vec3f = Vec3f::zero();

#[link_section = "data"]
#[no_mangle]
static mut EXTERNAL_CAMERA_RELEASE_TIMER: u8 = 0;

#[link_section = "data"]
#[no_mangle]
static mut PREV_FREE_CAM_ACTIVE: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut LAST_FREE_CAM_FOV: f32 = 50.0;

fn get_event_manager_mut() -> Option<&'static mut EventManager> {
    unsafe {
        if !is_valid_game_ptr(EVENT_MANAGER_INSTANCE) {
            return None;
        }
        EVENT_MANAGER_INSTANCE.as_mut()
    }
}

pub fn set_external_override(pos: Vec3f, target: Vec3f) {
    unsafe {
        EXTERNAL_CAMERA_POS = pos;
        EXTERNAL_CAMERA_TARGET = target;
        EXTERNAL_CAMERA_OVERRIDE_ACTIVE = true;
        EXTERNAL_CAMERA_RELEASE_TIMER = 0;
    }
}

pub fn clear_external_override() {
    unsafe {
        EXTERNAL_CAMERA_OVERRIDE_ACTIVE = false;
        EXTERNAL_CAMERA_RELEASE_TIMER = 0;
    }
}

pub fn clear_external_override_after(frames: u8) {
    unsafe {
        EXTERNAL_CAMERA_RELEASE_TIMER = frames;
    }
}

pub fn snap_external_camera_to_game(pos: Vec3f, target: Vec3f) {
    unsafe {
        EXTERNAL_CAMERA_OVERRIDE_ACTIVE = false;
        EXTERNAL_CAMERA_RELEASE_TIMER = 0;

        let mut view = current_view().unwrap_or(CamView {
            pos,
            target,
            fov: 50.0,
            tilt: 0.0,
        });
        view.pos = pos;
        view.target = target;

        if let Some(camera) = get_game_camera() {
            camera.view = view;
            camera.override_view = view;
            camera.override_once = 0;
        }
        sync_active_camera_driver_view(view);
        apply_pos_target_to_scnroot(pos, target);
    }
}

unsafe fn get_active_camera() -> Option<&'static mut CameraData> {
    let scn_root = SCN_ROOT_PTR as *mut u8;
    if scn_root.is_null() {
        return None;
    }

    let cam_idx = *scn_root.add(SCNROOT_CURRENT_CAMERA_ID_OFFSET) as usize;
    if cam_idx >= MAX_CAMERA_COUNT {
        return None;
    }

    let cam_ptr =
        scn_root.add(SCNROOT_CAMERA_ARRAY_OFFSET + cam_idx * CAMERA_DATA_SIZE) as *mut CameraData;
    Some(&mut *cam_ptr)
}

unsafe fn get_game_camera() -> Option<&'static mut DCamera> {
    let cam = dScGame_c__getCamera(0);
    if cam.is_null() {
        return None;
    }
    Some(&mut *cam)
}

fn initialize_free_cam_angles(cam: &CameraData) {
    let dx = cam.target.x - cam.pos.x;
    let dy = cam.target.y - cam.pos.y;
    let dz = cam.target.z - cam.pos.z;
    let horizontal = math::sqrt(dx * dx + dz * dz);
    unsafe {
        FREE_CAM_YAW = math::atan2(dz, dx);
        FREE_CAM_PITCH = math::atan2(dy, horizontal);
    }
}

fn consume_gameplay_input() {
    set_stick_pos([0.0, 0.0]);
    let mut pressed = buttons_pressed();
    pressed.remove(A | B | C | Z | MINUS | DPAD_DOWN | DPAD_LEFT | DPAD_RIGHT);
    set_buttons_pressed(pressed);
    let mut down = buttons_down();
    down.remove(A | B | C | Z | MINUS | DPAD_DOWN | DPAD_LEFT | DPAD_RIGHT);
    set_buttons_down(down);
}

fn apply_view_to_game_camera(view: CamView) {
    unsafe {
        if let Some(camera) = get_game_camera() {
            camera.override_view = view;
            camera.override_once = 1;
            camera.view = view;
        }
    }
}

fn sync_active_camera_driver_view(view: CamView) {
    unsafe {
        let Some(camera) = get_game_camera() else {
            return;
        };

        let active_camera = camera.active_camera as usize;
        if active_camera >= camera.camera_objects.len() {
            return;
        }

        let camera_object = camera.camera_objects[active_camera];
        if !is_valid_game_ptr(camera_object) {
            return;
        }

        let vtable_slot = camera_object.add(0x28) as *mut *mut usize;
        if !is_valid_game_ptr(vtable_slot) {
            return;
        }

        let vtable = *vtable_slot;
        if !is_valid_game_ptr(vtable) {
            return;
        }

        let set_view_ptr = *vtable.add(0x30 / 4) as *const ();
        if is_valid_game_ptr(set_view_ptr as *mut u8) {
            let set_view: unsafe extern "C" fn(*mut u8, *const CamView) =
                core::mem::transmute(set_view_ptr);
            set_view(camera_object, &view as *const CamView);
        } else {
            let get_view_ptr = *vtable.add(0x24 / 4) as *const ();
            if !is_valid_game_ptr(get_view_ptr as *mut u8) {
                return;
            }

            let get_view: unsafe extern "C" fn(*mut u8) -> *mut CamView =
                core::mem::transmute(get_view_ptr);
            let view_ptr = get_view(camera_object);
            if !is_valid_game_ptr(view_ptr) {
                return;
            }

            *view_ptr = view;
        }

        let flags = camera_object.add(0x58) as *mut u32;
        if is_valid_game_ptr(flags as *mut u8) {
            *flags |= 0x0000_4001;
        }
    }
}

fn clear_game_camera_override() {
    unsafe {
        if let Some(camera) = get_game_camera() {
            camera.override_once = 0;
        }
    }
}

fn apply_pos_target_to_scnroot(pos: Vec3f, target: Vec3f) {
    unsafe {
        if let Some(cam) = get_active_camera() {
            cam.pos = pos;
            cam.target = target;
        }
    }
}

fn lock_link_for_free_cam() {
    unsafe {
        if !FREE_CAM_LINK_LOCKED {
            if let Some(link) = player::as_mut() {
                FREE_CAM_LINK_POS = *link.position();
                FREE_CAM_LINK_ROT = *link.rotation();
                FREE_CAM_LINK_LOCKED = true;
            }
        }

        if !FREE_CAM_LINK_LOCKED {
            return;
        }

        player::force_set_link_pos_rot(&FREE_CAM_LINK_POS, &FREE_CAM_LINK_ROT);
        if let Some(link) = player::as_mut() {
            *link.position() = FREE_CAM_LINK_POS;
            *link.rotation() = FREE_CAM_LINK_ROT;
            link.obj_base.ac_base.position_copy = FREE_CAM_LINK_POS;
            link.obj_base.ac_base.rotation_copy = FREE_CAM_LINK_ROT;
            link.obj_base.old_position = FREE_CAM_LINK_POS;
            link.obj_base.position_copy2 = FREE_CAM_LINK_POS;
            link.obj_base.position_copy3 = FREE_CAM_LINK_POS;
            link.obj_base.angle = FREE_CAM_LINK_ROT;
            link.obj_base.speed = 0.0;
            link.obj_base.acceleration = 0.0;
            link.obj_base.velocity = Vec3f::zero();
        }
    }
}

fn set_free_cam_actor_freeze(active: bool) {
    unsafe {
        if active {
            if !FREE_CAM_EVENT_STATE_OWNED {
                if let Some(event_manager) = get_event_manager_mut() {
                    FREE_CAM_EVENT_STATE_ORIGINAL = event_manager.state;
                    if FREE_CAM_EVENT_STATE_ORIGINAL == 0 {
                        FREE_CAM_EVENT_STATE_OWNED = true;
                        event_manager.state = 1;
                    }
                }
            } else if let Some(event_manager) = get_event_manager_mut() {
                event_manager.state = 1;
            } else {
                FREE_CAM_EVENT_STATE_OWNED = false;
            }
        } else if FREE_CAM_EVENT_STATE_OWNED {
            if let Some(event_manager) = get_event_manager_mut() {
                event_manager.state = FREE_CAM_EVENT_STATE_ORIGINAL;
            }
            FREE_CAM_EVENT_STATE_OWNED = false;
        }
    }
}

fn current_view() -> Option<CamView> {
    unsafe {
        if let Some(camera) = get_game_camera() {
            return Some(camera.view);
        }
        get_active_camera().map(|cam| CamView {
            pos: cam.pos,
            target: cam.target,
            fov: 50.0,
            tilt: 0.0,
        })
    }
}

pub fn update(free_cam_active: bool, freeze_camera_active: bool) {
    let Some(mut view) = current_view() else {
        unsafe {
            FREE_CAM_INITIALIZED = false;
            FREEZE_CAMERA_INITIALIZED = false;
            PREV_FREE_CAM_ACTIVE = false;
            if FREE_CAM_EVENT_STATE_OWNED {
                if let Some(event_manager) = get_event_manager_mut() {
                    event_manager.state = FREE_CAM_EVENT_STATE_ORIGINAL;
                }
                FREE_CAM_EVENT_STATE_OWNED = false;
            }
        }
        return;
    };

    let apply_override: bool;
    let sync_camera_driver: bool;

    unsafe {
        if free_cam_active {
            set_free_cam_actor_freeze(true);
            if !FREE_CAM_INITIALIZED {
                let fake_cam = CameraData {
                    _camera_mtx: [0; 0x74],
                    pos: view.pos,
                    _up: Vec3f::zero(),
                    target: view.target,
                };
                initialize_free_cam_angles(&fake_cam);
                FREE_CAM_INITIALIZED = true;
            }

            let stick = get_stick_pos();
            let c_down = is_down(C);
            let vertical_displacement =
                (if is_down(A) { FREE_CAM_VERTICAL_SCALE } else { 0.0 }) -
                (if is_down(B) { FREE_CAM_VERTICAL_SCALE } else { 0.0 });

            let move_y = if c_down { 0.0 } else { stick[1] * STICK_MOVE_SCALE };
            let move_x = if c_down { 0.0 } else { stick[0] * STICK_MOVE_SCALE };
            let pitch_delta = if c_down { stick[1] * STICK_ROT_SCALE } else { 0.0 };
            let yaw_delta = if c_down { stick[0] * STICK_ROT_SCALE } else { 0.0 };

            let sin_pitch = math::sin(FREE_CAM_PITCH);
            let cos_pitch = math::cos(FREE_CAM_PITCH);
            let sin_yaw = math::sin(FREE_CAM_YAW);
            let cos_yaw = math::cos(FREE_CAM_YAW);

            let dy = move_y * sin_pitch + vertical_displacement;
            let dx = move_y * cos_yaw * cos_pitch - move_x * sin_yaw;
            let dz = move_y * sin_yaw * cos_pitch + move_x * cos_yaw;

            let speed = if is_down(Z) {
                if is_down(MINUS) {
                    FREE_CAM_VERY_FAST_SPEED
                } else {
                    FREE_CAM_FAST_SPEED
                }
            } else {
                FREE_CAM_SPEED
            };

            view.pos.x += speed * dx;
            view.pos.y += speed * dy;
            view.pos.z += speed * dz;

            // DPad-left: zoom out (higher FOV), DPad-right: zoom in (lower FOV).
            let fov_step = if is_down(Z) {
                FREE_CAM_FOV_STEP * FREE_CAM_FOV_FAST_MULT
            } else {
                FREE_CAM_FOV_STEP
            };
            if is_down(DPAD_LEFT) {
                view.fov += fov_step;
            }
            if is_down(DPAD_RIGHT) {
                view.fov -= fov_step;
            }
            view.fov = view.fov.clamp(FREE_CAM_MIN_FOV, FREE_CAM_MAX_FOV);

            view.target.x = view.pos.x + cos_yaw * cos_pitch;
            view.target.y = view.pos.y + sin_pitch;
            view.target.z = view.pos.z + sin_yaw * cos_pitch;

            FREE_CAM_YAW += yaw_delta * FREE_CAM_ROTATION_SPEED;
            if FREE_CAM_YAW >= TAU {
                FREE_CAM_YAW -= TAU;
            } else if FREE_CAM_YAW < 0.0 {
                FREE_CAM_YAW += TAU;
            }

            FREE_CAM_PITCH =
                (FREE_CAM_PITCH + pitch_delta * FREE_CAM_ROTATION_SPEED).clamp(-PITCH_LIMIT, PITCH_LIMIT);

            FROZEN_CAMERA_POS = view.pos;
            FROZEN_CAMERA_TARGET = view.target;
            FROZEN_CAMERA_FOV = view.fov;
            FROZEN_CAMERA_TILT = view.tilt;
            LAST_FREE_CAM_FOV = view.fov;

            lock_link_for_free_cam();
            consume_gameplay_input();
        } else {
            FREE_CAM_INITIALIZED = false;
            FREE_CAM_LINK_LOCKED = false;
            set_free_cam_actor_freeze(false);
        }

        if freeze_camera_active {
            if !FREEZE_CAMERA_INITIALIZED {
                FROZEN_CAMERA_POS = view.pos;
                FROZEN_CAMERA_TARGET = view.target;
                FROZEN_CAMERA_FOV =
                    if PREV_FREE_CAM_ACTIVE && !free_cam_active {
                        LAST_FREE_CAM_FOV
                    } else {
                        view.fov
                    };
                FROZEN_CAMERA_TILT = view.tilt;
                FREEZE_CAMERA_INITIALIZED = true;
            }
            view.pos = FROZEN_CAMERA_POS;
            view.target = FROZEN_CAMERA_TARGET;
            view.fov = FROZEN_CAMERA_FOV;
            view.tilt = FROZEN_CAMERA_TILT;
        } else {
            FREEZE_CAMERA_INITIALIZED = false;
        }

        let external_camera_override_active = EXTERNAL_CAMERA_OVERRIDE_ACTIVE;
        sync_camera_driver =
            external_camera_override_active && !free_cam_active && !freeze_camera_active;
        if external_camera_override_active && !free_cam_active && !freeze_camera_active {
            view.pos = EXTERNAL_CAMERA_POS;
            view.target = EXTERNAL_CAMERA_TARGET;
            if EXTERNAL_CAMERA_RELEASE_TIMER > 0 {
                EXTERNAL_CAMERA_RELEASE_TIMER -= 1;
                if EXTERNAL_CAMERA_RELEASE_TIMER == 0 {
                    EXTERNAL_CAMERA_OVERRIDE_ACTIVE = false;
                }
            }
        }

        apply_override = free_cam_active || freeze_camera_active || external_camera_override_active;
        PREV_FREE_CAM_ACTIVE = free_cam_active;
    }

    if apply_override {
        apply_view_to_game_camera(view);
        if sync_camera_driver {
            sync_active_camera_driver_view(view);
        }
        apply_pos_target_to_scnroot(view.pos, view.target);
    } else {
        clear_game_camera_override();
    }
}
