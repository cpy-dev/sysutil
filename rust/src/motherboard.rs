use crate::utils::{*};

/// Contains information relative to the motherboard and the installed bios
#[derive(Debug, Clone)]
pub struct Motherboard {
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub bios: Bios
}

/// Contains information relative to the installed bios
#[derive(Debug, Clone)]
pub struct Bios {
    pub vendor: String,
    pub release: String,
    pub version: String,
    pub date: String
}

/// Returns information about the currently installed BIOS
pub fn biosInfo() -> Bios {
    linuxCheck();

    let vendor = String::from(readFile("/sys/devices/virtual/dmi/id/bios_vendor").trim());
    let release = String::from(readFile("/sys/devices/virtual/dmi/id/bios_release").trim());

    let version = String::from(readFile("/sys/devices/virtual/dmi/id/bios_version").trim());
    let date = String::from(readFile("/sys/devices/virtual/dmi/id/bios_date").trim());

    Bios {
        vendor: vendor,
        release: release,
        version: version,
        date: date
    }
}

/// Returns information about the motherboard
pub fn motherboardInfo() -> Motherboard {
    linuxCheck();

    let name = String::from(readFile("/sys/devices/virtual/dmi/id/board_name").trim());
    let vendor = String::from(readFile("/sys/devices/virtual/dmi/id/board_vendor").trim());

    let version = String::from(readFile("/sys/devices/virtual/dmi/id/board_version").trim());
    let bios = biosInfo();

    Motherboard {
        name: name,
        version: version,
        vendor: vendor,
        bios: bios
    }
}
