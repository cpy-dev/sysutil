#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs;
use std::io::Read;
use std::path;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::i64;

fn readFile<T>(filePath: T) -> String
where T: AsRef<Path>, {
    match fs::File::open(filePath) {
        Err(_) => {
            return String::new();
        }
        Ok(mut file) => {
            let mut buffer = String::new();

            match file.read_to_string(&mut buffer) {
                Err(_) => {
                    return String::new();
                }
                Ok(_) => {
                    return buffer.trim().to_string();
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
}

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

pub fn batteryInfo() -> Option<Battery> {
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

pub fn gpuUsage() -> Option<f32> {
    let fileContent = readFile("/sys/class/drm/card0/device/gpu_busy_percent");
    let gpuUsage = fileContent.parse::<f32>().ok()?;
    return Some(gpuUsage);
}

#[derive(Debug)]
pub struct CpuUsage {
    pub average: ProcessorUsage,
    pub processors: Vec<ProcessorUsage>,
}

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

fn getStats() -> Vec<Vec<usize>> {
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

pub fn cpuUsage() -> CpuUsage {
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

pub fn cpuFrequency() -> Option<f32> {
    let fileContent = readFile("/proc/cpuinfo");
    let mut frequencies: f32 = 0.0;
    let mut count = 0;

    for line in fileContent.split("\n") {
        if line.find("cpu MHz") != None {
            frequencies += line.split(" ").last().unwrap().parse::<f32>().unwrap();
            count += 1;
        }
    }

    if frequencies != 0_f32 {
        return Some(frequencies / (count as f32));
    }

    return None;
}

pub fn ramUsage() -> f32 {
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

#[derive(Debug)]
pub struct NetworkRate {
    pub download: f32,
    pub upload: f32,
}

pub fn networkRate() -> NetworkRate {
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

#[derive(Debug)]
pub struct TemperatureSensor {
    pub label: String,
    pub temperature: Option<f32>,
}

pub fn temperatureSensors() -> Vec<TemperatureSensor> {
    let hwmonPath = Path::new("/sys/class/hwmon");
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
            temperature: Some(temperature.parse::<f32>().unwrap() / 1000_f32),
        });
    }
    return sensors;
}

#[derive(Debug)]
pub struct Cpu {
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

pub fn cpuInfo() -> Cpu {
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

    let baseDir = Path::new("/sys/devices/system/cpu");

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

    coreCount += 1;
    dieCount += 1;

    let cpuInfoFile = readFile("/proc/cpuinfo");
    let threadCount = cpuInfoFile.matches("processor").count();

    let mut governors = Vec::<String>::new();
    let policiesPath = Path::new("/sys/devices/system/cpu/cpufreq/");

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

    return Cpu {
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

#[derive(Debug)]
pub struct RamSize {
    pub gb: f32,
    pub gib: f32,
}

pub fn ramSize() -> RamSize {
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

#[derive(Debug)]
pub struct SchedulerPolicy {
    pub name: String,
    pub scalingGovernor: String,
    pub scalingDriver: String,
    pub minimumScalingMHz: f32,
    pub maximumScalingMHz: f32,
}

pub fn schedulerInfo() -> Vec<SchedulerPolicy> {
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

#[derive(Debug)]
pub struct VramSize {
    pub gb: f32,
    pub gib: f32
}

pub fn vramSize() -> Option<VramSize> {
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

pub fn vramUsage() -> Option<f32> {
    let vramTotal = readFile("/sys/class/drm/card0/device/mem_info_vram_total");
    let vramUsed = readFile("/sys/class/drm/card0/device/mem_info_vram_used");

    if vramTotal.is_empty() || vramUsed.is_empty() {
        return None;
    }

    let uVramTotal: usize = vramTotal.parse::<usize>().unwrap();
    let uVramUsed: usize = vramUsed.parse::<usize>().unwrap();

    return Some(uVramUsed as f32 * 100_f32 / uVramTotal as f32);
}

#[derive(Debug, Clone)]
pub enum RouteType {
    TCP,
    TCP6,
    UDP,
    UDP6
}

#[derive(Debug)]
pub struct NetworkRoute {
    pub routeType: RouteType,
    pub localAddress: String,
    pub localPort: u16,
    pub remoteAddress: String,
    pub remotePort: u16
}

fn bytesToAddress(address: String, separator: &str) -> String {
    let mut chunks = Vec::<String>::new();

    let mut index: usize = 0;
    while index < address.len() {
        chunks.push(
            String::from(i64::from_str_radix(&address[index..index+2], 16).unwrap().to_string())
        );
        index += 2;
    }

    chunks.reverse();

    return chunks.join(separator);
}

fn bytesToPort(port: String) -> u16 {
    let (LSB, MSB) = port.split_at(2);
    ((i64::from_str_radix(MSB, 16).unwrap() as u16) << 8) + (i64::from_str_radix(LSB, 16).unwrap() as u16)
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

pub fn networkRoutes() -> Vec<NetworkRoute> {
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

        assert_eq!(String::new(), String::new());
    }
}