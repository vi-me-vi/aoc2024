use alloc::string::{String, ToString};
use alloc::vec;
use core::char;
use core::ffi::c_void;
use psp::sys;


pub fn into_str(infile: &str) -> String {  // TODO: Use Result
    unsafe {
        let fd = sys::sceIoOpen(
            infile.as_ptr(),
            sys::IoOpenFlags::CREAT | sys::IoOpenFlags::RD_ONLY,
            0o777,
        );

        if fd.0 < 0 {
            panic!("Failed to open file: {}", infile); // Panic on failure
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
            panic!("Failed to get file stats: {}", infile); // Panic on failure
        }

        let mut buffer = vec![0u8; fd_stat.st_size as usize];

        if sys::sceIoRead(fd, buffer.as_mut_ptr() as *mut c_void, fd_stat.st_size as u32) < 0 {
            panic!("Failed to read file: {}", infile); // Panic on failure
        }

        sys::sceIoClose(fd);

        String::from_utf8(buffer).expect("File content is not valid UTF-8") // Panic if UTF-8 is invalid
    }
}

pub fn into_lines_vec(infile: &str) -> vec::Vec<String> {  // TODO: Use Result
    let content = into_str(infile); // If `into_str` panics, this will bubble up
    content.lines().map(|line| line.to_string()).collect()
}

pub fn into_char_metrix(infile: &str) -> vec::Vec<vec::Vec<char>> {  // TODO: Use Result
    let mut res_m: vec::Vec<vec::Vec<char>> = vec::Vec::new();
    let lines = into_lines_vec(infile);
    for line in lines {
        res_m.push(line.chars().collect());
    }
    res_m
}
