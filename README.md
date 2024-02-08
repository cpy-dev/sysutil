# sysutil-lib
- Linux system information library

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
- data structure wich encloses the different parameters relative to processor usage

### CpuUsage
```rust
pub struct CpuUsage {
    average: ProcessorUsage,
    processors: Vec::<ProcessorUsage>
}
```
- contains the average CPU usage, and the specific usage for each processor

### Cpu
```rust
pub struct Cpu {
    modelName: String,
    cores: usize,
    threads: usize,
    dies: usize,
    governors: Vec<String>,
    maxFrequencyMHz: f32,
    architecture: String
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
    temperature: f32
}
```
- contains sensor name (label) and the recorded temperature

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
pub fn cpuInfo() -> Cpu
```
- returns the cpu base information, enclosed in the `Cpu` data structure

```rust
pub fn ramSize() -> RamSize
```
- returns ram size as specified in the `RamSize` data structure

```rust
pub fn schedulerInfo() -> Vec<SchedulerPolicy>
```
- returns scheduoler information for each processor

```rust
pub fn gpuUsage() -> f32
```
- returns gpu usage percentage
- yet tested only on AMD 7000 series GPUs