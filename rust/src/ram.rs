use std::fs::read_dir;
use crate::utils::{*};

/// Contains all information about RAM
#[derive(Debug)]
pub struct RAM {
    pub size: ByteSize,
    pub usage: f32,
    pub frequency: Option<usize>,
    pub busWidth: Option<usize>
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            size: ramSize(),
            usage: ramUsage(),
            frequency: ramFrequency(),
            busWidth: ramBusWidth()
        }
    }
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

/// Returns RAM size using the `ByteSize` data structure
pub fn ramSize() -> ByteSize {
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

    ByteSize::fromBytes(uMemTotal * 1000)
}

/// Returns RAM frequency in MT/s
pub fn ramFrequency() -> Option<usize> {
    let kfdTopologyNodes = "/sys/class/kfd/kfd/topology/nodes/";
    for dir in read_dir(kfdTopologyNodes).unwrap() {
        let path = dir.unwrap().path();
        let directory = path.to_str().unwrap();

        let content = readFile(format!("{}/properties", directory));
        let mut isCpu = false;

        for line in content.split("\n") {
            if line.contains("cpu_cores_count") {
                let splitedLine = line.split(" ").collect::<Vec<&str>>();
                match splitedLine.last() {
                    Some(cores) => {
                        if cores.parse::<usize>().unwrap() != 0 {
                            isCpu = true;
                        }
                    },

                    None => {
                        continue
                    }
                }
            }

            if isCpu {
                break
            }
        }

        if isCpu {
            let memBanksInfo = readFile(format!("{}/mem_banks/0/properties", directory));
            let mut frequencyLine = String::new();

            for line in memBanksInfo.split("\n") {
                if line.contains("mem_clk_max") {
                    frequencyLine = line.to_string();
                    break
                }
            }

            let frequency = {
                let binding = frequencyLine.split(" ").collect::<Vec<&str>>();
                let last = binding.last().unwrap();
                last.parse::<usize>().unwrap()
            };

            return Some(frequency);
        }
    }

    return None;
}

/// Returns RAM bus width in bits
pub fn ramBusWidth() -> Option<usize> {
    let kfdTopologyNodes = "/sys/class/kfd/kfd/topology/nodes/";
    for dir in read_dir(kfdTopologyNodes).unwrap() {
        let path = dir.unwrap().path();
        let directory = path.to_str().unwrap();

        let content = readFile(format!("{}/properties", directory));
        let mut isCpu = false;

        for line in content.split("\n") {
            if line.contains("cpu_cores_count") {
                let splitedLine = line.split(" ").collect::<Vec<&str>>();
                match splitedLine.last() {
                    Some(cores) => {
                        if cores.parse::<usize>().unwrap() != 0 {
                            isCpu = true;
                        }
                    },

                    None => {
                        continue
                    }
                }
            }

            if isCpu {
                break
            }
        }

        if isCpu {
            let memBanksInfo = readFile(format!("{}/mem_banks/0/properties", directory));
            let mut widthLine = String::new();

            for line in memBanksInfo.split("\n") {
                if line.contains("width") {
                    widthLine = line.to_string();
                    break
                }
            }

            let width = {
                let binding = widthLine.split(" ").collect::<Vec<&str>>();
                let last = binding.last().unwrap();
                last.parse::<usize>().unwrap()
            };

            return Some(width);
        }
    }

    return None;
}