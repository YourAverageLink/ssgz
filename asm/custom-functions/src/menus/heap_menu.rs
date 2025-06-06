// use core::ffi::c_double;
// use core::ffi::c_uint;
//
// use crate::system::button::*;
// use crate::system::heap::*;
// use crate::utils::menu::SimpleMenu;
//
// #[derive(Clone, Copy, PartialEq, Eq)]
// enum HeapMenuState {
// Off,
// Main,
// }
//
// pub struct HeapMenu {
// state:  HeapMenuState,
// cursor: u32,
// }
//
//
// #[no_mangle]
// #[link_section = "data"]
// static mut HEAP_MENU: HeapMenu = HeapMenu {
// state:  HeapMenuState::Off,
// cursor: 0,
// };
//
//
// impl super::Menu for HeapMenu {
// fn enable() {
// let heap_menu = unsafe { &mut HEAP_MENU };
// heap_menu.state = HeapMenuState::Main;
// }
//
// fn disable() {
// let heap_menu = unsafe { &mut HEAP_MENU };
// heap_menu.state = HeapMenuState::Off;
// }
// fn input() {
// let heap_menu = unsafe { &mut HEAP_MENU };
//
// match heap_menu.state {
// HeapMenuState::Off => {},
// HeapMenuState::Main => {
// if is_pressed(B) {
// heap_menu.state = HeapMenuState::Off;
// } else if is_pressed(A) {
// match heap_menu.cursor {
// 0 => {
// for i in 0..get_num_heaps() {
// if let Some(heap) = get_heap_idx(i) {
// let free_size = heap.get_free_size();
// let total_size = heap.get_total_size();
// let name = heap.get_name().as_ptr();
// let used_size = total_size - free_size;
// let percent_used =
// (used_size as f32) * 100f32 / total_size as f32;
// unsafe {
// crate::system::printf(b"Heap at idx %3d: %6.2f%% Used(%8d) Free(%8d)
// Total(%8d) Name(%s)\n\0".as_ptr() as _, i as c_uint, percent_used as
// c_double, used_size, free_size, total_size, name); }
// }
// }
// },
// _ => {},
// }
// }
// },
// }
// }
// fn display() {
// let heap_menu = unsafe { &mut HEAP_MENU };
//
// match heap_menu.state {
// HeapMenuState::Off => {},
// HeapMenuState::Main => {
// let mut menu: SimpleMenu<3> = SimpleMenu::new();
// menu.set_heading("Heap Menu");
// menu.set_cursor(heap_menu.cursor);
// menu.add_entry("Print Heap Info To Console");
// menu.draw();
// heap_menu.cursor = menu.move_cursor();
// },
// }
// }
//
// fn is_active() -> bool {
// let heap_menu = unsafe { &mut HEAP_MENU };
// heap_menu.state != HeapMenuState::Off
// }
// }
