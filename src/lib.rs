#![no_std]

pub const IO_MAX_DEREF_COUNT: usize = 31;
pub const KINTERFACE_MIN_VERSION: u32 = (0x00 << 24) | (0x02 << 16) | (0x01 << 8) | (0x00 << 0);

pub mod requests;

mod pattern;
use alloc::{
    borrow::Cow,
    string::String,
};

pub use pattern::*;

extern crate alloc;

#[derive(Debug, Clone, Copy)]
pub struct ModuleInfo {
    pub base_dll_name: [u8; 0xFF],
    pub base_address: usize,
    pub module_size: usize,
}

impl ModuleInfo {
    pub fn base_dll_name(&self) -> Cow<'_, str> {
        let name_length = self.base_dll_name.iter().position(|char| *char == 0);
        String::from_utf8_lossy(&self.base_dll_name[0..name_length.unwrap_or(0)])
    }
}

impl Default for ModuleInfo {
    fn default() -> Self {
        Self {
            base_dll_name: [0u8; 0xFF],
            base_address: Default::default(),
            module_size: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ProcessModuleInfo {
    pub process_id: i32,
    pub module_count: usize,
}

#[derive(Debug, Default)]
pub struct MouseState {
    pub buttons: [Option<bool>; 0x05],
    pub hwheel: bool,
    pub wheel: bool,

    pub last_x: i32,
    pub last_y: i32,
}

#[derive(Debug, Default)]
pub struct KeyboardState {
    pub scane_code: u16,
    pub down: bool,
}
