#![feature(c_size_t)]
#![allow(dead_code)]

pub mod ffi;

pub mod n64types;
pub mod audio;
pub mod console;
pub mod debug;
pub mod joybus;
pub mod controller;
pub mod rtc;
pub mod mempak;
pub mod tpak;
pub mod display;
pub mod dma;
pub mod dragonfs;
pub mod eeprom;
pub mod graphics;
pub mod interrupt;
pub mod n64sys;
pub mod rdp;
pub mod rsp;
pub mod timer;
pub mod exception;
pub mod dir;
pub mod mixer;
pub mod samplebuffer;
pub mod wav64;
pub mod xm64;
pub mod ym64;
pub mod rspq;
pub mod surface;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
