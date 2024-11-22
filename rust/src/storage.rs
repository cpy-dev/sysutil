use std::{fs, path};
use crate::utils::{*};

/// Contains NVME device information
#[derive(Debug, Clone)]
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
/// let byteSize = /* some sysutil function returning ByteSize */;
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
#[derive(Debug, Clone)]
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
        self.bytes as f32 / 1000_f32.powf(2_f32)
    }

    pub fn gb(&self) -> f32 {
        self.bytes as f32 / 1000_f32.powf(3_f32)
    }

    pub fn tb(&self) -> f32 {
        self.bytes as f32 / 1000_f32.powf(4_f32)
    }

    pub fn kib(&self) -> f32 {
        self.bytes as f32 / 1024_f32
    }

    pub fn mib(&self) -> f32 {
        self.bytes as f32 / 1024_f32.powf(2_f32)
    }

    pub fn gib(&self) -> f32 {
        self.bytes as f32 / 1024_f32.powf(3_f32)
    }

    pub fn tib(&self) -> f32 {
        self.bytes as f32 / 1024_f32.powf(4_f32)
    }
}

/// Encloses device name, size and startpoint relative to a partition
#[derive(Debug, Clone)]
pub struct StoragePartition {
    pub device: String,
    pub mountPoint: String,
    pub fileSystem: String,
    pub size: ByteSize,
    pub startPoint: usize
}

/// Contains information relative to a storage device in the system
#[derive(Debug, Clone)]
pub struct StorageDevice {
    pub model: String,
    pub device: String,
    pub size: ByteSize,
    pub partitions: Vec<StoragePartition>
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
                break
            }
        }

        let mut localPartitions = Vec::<StoragePartition>::new();
        for mount in mountPoints.split("\n") {
            if mount.contains(&device) {

                let splitted: Vec<&str> = mount.split(" ").collect();
                let device = splitted.first().unwrap().to_string();
                let deviceName: String = device.split("/").collect::<Vec<&str>>().get(2).unwrap().to_string();

                let mountPoint = splitted.get(1).unwrap().to_string();
                let fileSystem = splitted.get(2).unwrap().to_string();

                let mut partSize = ByteSize{bytes: 0};
                let mut startPoint = 0;

                for partition in partitions.split("\n") {
                    if partition.contains(&deviceName) {

                        partSize = ByteSize{
                            bytes: {
                                let tmp = partition.split(" ").collect::<Vec<&str>>();
                                tmp[tmp.len() - 2].parse::<usize>().unwrap()
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
                tmp.parse::<usize>().unwrap_or(0)
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
                    tmp.parse().unwrap_or(0)
                }
            };

            let startByte = {
                let tmp = readFile(format!("{}/{}/start", baseDir, partitionDir));
                tmp.parse::<usize>().unwrap_or(0)
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
