use alloc::{string::String, format, vec};
use core::ffi::c_void;
use psp::sys;

use super::logging;


pub fn into_str(infile: &[u8]) -> String {  // TODO: Use Resul
    let logger = logging::AoCLogger::new(String::from("./reader.log"));
    logger.log(&format!("Trying to read {:?}, {}", infile, infile.len()));
    unsafe {
        let fd = sys::sceIoOpen(
            infile.as_ptr(),
            // sys::IoOpenFlags::CREAT | sys::IoOpenFlags::RD_ONLY,
             sys::IoOpenFlags::RD_ONLY,
            0o777,
        );

        if fd.0 < 0 {
            logger.log(&format!("[reader] error opening {:?}", infile));
            panic!("Failed to open file: {:?}", infile); // Panic on failure
        }

        let mut fd_stat = psp::sys::SceIoStat {
            st_mode: sys::IoStatMode::empty(),
            st_attr: sys::IoStatAttr::empty(),
            st_size: 0,
            st_ctime: sys::ScePspDateTime::default(),
            st_atime: sys::ScePspDateTime::default(),
            st_mtime: sys::ScePspDateTime::default(),
            st_private: [0, 0, 0, 0, 0, 0],
        };

        if sys::sceIoGetstat(infile.as_ptr(), &mut fd_stat) < 0 {
            logger.log(&format!("[reader] error getting file stats for {:?}", infile));
            panic!("Failed to get file stats: {:?}", infile); // Panic on failure
        }

        let mut buffer = vec![0u8; fd_stat.st_size as usize];

        if sys::sceIoRead(fd, buffer.as_mut_ptr() as *mut c_void, fd_stat.st_size as u32) < 0 {
            logger.log(&format!("[reader] error reading from {:?}", infile));
            panic!("Failed to read file: {:?}", infile); // Panic on failure
        }

        sys::sceIoClose(fd);

        String::from_utf8_unchecked(buffer)
    }
}
