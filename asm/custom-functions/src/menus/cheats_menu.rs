use crate::{
    game::{camera, file_manager, flag_managers::ItemflagManager, is_valid_game_ptr, player},
    system::{button::*, math::{self, Vec3f, Vec3s}},
    utils::menu::SimpleMenu,
};

pub struct Cheat {
    name:   &'static str,
    description: &'static str,
    active: bool,
}

extern "C" {
    fn set_instant_text(active: bool);
    fn set_ui_hidden(hidden: bool);
}

#[no_mangle]
#[link_section = "data"]
pub static mut CHEATS: [Cheat; 10] = [
    Cheat {
        name:   "Infinite Health",
        description: "Constantly refills health to 20 hearts (unless you're already dead).",
        active: false,
    },
    Cheat {
        name:   "Infinite Stamina",
        description: "Constantly refills stamina and underwater air to full.",
        active: false,
    },
    Cheat {
        name:   "Infinite Slingshot Seeds",
        description: "Constantly refills Slingshot Seeds to full.",
        active: false,
    },
    Cheat {
        name:   "Infinite Bombs",
        description: "Constantly refills Bombs to full.",
        active: false,
    },
    Cheat {
        name:   "Infinite Arrows",
        description: "Constantly refills Arrows to full.",
        active: false,
    },
    Cheat {
        name:   "Infinite Rupees",
        description: "Constantly refills Rupees to 9900.",
        active: false,
    },
    Cheat {
        name:   "Moon Jump while holding D-Pad Right",
        description: "Applies an upward velocity of 56 units to Link while holding D-Pad Right.",
        active: false,
    },
    Cheat {
        name:   "Move Link",
        description: "(Z + A + B) C+left/right rotate, C+up/down vertical, Z fast, Z+Minus very fast.",
        active: false,
    },
    //Cheat {
    //    name:   "Super Speed",
    //    active: false,
    //},
    Cheat {
        name:   "Instant Text",
        description: "Instantly fills in all text in text boxes.",
        active: false,
    },
    Cheat {
        name:   "Hide UI",
        description: "Hides most of the in-game UI.",
        active: false,
    },
];

const CHEAT_INFINITE_HEALTH: usize = 0;
const CHEAT_INFINITE_STAMINA: usize = 1;
const CHEAT_INFINITE_SLINGSHOT: usize = 2;
const CHEAT_INFINITE_BOMBS: usize = 3;
const CHEAT_INFINITE_ARROWS: usize = 4;
const CHEAT_INFINITE_RUPEES: usize = 5;
const CHEAT_MOON_JUMP: usize = 6;
const CHEAT_MOVE_LINK: usize = 7;
const CHEAT_INSTANT_TEXT: usize = 8;
const CHEAT_HIDE_UI: usize = 9;

const STICK_MOVE_SCALE: f32 = 72.0;
const STICK_ROT_SCALE: f32 = 59.0;
const MOVE_LINK_ROTATION_SPEED: f32 = 0.002;
const MOVE_LINK_SPEED: f32 = 0.2;
const MOVE_LINK_FAST_SPEED: f32 = MOVE_LINK_SPEED * 5.0;
const MOVE_LINK_VERY_FAST_SPEED: f32 = MOVE_LINK_SPEED * 25.0;
const MOVE_LINK_VERTICAL_SCALE: f32 = 75.0;
const MOVE_LINK_CAMERA_DIST: f32 = 600.0;
const MOVE_LINK_CAMERA_HEIGHT: f32 = 200.0;

const ACCH_FLAG_GRND_NONE: u32 = 1 << 1;
const ACCH_FLAG_WALL_NONE: u32 = 1 << 2;
const ACCH_FLAG_ROOF_NONE: u32 = 1 << 3;
const ACCH_FLAG_WALL_HIT: u32 = 1 << 4;
const ACCH_FLAG_GROUND_HIT: u32 = 1 << 5;
const ACCH_FLAG_GROUND_FIND: u32 = 1 << 6;
const ACCH_FLAG_GROUND_LANDING: u32 = 1 << 7;
const ACCH_FLAG_GROUND_AWAY: u32 = 1 << 8;
const ACCH_FLAG_ROOF_HIT: u32 = 1 << 9;
const ACCH_FLAG_LINE_CHECK: u32 = 1 << 13;
const ACCH_FLAG_LINE_CHECK_NONE: u32 = 1 << 14;
const ACCH_FLAG_LINE_CHECK_HIT: u32 = 1 << 16;

const MOVE_LINK_ACCH_SET_MASK: u32 =
    ACCH_FLAG_GRND_NONE | ACCH_FLAG_WALL_NONE | ACCH_FLAG_ROOF_NONE | ACCH_FLAG_LINE_CHECK_NONE;
