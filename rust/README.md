# sysutil-lib
- Linux system information library

## Importation
```rust
use sysutil;
```

## Data structures
### ProcessorUsage
```rust
pub struct ProcessorUsage {
    total: f32,
    user: f32,
    nice: f32,
    system: f32,
    idle: f32,
    iowait: f32,
    interrupt: f32,
    soft_interrupt: f32
}
```
- data structure which encloses the different parameters relative to processor usage

### CpuUsage
```rust
pub struct CpuUsage {
    average: ProcessorUsage,
    processors: Vec<ProcessorUsage>
}
```
- contains the average CPU usage, and the specific usage for each processor

### CpuInfo
```rust
pub struct CpuInfo {
    modelName: String,
    cores: usize,
    threads: usize,
    dies: usize,
    governors: Vec<String>,
    maxFrequencyMHz: f32,
    clockBoost: Option<bool>,
    architecture: String,
    byteOrder: String
}
```
- contains base information relative to the CPU

### SchedulerPolicy
```rust
pub struct SchedulerPolicy {
    name: String,
    scalingGovernor: String,
    scalingDriver: String,
    minimumScalingMHz: f32,
    maximumScalingMHz: f32
}
```
- contains scheduler information relative to a processor in your system

### RamSize
```rust
pub struct RamSize {
    gb: f32,
    gib: f32
}
```
- contains total ram size, both in GB (1000^3 bytes) and GiB (1024^3 bytes) 

### NetworkRate
```rust
pub struct NetworkRate {
    download: f32,
    upload: f32
}
```
- contains total upload and download network rate (in bytes)

### TemperatureSensor
```rust
pub struct TemperatureSensor {
    label: String,
    temperature: Option<f32>
}
```
- contains sensor name (label) and the recorded temperature

### Battery

```rust
pub struct Battery {
    capacity: u8,
    status: BatteryStatus, 
}
```

- contains capacity and status of battery

### BatteryStatus

```rust
enum BatteryStatus {
    Charging,
    Discharging,
    Full,
}
```

- represents status of battery

### VramSize
```rust
pub struct VramSize {
    gb: f32,
    gib: f32
}
```
- contains total gpu's vram size, both in GB (1000^3 bytes) and GiB (1024^3 bytes)

### RouteType
```rust
pub enum RouteType {
    TCP,
    TCP6,
    UDP,
    UDP6
}
```

### NetworkRoute
```rust
pub struct NetworkRoute {
    routeType: RouteType,
    localAddress: String,
    localPort: u16,
    remoteAddress: String,
    remotePort: u16
}
```
-  represents a network route and its type, containing local address+port and remote address+port

### CPU
```rust
pub struct CPU {
    pub info: CpuInfo,
    pub averageUsage: ProcessorUsage,
    pub perProcessorUsage: Vec<ProcessorUsage>,
    pub schedulerPolicies: Vec<SchedulerPolicy>
}
```
- encloses all cpu data available in the library
#### Methods

```rust 
CPU::new()
```
- standard constructor, generates a new instance

```rust
let mut cpu = CPU::new();

cpu.update();
```
- `update()` method updates usages and scheduler status

## Functions
```rust
pub fn cpuUsage() -> CpuUsage
```
- returns the cpu usage, both average and processor-wise, all the values are percentage
```rust
pub fn cpuFrequency() -> f32
```
- returns CPU frequency in MHz

```rust
pub fn ramUsage() -> f32 
```
- returns ram usage percentage

```rust
pub fn networkRate() -> NetworkRate
```
- returns network rate (download and upload), expressed in bytes

```rust
pub fn temperatureSensors() -> Vec<TemperatureSensor>
```
- returns every temperature sensor in `TemperatureSensor` format

```rust
pub fn cpuInfo() -> CpuInfo
```
- returns the cpu base information, enclosed in the `CpuInfo` data structure

```rust
pub fn ramSize() -> RamSize
```
- returns ram size as specified in the `RamSize` data structure

```rust
pub fn schedulerInfo() -> Vec<SchedulerPolicy>
```
- returns scheduler information for each processor

```rust
pub fn gpuUsage() -> Option<f32>
```
- returns gpu usage percentage
- yet tested only on AMD 7000 series GPUs, returns `None` in case it's not capable to retrieve information

```rust
pub fn batteryInfo() -> Option<Battery> 
```
- returns battery status and capacity

```rust
pub fn vramSize() -> Option<VramSize>
```
- returns vram size as specified in the `VramSize` data structure

```rust
pub fn vramUsage() -> Option<f32>
```
- returns vram usage percentage

```rust
pub fn networkRoutes() -> Vec<Route>
```
- returns a list containing each internal network route