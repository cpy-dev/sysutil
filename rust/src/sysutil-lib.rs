//! Linux system information library
//!
//! <div class="warning">This library is ment to be used only in linux systems. It is possible to write code using it on other systems, but it will not allow to run the code, panicking before execution </div>
//!
//! # Installation
//!
//! Run the following command in your terminal
//!
//! ```bash
//! cargo add sysutil
//! ```
//!
//! # Importation
//! Add in your code:
//! ```
//! use sysutil;
//! ```
//!
//! ---
//! <div class="warning">GPU's related functions have been tested only on AMD Radeon 7000 series, any other GPU model is not "officially supported"</div>

#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::fs;
use std::io::Read;
use std::path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::i64;

/// Represents the current status of battery
#[derive(Debug)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
}

/// Contains capacity and current status of battery
#[derive(Debug)]
pub struct Battery {
    pub capacity: u8,
    pub status: BatteryStatus,
}

impl Battery {
    fn new(capacity: u8, status: BatteryStatus) -> Battery {
        return Battery { capacity, status };
    }
}

/// Contains the average CPU usage and the discrete usage for each processor
#[derive(Debug)]
pub struct CpuUsage {
    pub average: ProcessorUsage,
    pub processors: Vec<ProcessorUsage>,
}

/// Encloses the different parameters relative to processor usage
#[derive(Clone, Debug)]
pub struct ProcessorUsage {
    pub total: f32,
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub idle: f32,
    pub iowait: f32,
    pub interrupt: f32,
    pub soft_interrupt: f32,
}

impl ProcessorUsage {
    fn clone(&self) -> ProcessorUsage {
        return ProcessorUsage {
            total: self.total,
            user: self.user,
            nice: self.nice,
            system: self.system,
            idle: self.idle,
            iowait: self.iowait,
            interrupt: self.interrupt,
            soft_interrupt: self.soft_interrupt,
        };
    }
}

/// Contains total download and upload newtwork rate (in bytes)
#[derive(Debug)]
pub struct NetworkRate {
    pub download: f32,
    pub upload: f32,
}

/// Contains temperature sensor's name and recorded temperature
#[derive(Debug)]
pub struct TemperatureSensor {
    pub label: String,
    pub temperature: Option<f32>,
}

/// Contains base information relative to the CPU
#[derive(Debug)]
pub struct CpuInfo {
    pub modelName: String,
    pub cores: usize,
    pub threads: usize,
    pub dies: usize,
    pub governors: Vec<String>,
    pub maxFrequencyMHz: f32,
    pub clockBoost: Option<bool>,
    pub architecture: String,
    pub byteOrder: String
}

/// Encloses all CPU-related data available in the library
/// ## Example
/// Once generating a `CPU` instance, usages and scheduler policies can be updated by the `update()` method
/// ```rust
/// use sysutil;
///
/// let mut cpu = sysutil::CPU::new();
/// cpu.update();
/// ```
#[derive(Debug)]
pub struct CPU {
    pub info: CpuInfo,
    pub averageUsage: ProcessorUsage,
    pub perProcessorUsage: Vec<ProcessorUsage>,
    pub schedulerPolicies: Vec<SchedulerPolicy>,
    pub averageFrequency: Frequency,
    pub perProcessorFrequency: Vec<ProcessorFrequency>
}

impl CPU {
    pub fn new() -> CPU {
        let cpuUsage = cpuUsage();
        let frequency = cpuFrequency();

        CPU {
            info: cpuInfo(),
            averageUsage: cpuUsage.average,
            perProcessorUsage: cpuUsage.processors,
            schedulerPolicies: schedulerInfo(),
            averageFrequency: frequency.average,
            perProcessorFrequency: frequency.processors
        }
    }

    pub fn update(&mut self) {
        self.schedulerPolicies = schedulerInfo();
        let cpuUsage = cpuUsage();

        self.averageUsage = cpuUsage.average;
        self.perProcessorUsage = cpuUsage.processors;
    }
}

