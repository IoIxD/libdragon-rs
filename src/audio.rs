
use core::ffi::c_size_t;
use std::ffi::c_short;

use libffi::high::{Closure2};

use crate::ffi::bindings::{audio_init,audio_set_buffer_callback,audio_pause,audio_write,audio_write_silence,audio_close,audio_get_frequency,audio_get_buffer_length};

/// Number of buffers the audio subsystem allocates and manages.
pub const NUM_BUFFERS: u8 = 4;

/// How many different audio buffers we want to schedule in one second.
pub const BUFFERS_PER_SECOND: u8 = 25;

/// Macro that calculates the size of a buffer based on frequency
#[macro_export]
macro_rules! calc_buffer {
    ($x:expr, u8) => {
        ((((x)/BUFFERS_PER_SECOND) >> 3) << 3)
    };
    ($x:expr, u16) => {
        ((((x)/BUFFERS_PER_SECOND) >> 3) << 3)
    };
    ($x:expr, u32) => {
        ((((x)/BUFFERS_PER_SECOND) >> 3) << 3)
    };
    ($x:expr, u64) => {
        ((((x)/BUFFERS_PER_SECOND) >> 3) << 3)
    };
    ($x:expr, i8) => {
        ((((x)/BUFFERS_PER_SECOND) >> 3) << 3)
    };
    ($x:expr, i16) => {
        ((((x)/BUFFERS_PER_SECOND) >> 3) << 3)
    };
    ($x:expr, i32) => {
        ((((x)/BUFFERS_PER_SECOND) >> 3) << 3)
    };
    ($x:expr, i64) => {
        ((((x)/BUFFERS_PER_SECOND) >> 3) << 3)
    };
    ($x:expr, f32) => {
        compile_error!("Cannot calculate buffers with an f32")
    };
    ($x:expr, f64) => {
        compile_error!("Cannot calculate buffers with an f64")
    };
}

/// The audio context.
/// Since the actual audio.c uses global variables and resets them
/// upon init being called, we have this dummy struct with methods
/// in order to make it more clear how this is normally used.
struct Audio;

impl Audio {
    /// Initialize the audio subsystem.
    pub fn new(frequency: i32, numbuffers: i32) -> Self {
        unsafe {
            audio_init(frequency, numbuffers);
        };
        Self{}
    }
    /// Install an audio callback to fill the audio buffer when required.
    pub fn set_buffer_callback(&self, f: fn(buffer: &mut [i16], num_samples: usize)) {
        // new closure based on the function we want to actually call
        let closure: &'static _ = Box::leak(
            Box::new(move |buffer: *mut c_short, num_samples: c_size_t| {
                let buffer = unsafe {
                    std::slice::from_raw_parts_mut(buffer, num_samples)
                };
                f(buffer, num_samples);
            })
        );
        let callback = Closure2::new(closure);
        let &code = callback.code_ptr();
        let ptr: unsafe extern "C" fn (*mut c_short, c_size_t) = unsafe {
            std::mem::transmute(code)
        };
        std::mem::forget(callback);
        unsafe {
            audio_set_buffer_callback(Some(ptr));
        }
    }

    /// Pause or resume audio playback
    pub fn pause(&self, pause: bool) {
        unsafe {audio_pause(pause);}
    }

    /// Write a chunk of audio data to the first free internal buffer
    pub fn write(&self, buf: &[i16]) {
        unsafe {
            audio_write(buf.as_ptr())    
        }
    }

    /// Write silence
    pub fn write_silence(&self) {
        unsafe {
            audio_write_silence()
        }
    }

    /// Getter for the actual frequency of audio playback
    pub fn frequency(&self) -> i32 {
        unsafe {
            audio_get_frequency()
        }
    }

    /// Getter for the number of stereo samples that fit into an allocated buffer.
    pub fn buffer_length(&self) -> i32 {
        unsafe {
            audio_get_buffer_length()
        }
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        unsafe {
            audio_close();
        }
    }
}


