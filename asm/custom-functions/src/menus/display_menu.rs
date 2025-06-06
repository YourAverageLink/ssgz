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
                                live_info::INPUT_VIEWER = !live_info::INPUT_VIEWER;
                            },
                            1 => {
                                live_info::LINK_POS_VIEWER = !live_info::LINK_POS_VIEWER;
                            },
                            2 => {
                                live_info::SCENE_FLAG_VIEWER = !live_info::SCENE_FLAG_VIEWER;
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
        let mut menu: SimpleMenu<3> = SimpleMenu::new();
        menu.set_heading("Display Menu");
        menu.set_cursor(disp_menu.cursor);
        menu.add_entry_fmt(format_args!(
            "Input Viewer [{}]",
            if unsafe { live_info::INPUT_VIEWER } {
                "x"
            } else {
                " "
            }
        ));
        menu.add_entry_fmt(format_args!(
            "Link Pos Viewer [{}]",
            if unsafe { live_info::LINK_POS_VIEWER } {
                "x"
            } else {
                " "
            }
        ));
        menu.add_entry_fmt(format_args!(
            "Scene Flag Viewer [{}]",
            if unsafe { live_info::SCENE_FLAG_VIEWER } {
                "x"
            } else {
                " "
            }
        ));
        menu.draw();
        disp_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        let disp_menu = unsafe { &mut DISPLAY_MENU };
        disp_menu.state != DisplayMenuState::Off
    }
}
