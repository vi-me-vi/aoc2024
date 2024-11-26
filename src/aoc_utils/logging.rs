use alloc::string::{String, ToString};
use alloc::format;
use alloc::vec::Vec;
use psp::sys;
use core::ffi::c_void;


pub struct AoCLogger {
    logfile: Vec<u8>,  // NOTE: Can't use String due to rust-psp limitation on sceIo
}

impl AoCLogger {
    pub fn new(logfile: String) -> Self {
        unsafe {
            psp::sys::sceIoRemove(
                b"./general.log\0" as *const u8,
            );
        }

        // HACK TO ADD \0
        let mut logfile_address = logfile.as_bytes().to_vec();
        logfile_address.push(0);
        // END HACK

        Self {
            logfile: logfile_address
        }
    }

    pub fn log(&self, message: &str) {
        let log_message = format!("{}\n", message).to_string();

        unsafe {
            let fd = sys::sceIoOpen(
                self.logfile.as_ptr(),
                sys::IoOpenFlags::CREAT | sys::IoOpenFlags::WR_ONLY,
                0o777,
            );

            sys::sceIoLseek(fd, 0, sys::IoWhence::End);
            sys::sceIoWrite(
                fd,
                log_message.as_ptr() as *const c_void,
                log_message.len()
            );

            sys::sceIoClose(fd);
        }
    }
}

impl Default for AoCLogger {
    fn default() -> Self {
        unsafe {
            psp::sys::sceIoRemove(
                b"./general.log\0" as *const u8,
            );
        }

        let logfile = String::from("./general.log");
        // HACK TO ADD \0
        let mut logfile_address = logfile.as_bytes().to_vec();
        logfile_address.push(0);
        // END HACK

        Self {
            logfile: logfile_address,
        }
    }
}