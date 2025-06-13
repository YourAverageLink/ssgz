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
        menu.set_cursor(disp_menu.cursor);
        menu.draw();
        disp_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        let disp_menu = unsafe { &mut DISPLAY_MENU };
        disp_menu.state != DisplayMenuState::Off
    }
}