/// Frequency data structure implements direct conversion for frequencies
/// in various size orders
/// ```rust
/// use sysutil::cpuFrequency;
/// let frequency = cpuFrequency().average;
///
/// frequency.khz(); // returns the frequency in Kilo Hertz
/// frequency.mhz(); // returns the frequency in Mega Hertz
/// frequency.ghz(); // return the frequency in Giga Hertz
/// ```
#[derive(Debug)]
pub struct Frequency {
    khz: usize
}

impl Frequency {
    pub fn khz(&self) -> f32 {
        return self.khz as f32;
    }

    pub fn mhz(&self) -> f32 {
        return self.khz as f32 / 1000_f32;
    }

    pub fn ghz(&self) -> f32 {
        return self.khz as f32 / 1000_000_f32;
    }
}

/// Contains processor id and its frequency
#[derive(Debug)]
pub struct ProcessorFrequency {
    pub processorID: String,
    pub frequency: Frequency
}

/// Contains cpu frequencies, both average and processor wise
#[derive(Debug)]
pub struct CpuFrequency {
    pub average: Frequency,
    pub processors: Vec<ProcessorFrequency>
}

/// Contains total ram size, both in GB (1000^3 bytes) and GiB (1024^3 bytes)
#[derive(Debug)]
pub struct RamSize {
    pub gb: f32,
    pub gib: f32,
}

/// Contains scheduler information relative to a processor in the system
#[derive(Debug)]
pub struct SchedulerPolicy {
    pub name: String,
    pub scalingGovernor: String,
    pub scalingDriver: String,
    pub minimumScalingMHz: f32,
    pub maximumScalingMHz: f32,
}

/// Contains total gpu's vram size, both in GB (1000^3 bytes) and GiB (1024^3 bytes)
#[derive(Debug)]
pub struct VramSize {
    pub gb: f32,
    pub gib: f32
}

/// Different route types
#[derive(Debug, Clone)]
pub enum RouteType {
    TCP,
    TCP6,
    UDP,
    UDP6
}

/// Represents a network route and its type, containing local address+port and remote address+port
#[derive(Debug)]
pub struct NetworkRoute {
    pub routeType: RouteType,
    pub localAddress: String,
    pub localPort: u16,
    pub remoteAddress: String,
    pub remotePort: u16
}

/// Contains currently active clock source and the available ones
#[derive(Debug)]
pub struct ClockSource {
    pub current: String,
    pub available: Vec<String>
}

/// Contains information relative to the motherboard and the installed bios
#[derive(Debug)]
pub struct Motherboard {
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub bios: Bios
}

/// Contains information relative to the installed bios
#[derive(Debug)]
pub struct Bios {
    pub vendor: String,
    pub release: String,
    pub version: String,
    pub date: String
}

/// Encloses gpu metrics parameters
#[derive(Debug)]
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
    pub throttleStatus: u32,
    pub currentFanSpeed: u16,
    pub pcieLinkWidth: u16,
    pub pcieLinkSpeed: u16,
}

/// Contains NVME device information
#[derive(Debug)]
pub struct NvmeDevice {
    pub device: String,
    pub pcieAddress: String,
    pub model: String,
    pub linkSpeedGTs: f32,
    pub pcieLanes: usize,
    pub size: ByteSize,
    pub partitions: Vec<StoragePartition>
}

/// Bytes size data structure implementing methods to convert in various size orders
/// The methods allow the convertion in the various size orders, both in base 1000 and base 1024
/// ```rust
/// let byteSize = /* some sysutil function returning ByteSize */ ;
/// byteSize.b(); // bytes
///
/// byteSize.kb(); // 1000 bytes
/// byteSize.kib(); // 1024 bytes
///
/// byteSize.mb(); // 1.000.000 bytes
/// byteSize.mib(); // 1.048.576 bytes
///
/// byteSize.gb(); // 1.000.000.000 bytes
/// byteSize.gib(); //1.073.741.824 bytes
///
/// byteSize.tb(); // 1.000.000.000.000 bytes
/// byteSize.tib(); // 1.099.511.627.776 bytes
/// ```
#[derive(Debug)]
pub struct ByteSize {
    bytes: usize
}

