use std::collections::HashMap;
use crate::utils::{*};

/// Contains the information regarding a bus input
#[derive(Debug)]
pub struct BusInput {
    pub bus: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
    pub name: String,
    pub physicalPath: String,
    pub sysfsPath: String,
    pub uniqueIdentifier: String,
    pub handles: Vec<String>,
    pub properties: usize,
    pub events: usize,
    pub keys: Vec<String>,
    pub miscellaneousEvents: usize,
    pub led: usize
}

impl BusInput {
    fn new() -> BusInput {
        BusInput {
            bus: 0,
            vendor: 0,
            product: 0,
            version: 0,
            name: String::new(),
            physicalPath: String::new(),
            sysfsPath: String::new(),
            uniqueIdentifier: String::new(),
            handles: Vec::<String>::new(),
            properties: 0,
            led: 0,
            miscellaneousEvents: 0,
            events: 0,
            keys: Vec::<String>::new()
        }
    }
}

fn hexToUsize(hexadecimal: String) -> usize {
    let mut hexaTable: HashMap<&str, usize> = HashMap::new();
    hexaTable.insert("0", 0);
    hexaTable.insert("1", 1);
    hexaTable.insert("2", 2);
    hexaTable.insert("3", 3);
    hexaTable.insert("4", 4);
    hexaTable.insert("5", 5);
    hexaTable.insert("6", 6);
    hexaTable.insert("7", 7);
    hexaTable.insert("8", 8);
    hexaTable.insert("9", 9);
    hexaTable.insert("a", 10);
    hexaTable.insert("b", 11);
    hexaTable.insert("c", 12);
    hexaTable.insert("d", 13);
    hexaTable.insert("e", 14);
    hexaTable.insert("f", 15);
    hexaTable.insert("A", 10);
    hexaTable.insert("B", 11);
    hexaTable.insert("C", 12);
    hexaTable.insert("D", 13);
    hexaTable.insert("E", 14);
    hexaTable.insert("F", 15);

    let hex = hexadecimal.clone();

    let mut power = hex.len();
    let mut res: usize = 0;

    for chr in hex.chars() {
        let char = chr.to_string();

        res += hexaTable.get(char.as_str()).unwrap() * (16_i32.pow(power as u32) as usize);
        power -= 1;
    }

    return res
}

/// Returns a vector containing all the bus inputs found in procfs
pub fn busInput() -> Vec<BusInput> {
    let mut inputs = Vec::<BusInput>::new();
    let fileContent = readFile("/proc/bus/input/devices");

    for chunk in fileContent.split("\n\n") {
        if chunk.trim().is_empty() {
            continue;
        }

        let mut bus = BusInput::new();
        for line in chunk.trim().split("\n") {

            if line.contains("I: ") {
                for block in line.trim().split(" ") {
                    if block.contains("Bus=") {
                        bus.bus = hexToUsize(block.replace("Bus=", "")) as u16;

                    } else if block.contains("Vendor=") {
                        bus.vendor = hexToUsize(block.replace("Vendor=", "")) as u16;

                    } else if block.contains("Version=") {
                        bus.version = hexToUsize(block.replace("Version=", "")) as u16;

                    } else if block.contains("Product=") {
                        bus.product = hexToUsize(block.replace("Product=", "")) as u16;
                    }
                }
            } else if line.contains("N: Name=") {
                bus.name = line.replace("N: Name=", "").replace("\"", "");

            } else if line.contains("P: Phys=") {
                bus.physicalPath = line.replace("P: Phys=", "");

            } else if line.contains("S: Sysfs=") {
                bus.sysfsPath = line.replace("S: Sysfs=", "/sys");

            } else if line.contains("U: Uniq=") {
                bus.uniqueIdentifier = line.replace("U: Uniq=", "");

            } else if line.contains("H: Handlers=") {
                bus.handles = {
                    let mut binding = Vec::<String>::new();

                    for handler in line.replace("H: Handlers=", "").split(" ") {
                        if handler.is_empty() {
                            continue
                        }

                        binding.push(handler.to_string())
                    }

                    binding
                };

            } else if line.contains("B: PROP=") {
                bus.properties = hexToUsize(line.replace("B: PROP=", ""));

            } else if line.contains("B: EV=") {
                bus.events = hexToUsize(line.replace("B: EV=", ""));

            } else if line.contains("B: KEY=") {
                bus.keys = {
                    let mut binding = Vec::<String>::new();

                    for key in line.replace("B: KEY=", "").split(" ") {
                        binding.push(key.to_string());
                    }

                    binding
                };

            } else if line.contains("B: MSC=") {
                bus.miscellaneousEvents = hexToUsize(line.replace("B: MSC=", ""));

            } else if line.contains("B: LED=") {
                bus.led = hexToUsize(line.replace("B: LED=", ""));
            }
        }

        inputs.push(bus);
    }

    return inputs
}