const MOVE_LINK_ACCH_CLEAR_MASK: u32 = ACCH_FLAG_WALL_HIT
    | ACCH_FLAG_GROUND_HIT
    | ACCH_FLAG_GROUND_FIND
    | ACCH_FLAG_GROUND_LANDING
    | ACCH_FLAG_GROUND_AWAY
    | ACCH_FLAG_ROOF_HIT
    | ACCH_FLAG_LINE_CHECK
    | ACCH_FLAG_LINE_CHECK_HIT;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_INITIALIZED: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_YAW: f32 = 0.0;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_ORIGINAL_ACCH_FLAGS: u32 = 0;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_PLAYER_PTR: usize = 0;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_POS: Vec3f = Vec3f::zero();

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_EVENT_STATE_OWNED: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_EVENT_STATE_ORIGINAL: i32 = 0;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_CAMERA_PLAY_OWNED: bool = false;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_CAMERA_PLAY_ORIGINAL: i32 = 0;

#[link_section = "data"]
#[no_mangle]
static mut MOVE_LINK_RUNTIME_ACTIVE: bool = false;

#[repr(C)]
struct EventManager {
    _pad0: [u8; 0x184],
    state: i32,
    _pad1: [u8; 4],
    camera_play: i32,
}

extern "C" {
    static mut EVENT_MANAGER_INSTANCE: *mut EventManager;
}

fn get_event_manager_mut() -> Option<&'static mut EventManager> {
    unsafe {
        if !is_valid_game_ptr(EVENT_MANAGER_INSTANCE) {
            return None;
        }
        EVENT_MANAGER_INSTANCE.as_mut()
    }
}

pub fn is_move_link_runtime_active() -> bool {
    unsafe { MOVE_LINK_RUNTIME_ACTIVE }
}

pub fn force_disable_move_link() {
    unsafe {
        MOVE_LINK_RUNTIME_ACTIVE = false;
    }
    update_move_link(false);
    camera::clear_external_override();
    set_move_link_camera_play(false);
}

#[derive(PartialEq, Eq)]
enum MenuState {
    Off,
    Main,
}

pub struct CheatsMenu {
    state:  MenuState,
    cursor: u32,
}

#[no_mangle]
#[link_section = "data"]
static mut CHEAT_MENU: CheatsMenu = CheatsMenu {
    state:  MenuState::Off,
    cursor: 0,
};

fn consume_move_link_input() {
    set_stick_pos([0.0, 0.0]);
    let mut down = buttons_down();
    down.remove(A | B | C | Z | MINUS);
    set_buttons_down(down);
    let mut pressed = buttons_pressed();
    pressed.remove(A | B | C | Z | MINUS);
    set_buttons_pressed(pressed);
}

fn apply_transform_to_actor(actor: &mut crate::game::actor::AcObjBase, pos: Vec3f, rot: Vec3s) {
    actor.ac_base.position = pos;
    actor.ac_base.rotation = rot;
    actor.ac_base.position_copy = pos;
    actor.ac_base.rotation_copy = rot;
}

fn get_move_link_camera_view(pos: Vec3f, yaw: f32) -> (Vec3f, Vec3f) {
    let camera_target = Vec3f {
        x: pos.x,
        y: pos.y + MOVE_LINK_CAMERA_HEIGHT,
        z: pos.z,
    };
    let camera_pos = Vec3f {
        x: pos.x - MOVE_LINK_CAMERA_DIST * math::sin(yaw),
        y: pos.y + MOVE_LINK_CAMERA_HEIGHT,
        z: pos.z - MOVE_LINK_CAMERA_DIST * math::cos(yaw),
    };
    (camera_pos, camera_target)
}

fn set_move_link_camera_override(pos: Vec3f, yaw: f32) {
    let (camera_pos, camera_target) = get_move_link_camera_view(pos, yaw);
    camera::set_external_override(camera_pos, camera_target);
}

fn snap_move_link_camera(pos: Vec3f, yaw: f32) {
    let (camera_pos, camera_target) = get_move_link_camera_view(pos, yaw);
    camera::snap_external_camera_to_game(camera_pos, camera_target);
}

