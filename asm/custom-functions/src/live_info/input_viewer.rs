use crate::system::button::*;
use crate::utils::char_writer::TextWriterBase;
use crate::utils::menu::get_y_top;
use wchar::wchz;

struct StickDir {}
impl StickDir {
    fn display(writer: &mut TextWriterBase) {
        let pos = get_stick_pos();
        let (x, y) = (pos[0], pos[1]);
        if x < 0f32 && y < 0f32 {
            if x < y {
                writer.print_symbol(wchz!(u16, "\x2B"));
            } else {
                writer.print_symbol(wchz!(u16, "\x2A"));
            }
        } else if x < 0f32 && y >= 0f32 {
            if -x > y {
                writer.print_symbol(wchz!(u16, "\x2B"));
            } else {
                writer.print_symbol(wchz!(u16, "\x29"));
            }
        } else if x >= 0f32 && y < 0f32 {
            if x > -y {
                writer.print_symbol(wchz!(u16, "\x2C"));
            } else {
                writer.print_symbol(wchz!(u16, "\x2A"));
            }
        } else if x > 0f32 && y > 0f32 {
            if x > y {
                writer.print_symbol(wchz!(u16, "\x2C"));
            } else {
                writer.print_symbol(wchz!(u16, "\x29"));
            }
        }
    }
}

struct DPadDir {}
impl DPadDir {
    fn display(writer: &mut TextWriterBase) {
        if is_down(DPAD_UP) {
            writer.print_symbol(wchz!(u16, "\x2F"));
        } else if is_down(DPAD_DOWN) {
            writer.print_symbol(wchz!(u16, "\x30"));
        } else if is_down(DPAD_RIGHT) {
            writer.print_symbol(wchz!(u16, "\x32"));
        } else if is_down(DPAD_LEFT) {
            writer.print_symbol(wchz!(u16, "\x31"));
        }
    }
}

pub fn display() {
    let mut writer = TextWriterBase::new();
    writer.set_font_color(0xFFFFFFFF, 0xFFFFFFFF);
    writer.set_position(0f32, get_y_top() - 10.0);
    writer.set_scale(0.75f32);
    writer.print(wchz!(u16, "Inputs: "));
    StickDir::display(&mut writer);
    DPadDir::display(&mut writer);
    if is_down(A) {
        writer.print_symbol(wchz!(u16, "\x20"));
    }
    if is_down(B) {
        writer.print_symbol(wchz!(u16, "\x21"));
    }
    if is_down(PLUS) {
        writer.print_symbol(wchz!(u16, "\x23"));
    }
    if is_down(MINUS) {
        writer.print_symbol(wchz!(u16, "\x22"));
    }
    if is_down(C) {
        writer.print_symbol(wchz!(u16, "\x26"));
    }
    if is_down(Z) {
        writer.print_symbol(wchz!(u16, "\x27"));
    }
    if is_down(ONE) {
        writer.print_symbol(wchz!(u16, "\x24"));
    }
    if is_down(TWO) {
        writer.print_symbol(wchz!(u16, "\x25"));
    }
}
