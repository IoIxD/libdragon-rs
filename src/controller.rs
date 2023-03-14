use std::{fmt::Display};

use crate::ffi::bindings::{controller_init, controller_read, controller_data, controller_read_gc, controller_origin_data, controller_read_gc_origin, controller_scan, get_keys_down, SI_condat, SI_condat_gc, SI_condat_gc__bindgen_ty_1__bindgen_ty_2, get_keys_up, get_keys_held, get_keys_pressed, get_dpad_direction, execute_raw_command, get_controllers_present, get_accessories_present, read_mempak_address, write_mempak_address, identify_accessory, rumble_start, rumble_stop};

pub enum ControllerError {
    NoError,
    BadCommand,
    NotPresent
}

impl Into<ControllerError> for u32 {
    fn into(self) -> ControllerError {
        match self {
            1 => ControllerError::BadCommand,
            2 => ControllerError::NotPresent,
            _ => ControllerError::NoError,
        }
    }
}

impl Display for ControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ControllerError::NoError => {
                write!(f,"No error.")
            },
            ControllerError::BadCommand => {
                write!(f,"Command not recognized or malformed.")
            },
            ControllerError::NotPresent => {
                write!(f,"Controller not present.")
            },
        }
    }
}

pub enum MempakError {
    NoError,
    OutOfRange,
    NoMempakPresent,
    InvalidData,
}

impl Display for MempakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MempakError::NoError => {
                write!(f,"No error.")
            },
            MempakError::OutOfRange => {
                write!(f,"The controller is out of range.")
            },
            MempakError::NoMempakPresent => {
                write!(f,"There is no mempak present in the controller.")
            },
            MempakError::InvalidData => {
                write!(f,"The mempak returned invalid data.")
            },
        }
    }
}

pub enum ControllerStatus {
    Controller1Inserted = 0xF000,
    Controller2Inserted = 0x0F00,
    Controller3Inserted = 0x00F0,
    Controller4Inserted = 0x000F,
}

pub enum Accessory {
    None = 0,
    MemPak = 1,
    RumblePak = 2,
    VRU = 3,
    TransferPak = 4,
    Unknown,
}

type ControllerData = SI_condat;
struct GamecubeControllerData {
    pub inner: SI_condat_gc
}

impl GamecubeControllerData {
    fn deref(&self) -> SI_condat_gc__bindgen_ty_1__bindgen_ty_2 {
        unsafe {
            self.inner.__bindgen_anon_1.__bindgen_anon_2
        }
    }
}


impl From<SI_condat_gc> for GamecubeControllerData {
    fn from(value: SI_condat_gc) -> Self {
        GamecubeControllerData {
            inner: value,
        }
    }
}

struct ControllerState {
    n64_state: (ControllerData, ControllerData, ControllerData, ControllerData),
    gc_state: (GamecubeControllerData, GamecubeControllerData, GamecubeControllerData, GamecubeControllerData),
}

impl From<controller_data> for ControllerState {
    fn from(value: controller_data) -> Self {
        Self {
            n64_state: (
                value.c[0].into(),
                value.c[1].into(),
                value.c[2].into(),
                value.c[3].into(),
            ),
            gc_state: (
                value.gc[0].into(),
                value.gc[1].into(),
                value.gc[2].into(),
                value.gc[3].into(),
            ),
        }
    }
}

type ControllerOriginData = controller_origin_data;

/// The controller subsystem.
/// Since the actual controller.c uses global variables and resets them
/// upon init being called, we have this dummy struct with methods
/// in order to make it more clear how this is normally used.
struct Controller {}

