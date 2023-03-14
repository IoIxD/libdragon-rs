use crate::ffi::bindings::{console_init, console_close, console_clear, console_render, console_set_render_mode, console_set_debug};

pub const TAB_WIDTH: u8 = 4;
pub const HORIZONTAL_PADDING: u8 = 4;
pub const VERTICAL_PADDING: u8 = 8;

pub enum RenderMode {
    Manual = 0,
    Automatic = 1,
}

pub const CONSOLE_WIDTH: u8 = 64;
pub const CONSOLE_HEIGHT: u8 = 28;

/// The console.
/// Since the actual console.c uses global variables and resets them
/// upon init being called, we have this dummy struct with methods
/// in order to make it more clear how this is normally used.
pub struct Console;

impl Console {
    pub fn new() -> Self {
        unsafe {
            console_init();
        };
        return Self{};
    }
    pub fn clear(&self) {
        unsafe {
            console_clear();
        }
    }
    pub fn render(&self) {
        unsafe {
            console_render();
        }
    }
    pub fn set_render_mode(&self, mode: RenderMode) {
        unsafe {
            console_set_render_mode(mode as i32);
        }
    }
    pub fn set_debug(&self, debug: bool) {
        unsafe {
            console_set_debug(debug);
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe {
            console_close();
        }
    }
}