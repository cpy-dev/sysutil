use std::{fs, path};
use crate::utils::{*};

/// Encloses gpu metrics parameters
#[derive(Debug, Clone)]
pub struct GpuMetrics {
    pub temperatureEdge: u16,
    pub temperatureHotspot: u16,
    pub temperatureMem: u16,
    pub temperatureVrgfx: u16,
    pub temperatureVrsoc: u16,
    pub temperatureVrmem: u16,
    pub averageSocketPower: u16,
    pub averageGfxclkFrequency: u16,
    pub averageSockclkFrequency: u16,
    pub averageUclkFrequency: u16,
    pub currentGfxclk: u16,
    pub currentSockclk: u16,
    pub currentUclk: u16,
    pub currentVclk0: u16,
    pub currentDclk0: u16,
    pub currentVclk1: u16,
    pub currentDclk1: u16,
    pub throttleStatus: u32,
    pub currentFanSpeed: u16,
    pub pcieLinkWidth: u16,
    pub pcieLinkSpeed: u16,
}

/// Contains total gpu's vram size, both in GB (1000^3 bytes) and GiB (1024^3 bytes)
#[derive(Debug, Clone)]
pub struct VramSize {
    pub gb: f32,
    pub gib: f32
}

/// returns current GPU usage in percentage, returns `None` if it's not possible to retrieve data
pub fn gpuUsage() -> Option<f32> {
    linuxCheck();

    let fileContent = readFile("/sys/class/drm/card0/device/gpu_busy_percent");
    let gpuUsage = fileContent.parse::<f32>().ok()?;
    return Some(gpuUsage);
}

/// Returns metrics parameters from the amdgpu driver
pub fn gpuMetrics() -> Option<GpuMetrics> {
    linuxCheck();

    let filePipe = fs::read(path::Path::new("/sys/class/drm/card0/device/gpu_metrics"));

    let mut bytes: Vec<u8> = Vec::<u8>::new();
    let mut error = false;

    match filePipe {
        Err(_) => {
            error = true;
        },
        Ok(bytesPipe) => {
            bytes = bytesPipe;
        }
    };

    if error {
        return None;
    }

    let format = bytes[2];
    let content = bytes[3];

    bytes = bytes[4..].to_vec();

    if format != 1 {
        return None;
    }

    Some(
        GpuMetrics {
            temperatureEdge: match content {
                0 => bytesToU16(bytes[8..10].to_vec()),
                _ => bytesToU16(bytes[0..2].to_vec())
            },

            temperatureHotspot: match content {
                0 => bytesToU16(bytes[10..12].to_vec()),
                _ => bytesToU16(bytes[2..4].to_vec())
            },

            temperatureMem: match content {
                0 => bytesToU16(bytes[12..14].to_vec()),
                _ => bytesToU16(bytes[4..6].to_vec())
            },

            temperatureVrgfx: match content {
                0 => bytesToU16(bytes[14..16].to_vec()),
                _ => bytesToU16(bytes[6..8].to_vec())
            },

            temperatureVrsoc: match content {
                0 => bytesToU16(bytes[16..18].to_vec()),
                _ => bytesToU16(bytes[8..10].to_vec())
            },

            temperatureVrmem: match content {
                0 => bytesToU16(bytes[18..20].to_vec()),
                _ => bytesToU16(bytes[10..12].to_vec())
            },

            averageSocketPower: match content {
                0 => bytesToU16(bytes[26..28].to_vec()),
                _ => bytesToU16(bytes[18..20].to_vec())
            },

            averageGfxclkFrequency: bytesToU16(bytes[36..38].to_vec()),
            averageSockclkFrequency: bytesToU16(bytes[38..40].to_vec()),
            averageUclkFrequency: bytesToU16(bytes[40..42].to_vec()),

            currentGfxclk: bytesToU16(bytes[50..52].to_vec()),
            currentSockclk: bytesToU16(bytes[52..54].to_vec()),
            currentUclk: bytesToU16(bytes[54..56].to_vec()),
            currentVclk0: bytesToU16(bytes[56..58].to_vec()),
            currentDclk0: bytesToU16(bytes[58..60].to_vec()),
            currentVclk1: bytesToU16(bytes[60..62].to_vec()),
            currentDclk1: bytesToU16(bytes[62..64].to_vec()),

            throttleStatus: bytesToU32(bytes[64..68].to_vec()),
            currentFanSpeed: bytesToU16(bytes[68..70].to_vec()),
            pcieLinkWidth: bytesToU16(bytes[70..72].to_vec()),
            pcieLinkSpeed: bytesToU16(bytes[72..74].to_vec())
        }
    )
}

/// Returns gpu's vram size as specified in `VramSize` struct, returns `None` if it's not possible to retrieve data
pub fn vramSize() -> Option<VramSize> {
    linuxCheck();

    let fileContent = readFile("/sys/class/drm/card0/device/mem_info_vram_total");
    match fileContent.parse::<usize>() {
        Err(_) => {
            return None
        },
        Ok(uMem) => {
            return Some(VramSize {
                gb: uMem as f32 / 1000_f32 / 1000_f32 / 1000_f32,
                gib: uMem as f32 / 1024_f32 / 1024_f32 / 1024_f32
            })
        }
    };
}

/// Returns gpu's vram usage in percentage, returns `None` if it's not possible to retrieve data
pub fn vramUsage() -> Option<f32> {
    linuxCheck();

    let vramTotal = readFile("/sys/class/drm/card0/device/mem_info_vram_total");
    let vramUsed = readFile("/sys/class/drm/card0/device/mem_info_vram_used");

    if vramTotal.is_empty() || vramUsed.is_empty() {
        return None;
    }

    let uVramTotal: usize = vramTotal.parse::<usize>().unwrap();
    let uVramUsed: usize = vramUsed.parse::<usize>().unwrap();

    return Some(uVramUsed as f32 * 100_f32 / uVramTotal as f32);
}