impl Controller {
    pub fn new() -> Self {
        unsafe {
            controller_init();
        };
        Self{}
    }
    #[deprecated(
        note="This function is slow: it blocks for about 10% of a frame time. To avoid this hit, use the managed functions (Controller::get_keys_down(),etc).")]
    pub fn read(&self) -> Result<[ControllerData; 4], ControllerError> {
        let output: *mut controller_data = std::ptr::null_mut();
        unsafe {
            controller_read(output);
            let c = output.as_ref().unwrap();
            for e in c.c {
                let err: ControllerError = e.err().into();
                match err {
                    ControllerError::NoError => {},
                    ControllerError::BadCommand => {
                        return Err(err);
                    },
                    ControllerError::NotPresent => {
                        return Err(err);
                    },
                };
            }
            Ok(c.c.into())
        }
    }

    pub fn read_gc(&self, rumble: (bool, bool, bool, bool)) -> Result<[ControllerData; 4], ControllerError> {
        let output: *mut controller_data = std::ptr::null_mut();
        unsafe {
            controller_read_gc(output, [
                rumble.0.into(),
                rumble.1.into(),
                rumble.2.into(),
                rumble.3.into(),
            ].as_ptr());
            let c = output.as_ref().unwrap();
            for e in c.gc {
                let err: ControllerError = e.__bindgen_anon_1.__bindgen_anon_2.err().into();
                match err {
                    ControllerError::NoError => {},
                    ControllerError::BadCommand => {
                        return Err(err);
                    },
                    ControllerError::NotPresent => {
                        return Err(err);
                    },
                };
            }
            Ok(c.c.into())
        }
    }

    pub fn read_gc_origin(&self) -> &ControllerOriginData {
        let output: *mut controller_origin_data = std::ptr::null_mut();
        unsafe {
            controller_read_gc_origin(output);
            output.as_ref().unwrap() // error cannot be caught
        }
    }

    pub fn scan(&self) {
        unsafe {
            controller_scan();
        }
    }

    pub fn keys_down(&self) -> ControllerState {
        unsafe {
            get_keys_down().into()
        }
    }
    pub fn keys_up(&self) -> ControllerState {
        unsafe {
            get_keys_up().into()
        }
    }
    pub fn keys_held(&self) -> ControllerState {
        unsafe {
            get_keys_held().into()
        }
    }
    pub fn keys_pressed(&self) -> ControllerState {
        unsafe {
            get_keys_pressed().into()
        }
    }
    pub fn dpad_direction(&self, controller: i32) -> i32 {
        unsafe {
            get_dpad_direction(controller)
        }
    }
    pub unsafe fn execute_raw_command(&self, controller: i32, command: i32, bytesout: i32, bytesin: i32, out: &mut [u8]) {
        let output: *mut u8 = std::ptr::null_mut();
        execute_raw_command(controller, command, bytesout, bytesin, out.as_mut_ptr(), output);
    }
    pub fn controllers_present(&self) -> (bool, bool, bool, bool) {
        unsafe {
            let present = get_controllers_present();
            (
                (present & 0xF000) == 1,
                (present & 0x0F00) == 1,
                (present & 0x00F0) == 1,
                (present & 0x000F) == 1
            )
        }
    }
    pub fn accessories_present(&self) -> (bool, bool, bool, bool) {
        unsafe {
            let output: *mut controller_data = std::ptr::null_mut();
            let present = get_accessories_present(output);
            (
                (present & 0xF000) == 1,
                (present & 0x0F00) == 1,
                (present & 0x00F0) == 1,
                (present & 0x000F) == 1
            )
        }
    }
    pub fn read_mempak_address(&self, controller: i32, address: u16) -> Result<[u8; 32], MempakError> {
        unsafe {
            let data: &mut [u8; 32] = &mut [0; 32];
            let err = read_mempak_address(controller, address, data.as_mut_ptr());
            match err {
                0 => {
                    Ok(*data)
                },
                1 => {
                    Err(MempakError::OutOfRange)
                },
                2 => {
                    Err(MempakError::NoMempakPresent)
                },
                _ => {
                    Err(MempakError::InvalidData)
                }
            }
        }
    }

    pub fn write_mempak_address(&self, controller: i32, address: u16, data: &mut [u8; 32]) -> Result<(), MempakError> {
        unsafe {
            let err = write_mempak_address(controller, address, data.as_mut_ptr());
            match err {
                0 => {
                    Ok(())
                },
                1 => {
                    Err(MempakError::OutOfRange)
                },
                2 => {
                    Err(MempakError::NoMempakPresent)
                },
                _ => {
                    Err(MempakError::InvalidData)
                }
            }
        }
    }

    pub fn identify_accessory(&self, controller: i32) -> Option<Accessory> {
        unsafe {
            let acc = identify_accessory(controller);
            match acc {
                0 => None,
                1 => Some(Accessory::MemPak),
                2 => Some(Accessory::RumblePak),
                3 => Some(Accessory::VRU),
                4 => Some(Accessory::TransferPak),
                _ => Some(Accessory::Unknown)
            }
        }
    }

    pub fn rumble_start(&self, controller: i32) {
        unsafe {
            rumble_start(controller)
        }
    }
    pub fn rumble_stop(&self, controller: i32) {
        unsafe {
            rumble_stop(controller)
        }
    }

}
