pub mod actor;
pub mod arc;
pub mod bird;
pub mod events;
pub mod file_manager;
pub mod flag_managers;
pub mod item;
pub mod item_flags;
pub mod message;
pub mod minigame;
pub mod player;
pub mod reloader;
pub mod save_file;
pub mod stage_info;
pub mod collision;
pub mod enemy;
pub mod camera;

pub fn is_valid_game_ptr<T>(ptr: *mut T) -> bool {
    let addr = ptr as usize;
    (0x8000_0000..0x8180_0000).contains(&addr) || (0x9000_0000..0x9400_0000).contains(&addr)
}
