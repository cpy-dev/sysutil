use std::path;
use std::fs;
use std::io::Read;

/// Byte measure unit
pub enum ByteUnit {
    B, KB, MB, GB,
    KiB, MiB, GiB
}

impl ByteUnit {
    pub fn toString(&self) -> String {
        match self {
            ByteUnit::B => "B".to_string(),
            ByteUnit::KB => "KB".to_string(),
            ByteUnit::MB => "MB".to_string(),
            ByteUnit::GB => "GB".to_string(),
            ByteUnit::KiB => "KiB".to_string(),
            ByteUnit::MiB => "MiB".to_string(),
            ByteUnit::GiB => "GiB".to_string(),
        }
    }
}

/// Data structure implementing conversion for the various measure units
#[derive(Debug)]
pub struct ByteSize {
    bytes: usize
}

impl ByteSize {
    pub fn fromBytes(bytes: usize) -> Self {
        Self {
            bytes: bytes
        }
    }

    pub fn b(&self) -> usize {
        self.bytes * 8
    }

    pub fn B(&self) -> usize {
        self.bytes
    }

    pub fn KB(&self) -> f32 {
        (self.bytes as f32) / 1000_f32
    }

    pub fn KiB(&self) -> f32 {
        (self.bytes as f32) / 1024_f32
    }

    pub fn MB(&self) -> f32 {
        self.KB() / 1000_f32
    }

    pub fn MiB(&self) -> f32 {
        self.KiB() / 1024_f32
    }

    pub fn GB(&self) -> f32 {
        self.MB() / 1000_f32
    }

    pub fn GiB(&self) -> f32 {
        self.MiB() / 1024_f32
    }

    pub fn fitBase1024(&self) -> (f32, ByteUnit) {
        if self.B() < 1024 {
            (self.B() as f32, ByteUnit::B)

        } else if self.KiB() < 1024_f32 {
            (self.KiB(), ByteUnit::KiB)

        } else if self.MiB() < 1024_f32 {
            (self.MiB(), ByteUnit::MiB)

        } else {
            (self.GiB(), ByteUnit::GiB)
        }
    }

    pub fn fitBase1000(&self) -> (f32, ByteUnit) {
        if self.B() < 1000 {
            (self.B() as f32, ByteUnit::B)

        } else if self.KB() < 1000_f32 {
            (self.KB(), ByteUnit::KB)

        } else if self.MB() < 1000_f32 {
            (self.MB(), ByteUnit::MB)

        } else {
            (self.GB(), ByteUnit::GB)
        }
    }
}

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