fn set_move_link_camera_play(active: bool) {
    unsafe {
        if active {
            if !MOVE_LINK_CAMERA_PLAY_OWNED {
                if let Some(event_manager) = get_event_manager_mut() {
                    MOVE_LINK_CAMERA_PLAY_ORIGINAL = event_manager.camera_play;
                    MOVE_LINK_CAMERA_PLAY_OWNED = true;
                }
            }
            if let Some(event_manager) = get_event_manager_mut() {
                event_manager.camera_play = 1;
            } else {
                MOVE_LINK_CAMERA_PLAY_OWNED = false;
            }
        } else if MOVE_LINK_CAMERA_PLAY_OWNED {
            if let Some(event_manager) = get_event_manager_mut() {
                event_manager.camera_play = MOVE_LINK_CAMERA_PLAY_ORIGINAL;
            }
            MOVE_LINK_CAMERA_PLAY_OWNED = false;
        }
    }
}

fn update_move_link(active: bool) {
    unsafe {
        if !active {
            if MOVE_LINK_INITIALIZED {
                if let Some(link) = player::as_mut() {
                    let current_ptr = link as *mut _ as usize;
                    if current_ptr == MOVE_LINK_PLAYER_PTR {
                        link.acch.flags = MOVE_LINK_ORIGINAL_ACCH_FLAGS;
                    }
                }
                MOVE_LINK_PLAYER_PTR = 0;
                MOVE_LINK_INITIALIZED = false;
                snap_move_link_camera(MOVE_LINK_POS, MOVE_LINK_YAW);
            }
            set_move_link_camera_play(false);
            if MOVE_LINK_EVENT_STATE_OWNED {
                if let Some(event_manager) = get_event_manager_mut() {
                    event_manager.state = MOVE_LINK_EVENT_STATE_ORIGINAL;
                }
                MOVE_LINK_EVENT_STATE_OWNED = false;
            }
            return;
        }

        let Some(link) = player::as_mut() else {
            return;
        };
        let link_ptr = link as *mut _ as usize;
        let riding_actor_ptr = link
            .get_riding_actor()
            .map(|actor| actor as *mut crate::game::actor::AcObjBase)
            .unwrap_or(core::ptr::null_mut());

        if !MOVE_LINK_INITIALIZED || MOVE_LINK_PLAYER_PTR != link_ptr {
            MOVE_LINK_YAW = math::short_to_rad(link.rotation().y);
            MOVE_LINK_ORIGINAL_ACCH_FLAGS = link.acch.flags;
            MOVE_LINK_PLAYER_PTR = link_ptr;
            MOVE_LINK_POS = if riding_actor_ptr.is_null() {
                *link.position()
            } else {
                (*riding_actor_ptr).ac_base.position
            };
            MOVE_LINK_INITIALIZED = true;
        }

        // Freezing event state while riding can destabilize rider/mount ownership.
        let should_freeze_actors = riding_actor_ptr.is_null();
        set_move_link_camera_play(true);
        if should_freeze_actors {
            if !MOVE_LINK_EVENT_STATE_OWNED {
                if let Some(event_manager) = get_event_manager_mut() {
                    MOVE_LINK_EVENT_STATE_ORIGINAL = event_manager.state;
                    if MOVE_LINK_EVENT_STATE_ORIGINAL == 0 {
                        event_manager.state = 1;
                        MOVE_LINK_EVENT_STATE_OWNED = true;
                    }
                }
            } else if let Some(event_manager) = get_event_manager_mut() {
                event_manager.state = 1;
            } else {
                MOVE_LINK_EVENT_STATE_OWNED = false;
            }
        } else if MOVE_LINK_EVENT_STATE_OWNED {
            if let Some(event_manager) = get_event_manager_mut() {
                event_manager.state = MOVE_LINK_EVENT_STATE_ORIGINAL;
            }
            MOVE_LINK_EVENT_STATE_OWNED = false;
        }

        let stick = get_stick_pos();
        let c_down = is_down(C);
        let vertical_displacement = if c_down { stick[1] * MOVE_LINK_VERTICAL_SCALE } else { 0.0 };

        let move_y = if c_down { 0.0 } else { stick[1] * STICK_MOVE_SCALE };
        let move_x = if c_down { 0.0 } else { -stick[0] * STICK_MOVE_SCALE };
        let yaw_delta = if c_down { -stick[0] * STICK_ROT_SCALE } else { 0.0 };

        MOVE_LINK_YAW = math::normalize_angle(MOVE_LINK_YAW + yaw_delta * MOVE_LINK_ROTATION_SPEED);
        let sin_yaw = math::sin(MOVE_LINK_YAW);
        let cos_yaw = math::cos(MOVE_LINK_YAW);

        let dx = move_y * sin_yaw + move_x * cos_yaw;
        let dz = move_y * cos_yaw - move_x * sin_yaw;

        let speed = if is_down(Z) {
            if is_down(MINUS) {
                MOVE_LINK_VERY_FAST_SPEED
            } else {
                MOVE_LINK_FAST_SPEED
            }
        } else {
            MOVE_LINK_SPEED
        };

        let mut pos = MOVE_LINK_POS;
        pos.x += speed * dx;
        pos.y += speed * vertical_displacement;
        pos.z += speed * dz;
        MOVE_LINK_POS = pos;

        let mut rot = *link.rotation();
        rot.y = math::rad_to_short(MOVE_LINK_YAW);

        player::force_set_link_pos_rot(&pos, &rot);
        *link.position() = pos;
        *link.rotation() = rot;
        link.obj_base.ac_base.position_copy = pos;
        link.obj_base.ac_base.rotation_copy = rot;

        link.acch.flags |= MOVE_LINK_ACCH_SET_MASK;
        link.acch.flags &= !MOVE_LINK_ACCH_CLEAR_MASK;

        if !riding_actor_ptr.is_null() {
            apply_transform_to_actor(&mut *riding_actor_ptr, pos, rot);
        }

        set_move_link_camera_override(pos, MOVE_LINK_YAW);

        consume_move_link_input();
    }
}

