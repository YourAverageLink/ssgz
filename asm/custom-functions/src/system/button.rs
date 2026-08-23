// This file was adapted using https://github.com/CryZe/libtww-core/blob/master/src/game/gamepad.rs as guidance

#[repr(C)]
struct CoreController {
    pad0:              [u8; 0x18],
    buttons_down:      u32,         // 0x18 (CoreStatus.hold)
    buttons_pressed:   u32,         // 0x1C (CoreStatus.trig)
    buttons_released:  u32,         // 0x20 (CoreStatus.release)
    pad1:              [u8; 0x0C],  // 0x24..0x2F
    wiimote_acc_value: f32,         // 0x30 (CoreStatus.acc_value)
    _wiimote_acc_speed: f32,        // 0x34 (CoreStatus.acc_speed)
    pad2:              [u8; 0x3C],  // 0x38..0x73
    dev_type:          u8,          // 0x74 (CoreStatus.dev_type)
    pad3:              [u8; 0x03],  // 0x75..0x77
    free_stick_pos:    [f32; 2],    // 0x78 (CoreStatus.ex_status.fs.stick)
    pad4:              [u8; 0x0C],  // 0x80..0x8B
    fs_acc_value:      f32,         // 0x8C (CoreStatus.ex_status.fs.acc_value)
    _fs_acc_speed:     f32,         // 0x90 (CoreStatus.ex_status.fs.acc_speed)
}
extern "C" {
    static CORE_CONTROLLER: *mut CoreController;
}

const DEV_FREESTYLE: u8 = 1;
const DEV_MPLS_PT_FS: u8 = 6;

#[link_section = "data"]
static mut RAW_BUTTONS_DOWN_BITS: u32 = 0;
#[link_section = "data"]
static mut RAW_BUTTONS_PRESSED_BITS: u32 = 0;
#[link_section = "data"]
static mut RAW_STICK_POS: [f32; 2] = [0.0, 0.0];
#[link_section = "data"]
static mut RAW_WIIMOTE_SHAKE: f32 = 0.0;
#[link_section = "data"]
static mut RAW_NUNCHUCK_SHAKE: f32 = 0.0;

bitflags::bitflags! {
    #[derive(Copy, Clone)]
    pub struct Buttons: u32 {
        const DPAD_LEFT = 0x0001;
        const DPAD_RIGHT = 0x0002;
        const DPAD_DOWN = 0x0004;
        const DPAD_UP = 0x0008;
        const PLUS = 0x0010;
        const TWO = 0x0100;
        const ONE = 0x0200;
        const B = 0x0400;
        const A = 0x0800;
        const MINUS = 0x1000;
        const Z = 0x2000;
        const C = 0x4000;
        const HOME = 0x8000;
    }
}

pub const DPAD_LEFT: Buttons = Buttons::DPAD_LEFT;
pub const DPAD_RIGHT: Buttons = Buttons::DPAD_RIGHT;
pub const DPAD_DOWN: Buttons = Buttons::DPAD_DOWN;
pub const DPAD_UP: Buttons = Buttons::DPAD_UP;
pub const PLUS: Buttons = Buttons::PLUS;
pub const TWO: Buttons = Buttons::TWO;
pub const ONE: Buttons = Buttons::ONE;
pub const B: Buttons = Buttons::B;
pub const A: Buttons = Buttons::A;
pub const MINUS: Buttons = Buttons::MINUS;
pub const Z: Buttons = Buttons::Z;
pub const C: Buttons = Buttons::C;
pub const HOME: Buttons = Buttons::HOME;

pub fn buttons_down() -> Buttons {
    unsafe { Buttons::from_bits_truncate((*CORE_CONTROLLER).buttons_down) }
}

pub fn buttons_pressed() -> Buttons {
    unsafe { Buttons::from_bits_truncate((*CORE_CONTROLLER).buttons_pressed) }
}

pub fn set_buttons_down(buttons: Buttons) {
    unsafe {
        (*CORE_CONTROLLER).buttons_down = buttons.bits();
    }
}

pub fn set_buttons_pressed(buttons: Buttons) {
    unsafe {
        (*CORE_CONTROLLER).buttons_pressed = buttons.bits();
    }
}
pub fn set_buttons_not_pressed(buttons: Buttons) {
    unsafe {
        (*CORE_CONTROLLER).buttons_pressed &= !buttons.bits();
    }
}

pub fn is_down(buttons: Buttons) -> bool {
    buttons_down().contains(buttons)
}

pub fn is_pressed(buttons: Buttons) -> bool {
    buttons_pressed().contains(buttons)
}

