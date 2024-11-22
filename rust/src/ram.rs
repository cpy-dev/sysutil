use crate::utils::{*};

/// Contains total ram size, both in GB (1000^3 bytes) and GiB (1024^3 bytes)
#[derive(Debug, Clone)]
pub struct RamSize {
    pub gb: f32,
    pub gib: f32,
}

/// Returns current RAM usage in percentage
pub fn ramUsage() -> f32 {
    linuxCheck();

    let content = readFile("/proc/meminfo");

    let mut memTotal = "";
    let mut memAvailable = "";

    for element in content.split('\n') {
        if element.contains("MemTotal") {
            memTotal = element;
        } else if element.contains("MemAvailable") {
            memAvailable = element;
        }
    }

    let uMemTotal = {
        let mut total = 0_usize;
        for element in memTotal.split(" ") {
            if element != "MemTotal:" && !element.is_empty() {
                total = element.parse::<usize>().unwrap();
                break;
            }
        }
        total
    };

    let uMemAvailable = {
        let mut available = 0_usize;
        for element in memAvailable.split(" ") {
            if element != "MemAvailable:" && !element.is_empty() {
                available = element.parse::<usize>().unwrap();
                break;
            }
        }
        available
    };

    return 100_f32 - uMemAvailable as f32 * 100_f32 / uMemTotal as f32;
}

/// Returns RAM size using the `RamSize` data structure
pub fn ramSize() -> RamSize {
    linuxCheck();

    let content = readFile("/proc/meminfo");
    let mut memTotal = "";

    for element in content.split('\n') {
        if element.contains("MemTotal") {
            memTotal = element;
        }
    }

    let uMemTotal = {
        let mut total = 0_usize;
        for element in memTotal.split(" ") {
            if element != "MemTotal:" && !element.is_empty() {
                total = element.parse::<usize>().unwrap();
                break;
            }
        }
        total
    };
    let GiB = uMemTotal as f32 * 1000_f32 / 1024_f32 / 1024_f32 / 1024_f32;
    let GB = uMemTotal as f32 / 1000_f32 / 1000_f32;

    return RamSize { gb: GB, gib: GiB };
}