impl super::Menu for CheatsMenu {
    fn enable() {
        unsafe { CHEAT_MENU.state = MenuState::Main };
    }

    fn disable() {
        unsafe { CHEAT_MENU.state = MenuState::Off };
    }

    fn input() {
        let cheats_menu: &mut CheatsMenu = unsafe { &mut CHEAT_MENU };

        match cheats_menu.state {
            MenuState::Off => {},
            MenuState::Main => {
                if is_pressed(B) {
                    CheatsMenu::disable();
                } else if is_pressed(A) {
                    unsafe {
                        CHEATS[cheats_menu.cursor as usize].active ^= true;
                    }
                }
            },
        }
    }

    fn display() {
        let cheats_menu: &mut CheatsMenu = unsafe { &mut CHEAT_MENU };

        let menu = crate::reset_menu();
        menu.set_heading("Cheats");
        for cheat in unsafe { &CHEATS } {
            menu.add_entry_fmt(format_args!(
                "{} [{}]",
                cheat.name,
                if cheat.active { "x" } else { "" }
            ), cheat.description)
        }
        menu.set_cursor(cheats_menu.cursor);
        menu.draw();
        cheats_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        unsafe { CHEAT_MENU.state != MenuState::Off }
    }
}

pub fn update_cheats() {
    unsafe {
        if CHEATS[CHEAT_MOVE_LINK].active && ButtonBuffer::check_combo_pressed(Z | A | B) {
            if MOVE_LINK_RUNTIME_ACTIVE {
                MOVE_LINK_RUNTIME_ACTIVE = false;
            } else if !super::display_menu::is_free_cam_runtime_active() {
                MOVE_LINK_RUNTIME_ACTIVE = true;
            }
        }
        if !CHEATS[CHEAT_MOVE_LINK].active {
            MOVE_LINK_RUNTIME_ACTIVE = false;
        }

        if CHEATS[CHEAT_INFINITE_HEALTH].active {
            // Don't overwrite 0 health (so the Kill Link action still works)
            if file_manager::get_current_health() != 0 {
                file_manager::set_current_health(80);
            }
        }
        if CHEATS[CHEAT_INFINITE_STAMINA].active {
            if let Some(player) = player::as_mut() {
                player.stamina_amount = 1_000_000;
            }
        }
        if CHEATS[CHEAT_INFINITE_SLINGSHOT].active && ItemflagManager::get_counter_by_index(4) < 20 {
            ItemflagManager::increase_counter(4, 20);
        }
        if CHEATS[CHEAT_INFINITE_BOMBS].active && ItemflagManager::get_counter_by_index(2) < 10 {
            ItemflagManager::increase_counter(2, 10);
        }
        if CHEATS[CHEAT_INFINITE_ARROWS].active && ItemflagManager::get_counter_by_index(1) < 20 {
            ItemflagManager::increase_counter(1, 20);
        }
        if CHEATS[CHEAT_INFINITE_RUPEES].active && ItemflagManager::get_counter_by_index(0) < 9900 {
            ItemflagManager::increase_counter(0, 9900);
        }
        if CHEATS[CHEAT_MOON_JUMP].active && ButtonBuffer::check_combo_down_up(DPAD_RIGHT, C) {
            if let Some(player) = player::as_mut() {
                player.obj_base.velocity.y = 56f32; // Minimum amount for consistent liftoff on the ground
            }
        }
        update_move_link(MOVE_LINK_RUNTIME_ACTIVE);
        set_instant_text(CHEATS[CHEAT_INSTANT_TEXT].active);
        set_ui_hidden(CHEATS[CHEAT_HIDE_UI].active);
    }
}
