mod anfield;
mod process;

pub use anfield::*;
pub use process::*;

pub mod flag {
    pub static mut DEBUG: bool = false;
}

pub mod logger {
    use crate::flag;

    pub fn console_log<T: std::fmt::Debug>(value: T) {
        if unsafe { flag::DEBUG } {
            println!("{:?}", value);
        }
    }
}
