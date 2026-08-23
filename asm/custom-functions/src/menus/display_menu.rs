use crate::game::camera;
use crate::live_info;
use crate::system::button::*;
use crate::utils::menu::SimpleMenu;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DisplayMenuState {
    Off,
    Main,
}

pub struct DisplayMenu {
    state:  DisplayMenuState,
    cursor: u32,
}

#[link_section = "data"]
#[no_mangle]
pub static mut DISPLAY_MENU: DisplayMenu = DisplayMenu {
    state:  DisplayMenuState::Off,
    cursor: 0,
};

#[link_section = "data"]
#[no_mangle]
pub static mut FREE_CAM_ENABLED: bool = false;

#[link_section = "data"]
#[no_mangle]
pub static mut FREE_CAM_RUNTIME_ACTIVE: bool = false;

#[link_section = "data"]
#[no_mangle]
pub static mut FREEZE_CAMERA_ENABLED: bool = false;

pub fn update_display_features() {
    unsafe {
        if FREE_CAM_ENABLED && ButtonBuffer::check_combo_pressed(Z | TWO) {
            FREE_CAM_RUNTIME_ACTIVE ^= true;
        }
        if !FREE_CAM_ENABLED || super::cheats_menu::is_move_link_runtime_active() {
            FREE_CAM_RUNTIME_ACTIVE = false;
        }
        camera::update(FREE_CAM_RUNTIME_ACTIVE, FREEZE_CAMERA_ENABLED);
    }
}

pub fn is_free_cam_runtime_active() -> bool {
    unsafe { FREE_CAM_RUNTIME_ACTIVE }
}

pub fn force_disable_free_cam() {
    unsafe {
        FREE_CAM_RUNTIME_ACTIVE = false;
    }
}

impl super::Menu for DisplayMenu {
    fn enable() {
        let disp_menu = unsafe { &mut DISPLAY_MENU };
        disp_menu.state = DisplayMenuState::Main;
    }

    fn disable() {
        let disp_menu = unsafe { &mut DISPLAY_MENU };
        disp_menu.state = DisplayMenuState::Off;
    }

    fn input() {
        let disp_menu = unsafe { &mut DISPLAY_MENU };
        match disp_menu.state {
            DisplayMenuState::Off => {},
            DisplayMenuState::Main => {
                if is_pressed(B) {
                    disp_menu.state = DisplayMenuState::Off;
                } else if is_pressed(A) {
                    unsafe {
                        match disp_menu.cursor {
                            0 => {
                                live_info::INPUT_VIEWER ^= true;
                            },
                            1 => {
                                live_info::LINK_POS_VIEWER ^= true;
                            },
                            2 => {
                                live_info::SCENE_FLAG_VIEWER ^= true;
                            },
                            3 => {
                                live_info::FRAME_VIEWER ^= true;
                            },
                            4 => {
                                FREE_CAM_ENABLED ^= true;
                                if !FREE_CAM_ENABLED {
                                    FREE_CAM_RUNTIME_ACTIVE = false;
                                }
                            },
                            5 => {
                                FREEZE_CAMERA_ENABLED ^= true;
                            },
                            _ => {},
                        }
                    }
                }
            },
        }
    }

    fn display() {
        let disp_menu = unsafe { &mut DISPLAY_MENU };
        let menu = crate::reset_menu();
        menu.set_heading("Display Menu");
        menu.add_entry_fmt(format_args!(
            "Input Viewer [{}]",
            if unsafe { live_info::INPUT_VIEWER } {
                "x"
            } else {
                " "
            }
        ),  if unsafe { live_info::INPUT_VIEWER } {
                "Inputs are currently shown."
            } else {
                "Inputs are currently hidden."
            }
        );
        menu.add_entry_fmt(format_args!(
            "Link Pos Viewer [{}]",
            if unsafe { live_info::LINK_POS_VIEWER } {
                "x"
            } else {
                " "
            }
        ),  if unsafe { live_info::LINK_POS_VIEWER } {
                "Link's position is currently shown."
            } else {
                "Link's position is currently hidden."
            }
        );
        menu.add_entry_fmt(format_args!(
            "Scene Flag Viewer [{}]",
            if unsafe { live_info::SCENE_FLAG_VIEWER } {
                "x"
            } else {
                " "
            }
        ),  if unsafe { live_info::SCENE_FLAG_VIEWER } {
                "Scene flags and temporary flags are currently shown."
            } else {
                "Scene flags and temporary flags are currently hidden."
            }
        );
        menu.add_entry_fmt(format_args!(
            "Frame Count Viewer [{}]",
            if unsafe { live_info::FRAME_VIEWER } {
                "x"
            } else {
                " "
            }
        ),  if unsafe { live_info::FRAME_VIEWER } {
                "The game's frame count is currently shown."
            } else {
                "The game's frame count is currently hidden."
            }
        );
        menu.add_entry_fmt(
            format_args!(
                "FreeCam [{}]",
                if unsafe { FREE_CAM_ENABLED } { "x" } else { " " }
            ),
            "Toggle with Z+2. Move with Stick, Hold C to rotate, Z for 5x speed, Z+Minus for 25x speed.",
        );
        menu.add_entry_fmt(
            format_args!(
                "Freeze Camera [{}]",
                if unsafe { FREEZE_CAMERA_ENABLED } { "x" } else { " " }
            ),
            "Freeze the camera at its current position.",
        );
        menu.set_cursor(disp_menu.cursor);
        menu.draw();
        disp_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        let disp_menu = unsafe { &mut DISPLAY_MENU };
        disp_menu.state != DisplayMenuState::Off
    }
}