impl ByteSize {
    fn new(value: usize) -> ByteSize {
        return ByteSize {
            bytes: value
        }
    }

    pub fn b(&self) -> usize {
        self.bytes
    }

    pub fn kb(&self) -> f32 {
        self.bytes as f32 / 1000_f32
    }

    pub fn mb(&self) -> f32 {
        self.bytes as f32 / 1000_f32 / 1000_f32
    }

    pub fn gb(&self) -> f32 {
        self.bytes as f32 / 1000_f32 / 1000_f32 / 1000_f32
    }

    pub fn tb(&self) -> f32 {
        self.bytes as f32 / 1000_f32 / 1000_f32 / 1000_f32 / 1000_f32
    }

    pub fn kib(&self) -> f32 {
        self.bytes as f32 / 1024_f32
    }

    pub fn mib(&self) -> f32 {
        self.bytes as f32 / 1024_f32 / 1024_f32
    }

    pub fn gib(&self) -> f32 {
        self.bytes as f32 / 1024_f32 / 1024_f32 / 1024_f32
    }

    pub fn tib(&self) -> f32 {
        self.bytes as f32 / 1024_f32 / 1024_f32 / 1024_f32 / 1024_f32
    }
}

/// Encloses device name, size and startpoint relative to a partition
#[derive(Debug)]
pub struct StoragePartition {
    pub device: String,
    pub mountPoint: String,
    pub fileSystem: String,
    pub size: ByteSize,
    pub startPoint: usize
}

/// Contains information relative to a storage device in the system
#[derive(Debug)]
pub struct StorageDevice {
    pub model: String,
    pub device: String,
    pub size: ByteSize,
    pub partitions: Vec<StoragePartition>
}

fn linuxCheck() {
    if !path::Path::new("/sys").exists() || !path::Path::new("/proc").exists() {
        panic!("Detected non-Linux system");
    }
}

