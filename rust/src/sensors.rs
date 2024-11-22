use std::{fs, path, thread};
use crate::utils::{*};
use crate::utils::linuxCheck;

/// Represents the current status of battery
#[derive(Debug, Copy, Clone)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
}

/// Contains capacity and current status of battery
#[derive(Debug, Copy, Clone)]
pub struct Battery {
    pub capacity: u8,
    pub status: BatteryStatus,
}

impl Battery {
    fn new(capacity: u8, status: BatteryStatus) -> Battery {
        return Battery { capacity, status };
    }
}

/// Contains temperature sensor's name and recorded temperature
#[derive(Debug, Clone)]
pub struct TemperatureSensor {
    pub label: String,
    pub temperature: Option<f32>,
}

/// Holds information about backlight
#[derive(Debug, Clone)]
pub struct Backlight {
    pub brightness: u32,
    pub maxBrightness: u32
}

/// Returns every temperature sensor in the system, using the `TemperatureSensor` struct
pub fn temperatureSensors() -> Vec<TemperatureSensor> {
    linuxCheck();

    let hwmonPath = path::Path::new("/sys/class/hwmon");
    let dirs = fs::read_dir(hwmonPath).unwrap();

    let mut sensors = Vec::<TemperatureSensor>::new();

    for dir in dirs {
        let dirPath = dir.unwrap().path();

        let labelFile = dirPath.join("name");
        let label = readFile(labelFile.to_str().unwrap());

        let temperatureFile = dirPath.join("temp1_input");
        let temperature = readFile(temperatureFile.to_str().unwrap());

        sensors.push(TemperatureSensor {
            label: label,
            temperature: match temperature.parse::<f32>() {
                Err(_) => None,
                Ok(value) => Some(value / 1000_f32),
            },
        });
    }
    return sensors;
}

fn battery_path() -> Option<path::PathBuf> {
    fs::read_dir("/sys/class/power_supply")
        .ok()?
        .map(|entry| {
            let path = entry.ok()?.path();
            let handle = thread::spawn(move || {
                let file_content = fs::read_to_string(path.join("type")).ok()?;
                if file_content.trim() == "Battery"
                    && path.join("status").exists()
                    && path.join("capacity").exists()
                {
                    Some(path)
                } else {
                    None
                }
            });
            Some(handle)
        })
        .find_map(|handle| handle?.join().ok()?)
}

/// Returns battery current status and capacity as specified in `Battery` struct, returns `None` if it's not possible to retrieve data
pub fn batteryInfo() -> Option<Battery> {
    linuxCheck();

    let battery_path = battery_path()?;
    let capacity = readFile(battery_path.join("capacity"));
    let status = readFile(battery_path.join("status"));

    if capacity.is_empty() || status.is_empty() {
        return None;
    }

    let status = match status.as_str() {
        "Charging" => BatteryStatus::Charging,
        "Discharging" => BatteryStatus::Discharging,
        "Full" => BatteryStatus::Full,
        _ => return None,
    };

    Some(Battery::new(capacity.parse::<u8>().unwrap_or(0), status))
}

/// Returns the current backlight brightness and the maximum possible value or `None` if it's not possible to retrieve data
pub fn getBacklight() -> Option<Backlight> {
    let mut dirs = fs::read_dir("/sys/class/backlight").ok()?;
    let path = dirs.find(|entry| {
        let entry = entry.as_ref().unwrap().path();
        if entry.join("brightness").exists() && entry.join("max_brightness").exists() {
            return true;
        }

        false
    })?.ok()?;

    let brightness = fs::read_to_string(path.path().join("brightness")).ok()?.trim().parse::<u32>().ok()?;
    let maxBrightness = fs::read_to_string(path.path().join("max_brightness")).ok()?.trim().parse::<u32>().ok()?;

    Some(
        Backlight {
            brightness,
            maxBrightness,
        }
    )
}