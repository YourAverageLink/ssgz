use crate::game::reloader;
use crate::game::stage_info::*;
use crate::menus::main_menu::MainMenu;
use crate::system::button::*;
use crate::utils::menu::SimpleMenu;

use super::Menu;

#[derive(Copy, Clone, PartialEq, Eq)]
enum WarpState {
    Off,
    Main,
    Stage,
    Details,
}

const STAGES: [&StageCategory; 7] = [
    &THE_SKY,
    &FARON,
    &ELDIN,
    &LANAYRU,
    &SEALED_GROUNDS,
    &DUNGEONS,
    &SILENT_REALMS,
];

pub struct WarpMenu {
    state:             WarpState,
    stage_selected:    [u8; 8],
    main_cursor:       u32,
    stage_cursor:      u32,
    detail_cursor:     u32,
    selected_room:     u8,
    selected_layer:    u8,
    selected_entrance: u8,
}

impl WarpMenu {
    fn get_stages(&mut self) -> &'static [StageInfo] {
        if STAGES.len() <= self.main_cursor as usize {
            self.main_cursor = 0;
        }
        STAGES[self.main_cursor as usize].stages
    }

    fn get_stage(&mut self) -> &'static StageInfo {
        let stages = self.get_stages();
        if stages.len() <= self.stage_cursor as usize {
            self.stage_cursor = 0;
        }
        &stages[self.stage_cursor as usize]
    }

    fn get_rooms(&mut self) -> &'static [u8] {
        self.get_stage().rooms
    }

    fn get_layers(&mut self) -> &'static [u8] {
        self.get_stage().layers
    }

    fn get_room(&mut self) -> u8 {
        let rooms = self.get_rooms();
        if rooms.len() <= self.selected_room as usize {
            self.selected_room = 0;
        }
        rooms[self.selected_room as usize]
    }

    fn get_layer(&mut self) -> u8 {
        let layers = self.get_layers();
        if layers.len() <= self.selected_layer as usize {
            self.selected_layer = 0;
        }
        layers[self.selected_layer as usize]
    }

    fn get_entrance(&self) -> u8 {
        self.selected_entrance
    }

    fn warp(&mut self) {
        let stage_name = self.get_stage().name;
        for n in 0..8 {
            self.stage_selected[n] = if n < stage_name.len() {
                stage_name.as_bytes()[n] as u8
            } else {
                0
            };
        }
        let room = self.get_room();
        let layer = self.get_layer();
        let entrance = self.get_entrance();
        let forced_night: u8 = match self.main_cursor {
            0 => {
                if layer % 2 == 0 {
                    0
                } else {
                    1
                }
            },
            _ => 0,
        };
        let forced_trial: u8 = if self.stage_selected[0] == b'S' { 1 } else { 0 };
        let transition_type = 0;
        reloader::trigger_entrance(
            self.stage_selected.as_ptr(),
            room,
            layer,
            entrance,
            forced_night,
            forced_trial,
            transition_type,
            0xF,  // transition_fade_frames:  u8
            0xFF, // param_9: u8
        );
        reloader::set_reload_trigger(5);
    }

    fn change_room(&mut self, num: i8) {
        let num_rooms = self.get_rooms().len();
        self.selected_room =
            (self.selected_room as i8 + num_rooms as i8 + num) as u8 % num_rooms as u8;
    }
    fn change_layer(&mut self, num: i8) {
        let num_layers = self.get_layers().len();
        self.selected_layer =
            (self.selected_layer as i8 + num_layers as i8 + num) as u8 % num_layers as u8;
    }
    fn change_entrance(&mut self, num: i8) {
        self.selected_entrance = (self.selected_entrance as i8 + num) as u8;
    }
}

#[link_section = "data"]
#[no_mangle]
static mut WARP_MENU: WarpMenu = WarpMenu {
    state:             WarpState::Off,
    stage_selected:    [0u8; 8],
    main_cursor:       0,
    stage_cursor:      0,
    detail_cursor:     0,
    selected_room:     0,
    selected_layer:    0,
    selected_entrance: 0,
};

impl Menu for WarpMenu {
    fn enable() {
        let warp_menu = unsafe { &mut WARP_MENU };
        warp_menu.state = WarpState::Main;
    }
    fn disable() {
        let warp_menu = unsafe { &mut WARP_MENU };
        warp_menu.state = WarpState::Off;
    }

    fn is_active() -> bool {
        let warp_menu = unsafe { &mut WARP_MENU };
        warp_menu.state != WarpState::Off
    }

    fn input() {
        let warp_menu = unsafe { &mut WARP_MENU };

        let b_pressed = is_pressed(B);
        let a_pressed = is_pressed(A);
        let right_pressed = is_pressed(DPAD_RIGHT) || should_scroll(DPAD_RIGHT);
        let left_pressed = is_pressed(DPAD_LEFT) || should_scroll(DPAD_LEFT);

        match warp_menu.state {
            WarpState::Off => {},
            WarpState::Main => {
                if b_pressed {
                    warp_menu.state = WarpState::Off;
                } else if a_pressed {
                    warp_menu.state = WarpState::Stage;
                }
            },
            WarpState::Stage => {
                if b_pressed {
                    warp_menu.state = WarpState::Main;
                } else if a_pressed {
                    warp_menu.state = WarpState::Details;
                }
            },
            WarpState::Details => {
                if b_pressed {
                    warp_menu.state = WarpState::Stage;
                } else if a_pressed {
                    warp_menu.warp();
                    warp_menu.state = WarpState::Off;
                    MainMenu::disable();
                } else if right_pressed || left_pressed {
                    match warp_menu.detail_cursor {
                        0 => warp_menu.change_room(if right_pressed { 1 } else { -1 }),
                        1 => warp_menu.change_layer(if right_pressed { 1 } else { -1 }),
                        2 => warp_menu.change_entrance(if right_pressed { 1 } else { -1 }),
                        _ => {},
                    }
                }
            },
        }
    }

    fn display() {
        let warp_menu = unsafe { &mut WARP_MENU };

        match warp_menu.state {
            WarpState::Off => {},
            WarpState::Main => {
                let menu = crate::reset_menu();
                menu.set_heading("Warp Menu");
                for stage in STAGES {
                    menu.add_entry(stage.name, "Choose a stage in this area.");
                }

                menu.set_cursor(warp_menu.main_cursor);
                menu.draw();
                warp_menu.main_cursor = menu.move_cursor();
            },
            WarpState::Stage => {
                let stage_ref = STAGES[warp_menu.main_cursor as usize];
                let menu = crate::reset_menu();
                menu.set_heading(stage_ref.name);
                for stage in stage_ref.stages {
                    menu.add_entry(stage.pretty_name, "Warp to this stage.");
                }

                menu.set_cursor(warp_menu.stage_cursor);
                menu.draw();
                warp_menu.stage_cursor = menu.move_cursor();
            },
            WarpState::Details => {
                let menu = crate::reset_menu();
                menu.set_heading(warp_menu.get_stage().name);
                let (room, layer, entrance) = (
                    warp_menu.get_room(),
                    warp_menu.get_layer(),
                    warp_menu.get_entrance(),
                );
                menu.add_entry_fmt(format_args!("Room: {room}"), "Choose which Room to warp to.");
                menu.add_entry_fmt(format_args!("Layer: {layer}"), "Choose which layer to warp to.");
                menu.add_entry_fmt(format_args!("Entrance: {entrance}"), "Choose which entrance to warp to.");

                menu.set_cursor(warp_menu.detail_cursor);
                menu.draw();

                warp_menu.detail_cursor = menu.move_cursor();
            },
        }
    }
}
