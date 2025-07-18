use core::fmt::{Arguments, Write};

use alloc::vec::Vec;
use wchar::wchz;

use crate::system::{button::*, gx::Color};

use super::char_writer::{CharWriter, TextWriterBase};

const DEFAULT_LINE_SIZE: usize = 72;

pub struct SimpleMenu {
    pos:            [f32; 2],
    font_color:     Color,
    bg_color:       Color,
    max_view_lines: u32,
    cursor:         u32,
    heading:        CharWriter,
    pub lines:      Vec<CharWriter>,
    pub descriptions: Vec<&'static str>,
}

impl SimpleMenu {
    pub fn new() -> Self {
        Self {
            pos:            [10f32; 2],
            font_color:     Color::from_u32(0xFFFFFFFF),
            bg_color:       Color::from_u32(0x000000FF),
            max_view_lines: 10,
            cursor:         0,
            heading:        CharWriter::new(),
            lines:          Vec::new(),
            descriptions:   Vec::new(),
        }
    }

    pub fn set_pos(&mut self, posx: f32, posy: f32) {
        self.pos = [posx, posy];
    }

    pub fn set_font_color(&mut self, clr: u32) {
        self.font_color = Color::from_u32(clr);
    }

    pub fn set_bg_color(&mut self, clr: u32) {
        self.bg_color = Color::from_u32(clr);
    }

    pub fn set_max_line_count(&mut self, cnt: u32) {
        self.max_view_lines = cnt;
    }

    pub fn set_heading(&mut self, str: &str) {
        let _ = self.heading.write_str(str);
    }

    pub fn set_heading_fmt(&mut self, args: Arguments<'_>) {
        let _ = self.heading.write_fmt(args);
    }

    pub fn set_cursor(&mut self, cursor: u32) {
        self.cursor = cursor % self.lines.len() as u32;
    }

    pub fn add_entry(&mut self, str: &str, desc: &'static str) {
        let mut writer_entry = CharWriter::new();
        writer_entry.set_bg_color(self.bg_color.as_u32());
        writer_entry.set_font_color(self.font_color.as_u32());
        let _ = writer_entry.write_str(str);
        self.lines.push(writer_entry);
        self.descriptions.push(desc);
    }

    pub fn add_entry_fmt(&mut self, args: Arguments<'_>, desc: &'static str) {
        let mut writer_entry = CharWriter::new();
        writer_entry.set_bg_color(self.bg_color.as_u32());
        writer_entry.set_font_color(self.font_color.as_u32());
        let _ = writer_entry.write_fmt(args);
        self.lines.push(writer_entry);
        self.descriptions.push(desc);
    }

    pub fn move_cursor(&self) -> u32 {
        let len = self.lines.len() as u32;
        if is_pressed(DPAD_UP) || should_scroll(DPAD_UP) {
            (self.cursor + len - 1) % len
        } else if is_pressed(DPAD_DOWN) || should_scroll(DPAD_DOWN) {
            (self.cursor + 1) % len
        } else {
            self.cursor % len
        }
    }

    pub fn draw(&mut self) {
        let mut writer = TextWriterBase::new();
        writer.set_position(self.pos[0], self.pos[1]);

        // Draw Heading
        self.heading.draw(&mut writer);
        let mut pos = writer.get_cursor();
        pos[0] = 0f32;
        writer.set_cursor(pos);

        // Calc View Area
        //  get lower and upper range
        let range = self.max_view_lines as i32;
        let len = self.lines.len() as i32;
        let curr_line = self.cursor as i32;

        let (mut lower, mut upper) = (0, len);
        if len > range {
            let (try_low, try_high) = (curr_line - range / 2, curr_line + range / 2);
            if len > try_high && 0 < try_low {
                (lower, upper) = (try_low, try_high);
            } else {
                if len - curr_line > range / 2 {
                    (lower, upper) = (0, range);
                } else {
                    (lower, upper) = (len - range, len);
                }
            }
        }

        // Display Up arrow if needed
        if lower > 0 {
            writer.print_symbol(wchz!(u16, "\n3")); // up arrow
        } else {
            writer.print(wchz!(u16, "\n"));
        }

        // Draw Lines
        for n in lower..upper {
            // Set to beginning of line
            let mut pos = writer.get_cursor();
            pos[0] = 0f32;
            writer.set_cursor(pos);

            // Grab the writer
            let line = &mut self.lines[n as usize];

            // Set line color
            writer.print(wchz!(u16, "\n    "));
            if n == self.cursor as _ {
                line.set_font_color(0x00FF00FF);
                line.set_bg_color(self.bg_color.as_u32());
                crate::menus::main_menu::write_description(self.descriptions[n as usize]);
                // writer.print_symbol(wchz!(u16, "\n6")); // right arrow
            } else {
                line.set_font_color(self.font_color.as_u32());
                line.set_bg_color(self.bg_color.as_u32());
                // writer.print(wchz!(u16, "\n    "));
            }

            // draw the line
            line.draw(&mut writer);
        }

        // Display Down arrow if needed
        if upper < len {
            let mut pos = writer.get_cursor();
            pos[0] = 0f32;
            writer.set_cursor(pos);
            writer.print_symbol(wchz!(u16, "\n4")); // up arrow
        }
    }
}