fn readFile<T>(filePath: T) -> String
where T: AsRef<path::Path>, {
    if let Ok(mut file) = fs::File::open(filePath) {
        let mut buffer = String::new();

        if let Ok(_) = file.read_to_string(&mut buffer) {
            return buffer.trim().to_string();
        }
    }

    return String::new();
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

/// returns current GPU usage in percentage, returns `None` if it's not possible to retrieve data
pub fn gpuUsage() -> Option<f32> {
    linuxCheck();

    let fileContent = readFile("/sys/class/drm/card0/device/gpu_busy_percent");
    let gpuUsage = fileContent.parse::<f32>().ok()?;
    return Some(gpuUsage);
}

fn getStats() -> Vec<Vec<usize>> {
    linuxCheck();

    let fileContent = readFile("/proc/stat");

    let lines = fileContent.split("\n");
    let mut strLines = Vec::<String>::new();

    for currentLine in lines {
        if currentLine.find("cpu") != None {
            strLines.push(currentLine.to_string());
        }
    }

    let mut uLines = Vec::<Vec<usize>>::new();
    for line in strLines {
        let splittedLine = line.split(" ").into_iter();
        let mut fixedLine = Vec::<usize>::new();

        for chunk in splittedLine {
            if !chunk.is_empty() && chunk.find("cpu") == None {
                fixedLine.push(chunk.parse().unwrap());
            }
        }
        uLines.push(fixedLine);
    }

    return uLines;
}

/// Returns CPU usage, both average and processor-wise, each value is in percentage
pub fn cpuUsage() -> CpuUsage {
    linuxCheck();

    let before = getStats();
    thread::sleep(Duration::from_millis(250));
    let after = getStats();

    let mut processors = Vec::<ProcessorUsage>::new();
    for i in 0..before.len() {
        let beforeLine = &before[i];
        let beforeSum = {
            let mut sum = 0;

            for element in beforeLine {
                sum += element;
            }
            sum
        };

        let afterLine = &after[i];
        let afterSum = {
            let mut sum = 0;

            for element in afterLine {
                sum += element;
            }
            sum
        };

        let delta: f32 = (afterSum - beforeSum) as f32;

        processors.push(ProcessorUsage {
            total: {
                100_f32 - (afterLine[3] - beforeLine[3]) as f32 * 100_f32 / delta
            },
            user: {
                (afterLine[0] - beforeLine[0]) as f32 * 100_f32 / delta
            },
            nice: {
                (afterLine[1] - beforeLine[1]) as f32 * 100_f32 / delta
            },
            system: {
                (afterLine[2] - beforeLine[2]) as f32 * 100_f32 / delta
            },
            idle: {
                (afterLine[3] - beforeLine[3]) as f32 * 100_f32 / delta
            },
            iowait: {
                (afterLine[4] - beforeLine[4]) as f32 * 100_f32 / delta
            },
            interrupt: {
                (afterLine[5] - beforeLine[5]) as f32 * 100_f32 / delta
            },
            soft_interrupt: {
                (afterLine[6] - beforeLine[6]) as f32 * 100_f32 / delta
            },
        });
    }

    return CpuUsage {
        average: processors[0].clone(),
        processors: processors[1..].to_vec(),
    };
}

/// Returns current RAM usage in percentage
pub fn ramUsage() -> f32 {
    linuxCheck();

    let content = readFile("/proc/meminfo");

    let mut memTotal = "";
    let mut memAvailable = "";

    for element in content.split("\n") {
        if element.find("MemTotal") != None {
            memTotal = element;
        } else if element.find("MemAvailable") != None {
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

fn getRate() -> (usize, usize) {
    let stats = readFile("/proc/net/dev");

    let mut downloadRate = 0_usize;
    let mut uploadRate = 0_usize;

    for line in stats.split("\n") {
        if line.find(":") != None {
            let splitted = {
                let tmp = line.split(" ");

                let mut data = Vec::<usize>::new();
                for chunk in tmp {
                    if !chunk.is_empty() && chunk.find(":") == None {
                        data.push(chunk.parse().unwrap());
                    }
                }
                data
            };

            downloadRate += splitted[0];
            uploadRate += splitted[8];
        }
    }
    return (downloadRate, uploadRate);
}

/// Returns current network rate (downlaod and upload), expressed in bytes
pub fn networkRate() -> NetworkRate {
    linuxCheck();

    let (downBefore, upBefore) = getRate();
    thread::sleep(Duration::from_millis(500));
    let (downAfter, upAfter) = getRate();

    let downloadRate: f32 = ((downAfter - downBefore) as f32) / 0.5_f32;
    let uploadRate: f32 = ((upAfter - upBefore) as f32) / 0.5_f32;

    return NetworkRate {
        download: downloadRate,
        upload: uploadRate,
    };
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

/// Returns CPU base information, enclosed in the `CpuInfo` data structure
pub fn cpuInfo() -> CpuInfo {
    linuxCheck();

    let infoFile = readFile("/proc/cpuinfo");
    let modelName = {
        let mut name = String::new();
        for line in infoFile.split("\n") {
            if line.contains("model name") {
                name = line.split(":").last().unwrap().to_string();
                break;
            }
        }
        name.trim().to_string()
    };

    let baseDir = path::Path::new("/sys/devices/system/cpu");

    let mut coreCount: usize = 0;
    let mut dieCount: usize = 0;

    for processor in fs::read_dir(baseDir).unwrap() {
        let processorPath = processor.unwrap().path();
        let path = processorPath.to_str().unwrap();

        if path.find("cpu") != None && path.find("cpufreq") == None && path.find("cpuidle") == None
        {
            let coreId = readFile(format!("{path}/topology/core_id").as_str());
            let dieId = readFile(format!("{path}/topology/die_id").as_str());

            if !coreId.is_empty() {
                match coreId.parse::<usize>() {
                    Err(_) => (),
                    Ok(uCoreId) => {
                        if uCoreId > coreCount {
                            coreCount = uCoreId;
                        }
                    }
                }
            }

            if !dieId.is_empty() {
                match dieId.parse::<usize>() {
                    Err(_) => (),
                    Ok(uDieId) => {
                        if uDieId > dieCount {
                            dieCount = uDieId;
                        }
                    }
                }
            }
        }
    }

    if coreCount % 2 == 1 {
        coreCount += 1;
    }
    dieCount += 1;

    let cpuInfoFile = readFile("/proc/cpuinfo");
    let threadCount = cpuInfoFile.matches("processor").count();

    let mut governors = Vec::<String>::new();
    let policiesPath = path::Path::new("/sys/devices/system/cpu/cpufreq/");

    let mut maxFrequency: usize = 0;
    let mut clockBoost: Option<bool> = None;

    for dir in fs::read_dir(policiesPath).unwrap() {
        let path = dir.unwrap().path();
        let sPath = path.to_str().unwrap();

        if sPath.contains("policy") {
            let localGovernors = readFile(format!("{sPath}/scaling_available_governors").as_str());
            let maxFreq = readFile(format!("{sPath}/cpuinfo_max_freq").as_str());

            if !maxFreq.is_empty() {
                match maxFreq.parse::<usize>() {
                    Err(_) => (),
                    Ok(freq) => {
                        if freq > maxFrequency {
                            maxFrequency = freq;
                        }
                    }
                }
            }

            for governor in localGovernors.split(" ") {
                if !governors.contains(&governor.to_string()) {
                    governors.push(governor.to_string());
                }
            }
        } else if sPath.contains("boost") {
            let content = readFile(sPath);

            if content.trim() == "1" {
                clockBoost = Some(true);
            } else {
                clockBoost = Some(false);
            }
        }
    }

    let freqMHz = maxFrequency as f32 / 1000_f32;
    let maxInteger: usize = usize::MAX;

    let arch = {
        if maxInteger as u128 == 2_u128.pow(64) - 1 {
            String::from("64 bit")
        } else if maxInteger as u128 == 2_u128.pow(32) - 1 {
            String::from("32 bit")
        } else {
            String::new()
        }
    };

    let pipe = Command::new("sh").arg("-c").args(
        ["echo -n I | od -t o2 | head -n 1 | cut -f 2 -d \" \" | cut -c 6"]
    ).output().unwrap().stdout;

    let byteOrder;
    match String::from_utf8(pipe).unwrap().trim() {
        "1" => {
            byteOrder = String::from("Little Endian");
        },
        "0" => {
            byteOrder = String::from("Big Endian");
        },

        _ => {
            byteOrder = String::new();
        }
    };

    return CpuInfo {
        modelName: modelName,
        cores: coreCount,
        threads: threadCount,
        dies: dieCount,
        governors: governors,
        maxFrequencyMHz: freqMHz,
        clockBoost: clockBoost,
        architecture: arch,
        byteOrder: byteOrder,
    };
}

/// Returns RAM size using the `RamSize` data structure
pub fn ramSize() -> RamSize {
    linuxCheck();

    let content = readFile("/proc/meminfo");

    let mut memTotal = "";

    for element in content.split("\n") {
        if element.find("MemTotal") != None {
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

/// Returns scheduler information for each processor
pub fn schedulerInfo() -> Vec<SchedulerPolicy> {
    linuxCheck();

    let schedulerDir = path::Path::new("/sys/devices/system/cpu/cpufreq/");
    let mut policies = Vec::<SchedulerPolicy>::new();

    for dir in fs::read_dir(schedulerDir).unwrap() {
        let path = dir.unwrap().path();
        let sPath = path.to_str().unwrap();

        if sPath.contains("policy") {
            let policyName = sPath.split("/").last().unwrap().to_string();

            let scalingGovernor = readFile(format!("{sPath}/scaling_governor").as_str());
            let scalingDriver = readFile(format!("{sPath}/scaling_driver").as_str());

            let maxScalingFrequency = readFile(
                format!("{sPath}/scaling_max_freq").as_str()
            ).parse::<f32>().unwrap() / 1000_f32;

            let minScalingFrequency = readFile(
                format!("{sPath}/scaling_min_freq").as_str()
            ).parse::<f32>().unwrap() / 1000_f32;

            policies.push(SchedulerPolicy {
                name: policyName,
                scalingGovernor: scalingGovernor,
                scalingDriver: scalingDriver,
                minimumScalingMHz: minScalingFrequency,
                maximumScalingMHz: maxScalingFrequency,
            });
        }
    }

    return policies;
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

fn bytesToAddress(address: String, separator: &str) -> String {
    let mut chunks = Vec::<String>::new();

    let mut index: usize = 0;
    while index < address.len() {
        chunks.push(
            String::from(
                i64::from_str_radix(
                    &address[index..index+2],
                    16
                ).unwrap().to_string()
            )
        );
        index += 2;
    }

    chunks.reverse();
    return chunks.join(separator);
}

fn bytesToPort(port: String) -> u16 {
    let (LSB, MSB) = port.split_at(2);
    (
        (
            i64::from_str_radix(MSB, 16).unwrap() as u16
        ) << 8
    ) + (
        i64::from_str_radix(LSB, 16).unwrap() as u16
    )
}

fn getRoutes(file: String, separator: &str, routeType: RouteType) -> Vec<NetworkRoute> {
    let mut routes = Vec::<NetworkRoute>::new();

    for line in file.split("\n") {
        if !line.contains(":") {
            continue;
        }

        let splittedLine: Vec<&str> = line.trim().split(" ").collect();
        let local: Vec<&str> = splittedLine[1].split(":").collect();

        let localAddress = bytesToAddress(local[0].to_string(), separator);
        let localPort = bytesToPort(local[1].to_string());

        let remote: Vec<&str> = splittedLine[2].split(":").collect();
        let remoteAddress = bytesToAddress(remote[0].to_string(), separator);
        let remotePort = bytesToPort(remote[1].to_string());

        routes.push(
            NetworkRoute {
                routeType: routeType.clone(),
                localAddress: localAddress,
                localPort: localPort,
                remoteAddress: remoteAddress,
                remotePort: remotePort
            }
        );
    }

    return routes;
}

/// Returns a list of each internal network route
pub fn networkRoutes() -> Vec<NetworkRoute> {
    linuxCheck();

    let mut routes: Vec<NetworkRoute> = Vec::<NetworkRoute>::new();

    routes.append(
        &mut getRoutes(readFile("/proc/net/tcp"), ".", RouteType::TCP)
    );

    routes.append(
        &mut getRoutes(readFile("/proc/net/udp"), ".", RouteType::UDP)
    );

    routes.append(
        &mut getRoutes(readFile("/proc/net/tcp6"), ":", RouteType::TCP6)
    );

    routes.append(
        &mut getRoutes(readFile("/proc/net/udp6"), ":", RouteType::UDP6)
    );

    return routes;
}

/// Returns the currently active clock source and the different ones available, enclosed in `ClockSource` struct
pub fn clockSource() -> ClockSource {
    linuxCheck();

    let currentClockSource = readFile(
        "/sys/devices/system/clocksource/clocksource0/current_clocksource"
    );

    let availableClockSources = readFile(
        "/sys/devices/system/clocksource/clocksource0/available_clocksource"
    );

    let mut sources = Vec::<String>::new();
    for source in availableClockSources.split(" ") {
        sources.push(String::from(source));
    }

    ClockSource {
        current: currentClockSource,
        available: sources,
    }
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

fn bytesToU16(bytes: Vec<u8>) -> u16 {
    let first = bytes[1];
    let second = bytes[0];

    let mut tmp = first as u16;
    for _ in 0..8 {
        tmp = tmp * 2;
    }

    tmp + (second as u16)
}

fn bytesToU32(bytes: Vec<u8>) -> u32 {
    let first = bytes[3];
    let second = bytes[2];
    let third = bytes[1];
    let fourth = bytes[0];

    ((first as u32) << 24) + ((second as u32) << 16) + ((third as u32) << 8) + (fourth as u32)
}

fn bytesToU64(bytes: Vec<u8>) -> u64 {
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

/// Returns metrics parameters from the amdgpu driver
pub fn gpuMetrics() -> Option<GpuMetrics>   {
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

            throttleStatus: bytesToU32(bytes[64..68].to_vec()),
            currentFanSpeed: bytesToU16(bytes[68..70].to_vec()),
            pcieLinkWidth: bytesToU16(bytes[70..72].to_vec()),
            pcieLinkSpeed: bytesToU16(bytes[72..74].to_vec())
        }
    )
}

/// Returns a vector containing all NVME devices found in the system
pub fn nvmeDevices() -> Vec<NvmeDevice> {
    linuxCheck();

    let mut devices = Vec::<NvmeDevice>::new();
    let mut deviceNames = Vec::<String>::new();
    let mut error = false;

    match fs::read_dir(path::Path::new("/sys/class/nvme")) {
        Err(_) => {
            error = true
        },

        Ok(devs) => {
            for device in devs {
                deviceNames.push(device.unwrap().file_name().to_str().unwrap().to_string());
            }
        }
    }

    if error {
        return devices;
    }

    let partitions = readFile("/proc/partitions");
    let mountPoints = readFile("/proc/mounts");

    for device in deviceNames {
        let path = format!("/sys/class/nvme/{}", device.clone());

        let deviceAddress = readFile(format!("{}/address", path));
        let model = readFile(format!("{}/model", path));

        let linkSpeed = {
            let tmp = readFile(format!("{}/device/current_link_speed", path));
            tmp.split(" ").collect::<Vec<&str>>()[0].to_string().parse::<f32>().unwrap()
        };
        let pcieLanes: usize = readFile(format!("{}/device/current_link_width", path)).parse().unwrap();

        let mut size: usize = 0;
        for partitionLine in partitions.split("\n") {
            if partitionLine.contains(&device) {

                let splitted = partitionLine.split(" ");
                let collected = splitted.collect::<Vec<&str>>();

                let tempSize = collected[collected.len() - 2];
                size = tempSize.parse::<usize>().unwrap();
            }
        }

        let mut localPartitions = Vec::<StoragePartition>::new();
        for mount in mountPoints.split("\n") {
            if mount.contains(&device) {
                let splitted: Vec<&str> = mount.split(" ").collect();
                let device = splitted.get(0).unwrap().to_string();
                let deviceName: String = device.split("/").collect::<Vec<&str>>().get(2).unwrap().to_string();

                let mountPoint = splitted.get(1).unwrap().to_string();
                let fileSystem = splitted.get(2).unwrap().to_string();

                let mut partSize = ByteSize{bytes: 0};
                let mut startPoint = 0;

                for partition in partitions.split("\n") {
                    if partition.contains(&deviceName) {

                         partSize = ByteSize{
                             bytes: {
                                 let tmp = readFile(format!("/sys/class/block/{}/size", deviceName));
                                 tmp.parse::<usize>().unwrap()
                             }
                         };

                        startPoint = {
                            let tmp = readFile(format!("/sys/class/block/{}/start", deviceName));
                            tmp.parse::<usize>().unwrap()
                        };
                        break
                    }
                }

                localPartitions.push(StoragePartition{
                    device: device,
                    mountPoint: mountPoint,
                    fileSystem: fileSystem,
                    size: partSize,
                    startPoint: startPoint
                });
            }
        }

        devices.push(
            NvmeDevice {
                device: device,
                model: model,
                pcieAddress: deviceAddress,
                linkSpeedGTs: linkSpeed,
                pcieLanes: pcieLanes,
                size: ByteSize::new(size),
                partitions: localPartitions
            }
        );
    }

    return devices;
}

/// Returns a vector containing all storage devices (NVME excluded) in the system
pub fn storageDevices() -> Vec<StorageDevice> {
    linuxCheck();

    let baseDir = "/sys/class/block";

    let mut error = false;
    let mut dirContent = Vec::<String>::new();

    match fs::read_dir(path::Path::new(baseDir)) {
        Err(_) => {
            error = true;
        },
        Ok(content) => {
            for dir in content {
                dirContent.push(dir.unwrap().file_name().to_str().unwrap().to_string())
            }
        }
    }

    if error {
        return Vec::<StorageDevice>::new();
    }

    let mountPoints = readFile("/proc/mounts");

    let mut devices = Vec::<StorageDevice>::new();
    for dir in &dirContent {

        if !dir.contains("sd") || dir.len() != 3 {
            continue
        }

        let device = format!("/dev/{}", dir);
        let size = ByteSize{
            bytes: {
                let tmp = readFile(format!("{}/{}/size", baseDir, dir));
                match tmp.parse::<usize>() {
                    Err(_) => 0,
                    Ok(value) => value
                }
            }
        };

        let model = readFile(format!("{}/{}/device/model", baseDir, dir));
        let mut partitions = Vec::<StoragePartition>::new();

        for partitionDir in &dirContent {
            if ! partitionDir.contains(&dir.clone()) {
                continue
            }

            if partitionDir.len() <= 3 {
                continue
            }

            let partitionSize = ByteSize{
                bytes: {
                    let tmp = readFile(format!("{}/{}/size", baseDir, partitionDir));
                    match tmp.parse::<usize>() {
                        Err(_) => 0,
                        Ok(value) => value
                    }
                }
            };

            let startByte = {
                let tmp = readFile(format!("{}/{}/start", baseDir, partitionDir));
                match tmp.parse::<usize>() {
                    Err(_) => 0,
                    Ok(value) => value
                }
            };

            let mut mountPoint = String::new();
            let mut filesystem = String::new();

            for mount in mountPoints.split("\n") {
                if mount.contains(&format!("/dev/{} ", partitionDir).to_string()) {
                    let splittedLine: Vec<&str> = mount.split(" ").collect();

                    mountPoint = splittedLine.get(1).unwrap().to_string();
                    filesystem = splittedLine.get(2).unwrap().to_string();

                    break
                }
            }

            partitions.push(
                StoragePartition {
                    device: format!("/dev/{}", partitionDir).to_string(),
                    size: partitionSize,
                    mountPoint: mountPoint,
                    fileSystem: filesystem,
                    startPoint: startByte
                }
            );
        }

        devices.push(
            StorageDevice {
                model: model,
                device: device,
                size: size,
                partitions: partitions
            }
        );
    }

    return devices;
}

/// Returns cpu frequency, both average and processor wise
pub fn cpuFrequency() -> CpuFrequency {
    linuxCheck();
    let mut totalFreq: f32 = 0_f32;
    let mut frequencies: Vec<ProcessorFrequency> = Vec::new();

    let fileContent = readFile("/proc/cpuinfo");
    for chunk in fileContent.split("\n\n") {

        if chunk.is_empty() {
            continue
        }

        let mut id = String::new();
        let mut freq: f32 = 0_f32;

        for line in chunk.split("\n") {
            if line.contains("processor") {
                id = line.trim().split(":").last().unwrap().trim().to_string();

            } else if line.contains("cpu MHz") {
                freq = line.trim().split(":").last().unwrap().trim().parse::<f32>().unwrap();
            }
        }

        if id.is_empty() || freq == 0_f32 {
            continue
        }

        totalFreq += freq;
        frequencies.push(ProcessorFrequency{
            processorID: id,
            frequency: Frequency {
                khz: (freq * 1000.0) as usize
            }
        });
    }

    CpuFrequency {
        average: Frequency {
            khz: (totalFreq * 1000.0) as usize / frequencies.len()
        },
        processors: frequencies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", cpuUsage());
        println!("RAM usage: {:?}", ramUsage());

        println!("{:?}", networkRate());
        println!("GPU usage: {:?}", gpuUsage());

        println!("{:?}", temperatureSensors());
        println!("{:?}", cpuInfo());

        println!("{:?}", ramSize());
        println!("{:?}", schedulerInfo());

        println!("{:?}", batteryInfo());
        println!("{:?}", vramSize());

        println!("VRAM usage: {:?}", vramUsage());
        println!("{:?}", networkRoutes());

        let cpu = CPU::new();
        println!("{:?}", cpu);

        println!("{:?}", cpuFrequency());

        println!("{:?}", clockSource());
        println!("{:?}", motherboardInfo());

        println!("{:?}", gpuMetrics());
        println!("{:?}", nvmeDevices());
        println!("{:?}", storageDevices());

        assert_eq!(0_u8, 0_u8);
    }
}
