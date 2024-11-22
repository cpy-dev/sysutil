use std::path;
use std::fs;
use std::io::Read;

pub fn linuxCheck() {
    if !path::Path::new("/sys").exists() || !path::Path::new("/proc").exists() {
        panic!("Detected non-Linux system");
    }
}

pub fn readFile<T>(filePath: T) -> String
where T: AsRef<path::Path>, {
    if let Ok(mut file) = fs::File::open(filePath) {
        let mut buffer = String::new();

        if file.read_to_string(&mut buffer).is_ok() {
            return buffer.trim().to_string();
        }
    }

    return String::new();
}

pub fn bytesToU16(bytes: Vec<u8>) -> u16 {
    let first = bytes[1];
    let second = bytes[0];

    let mut tmp = first as u16;
    for _ in 0..8 {
        tmp *= 2;
    }

    tmp + (second as u16)
}

pub fn bytesToU32(bytes: Vec<u8>) -> u32 {
    let first = bytes[3];
    let second = bytes[2];
    let third = bytes[1];
    let fourth = bytes[0];

    ((first as u32) << 24) + ((second as u32) << 16) + ((third as u32) << 8) + (fourth as u32)
}

pub fn bytesToU64(bytes: Vec<u8>) -> u64 {
    let mut res: u64 = 0;
    let mut shift: u64 = 64;

    let mut reversed = bytes.clone();
    reversed.reverse();

    for byte in reversed {
        shift -= 8;
        res += (byte as u64) << shift;
    }

    return res;
}