pub fn is_any_down(buttons: Buttons) -> bool {
    buttons_down().intersects(buttons)
}

pub fn is_any_pressed(buttons: Buttons) -> bool {
    buttons_pressed().intersects(buttons)
}

pub fn get_stick_pos() -> [f32; 2] {
    unsafe { (*CORE_CONTROLLER).free_stick_pos }
}

pub fn raw_buttons_down() -> Buttons {
    unsafe { Buttons::from_bits_truncate(RAW_BUTTONS_DOWN_BITS) }
}

pub fn raw_buttons_pressed() -> Buttons {
    unsafe { Buttons::from_bits_truncate(RAW_BUTTONS_PRESSED_BITS) }
}

pub fn raw_stick_pos() -> [f32; 2] {
    unsafe { RAW_STICK_POS }
}

pub fn raw_wiimote_shake() -> f32 {
    unsafe { RAW_WIIMOTE_SHAKE }
}

pub fn raw_nunchuck_shake() -> f32 {
    unsafe { RAW_NUNCHUCK_SHAKE }
}

pub fn set_stick_pos(stick_pos: [f32; 2]) {
    unsafe {
        (*CORE_CONTROLLER).free_stick_pos = stick_pos;
    }
}

// Mainly for d-pad directions in menus
pub fn should_scroll(button: Buttons) -> bool {
    let frames = ButtonBuffer::num_frames_held(button);
    if frames >= 60 {
        return frames & 1 == 0;
    }
    if frames >= 30 {
        return frames & 3 == 0;
    }
    if frames >= 8 {
        return frames & 7 == 0;
    }

    false
}

pub struct ButtonBuffer {
    pub frames_down: [u16; 16], // Number of frames each button has been held for
}

pub static mut BUTTON_BUFFER: ButtonBuffer = ButtonBuffer {
    frames_down: [0u16; 16],
};

impl ButtonBuffer {
    fn get_buf() -> &'static ButtonBuffer {
        unsafe { &BUTTON_BUFFER }
    }
    fn get_buf_mut() -> &'static mut ButtonBuffer {
        unsafe { &mut BUTTON_BUFFER }
    }
    pub fn update() {
        unsafe {
            RAW_BUTTONS_DOWN_BITS = (*CORE_CONTROLLER).buttons_down;
            RAW_BUTTONS_PRESSED_BITS = (*CORE_CONTROLLER).buttons_pressed;
            RAW_STICK_POS = (*CORE_CONTROLLER).free_stick_pos;
            let mut wiimote_shake = (*CORE_CONTROLLER).wiimote_acc_value - 1.0;
            if wiimote_shake < 0.0 {
                wiimote_shake = -wiimote_shake;
            }
            if !wiimote_shake.is_finite() {
                wiimote_shake = 0.0;
            }

            let dev_type = (*CORE_CONTROLLER).dev_type;
            let mut nunchuck_shake =
                if dev_type == DEV_FREESTYLE || dev_type == DEV_MPLS_PT_FS {
                    let mut v = (*CORE_CONTROLLER).fs_acc_value - 1.0;
                    if v < 0.0 {
                        v = -v;
                    }
                    v
                } else {
                    0.0
                };
            if !nunchuck_shake.is_finite() {
                nunchuck_shake = 0.0;
            }

            RAW_WIIMOTE_SHAKE = wiimote_shake;
            RAW_NUNCHUCK_SHAKE = nunchuck_shake;
        }
        let buf = Self::get_buf_mut();
        let down = buttons_down();
        let up = down.complement();
        for btn in down.iter() {
            buf.frames_down[btn.bits().trailing_zeros() as usize] += 1;
        }
        for btn in up.iter() {
            buf.frames_down[btn.bits().trailing_zeros() as usize] = 0;
        }
    }

    pub fn num_frames_held(buttons: Buttons) -> u16 {
        let buf = Self::get_buf();
        buttons
            .iter()
            .map(|btn| buf.frames_down[btn.bits().trailing_zeros() as usize])
            .min()
            .unwrap_or(0)
    }

    pub fn check_combo_down(buttons: Buttons) -> bool {
        Self::num_frames_held(buttons) > 0
    }

    pub fn check_combo_pressed(buttons: Buttons) -> bool {
        Self::num_frames_held(buttons) == 1
    }

    pub fn check_combo_down_up(down: Buttons, up: Buttons) -> bool {
        Self::check_combo_down(down) && !Self::check_combo_down(up)
    }

    pub fn check_combo_pressed_up(pressed: Buttons, up: Buttons) -> bool {
        Self::check_combo_pressed(pressed) && !Self::check_combo_down(up)
    }
}
