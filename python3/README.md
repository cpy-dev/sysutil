# sysutil-lib
- Linux system information library

## Warning
- this library is ment to be used only in linux systems
- it is possible to write code using it on other systems, but it will not allow to run the code, raising an exception before execution

## Other implementations
- this library is also available for Rust
- check it out at [crates.io](https://crates.io/crates/sysutil)

## Importation
```python
import sysutil
```

## Data structures
### ProcessorUsage
```python3
class ProcessorUsage:
    total: float
    user: float
    nice: float
    system: float
    idle: float
    iowait: float
    interrupt: float
    soft_interrupt: float
```
- data structure which encloses the different parameters relative to processor usage

### CpuUsage
```python3
class CpuUsage:
    average: ProcessorUsage
    processors: [ProcessorUsage]
```
- contains the average CPU usage, and the specific usage for each processor

### CpuInfo
```python3
class CpuInfo:
    modelName: str
    cores: int
    threads: int
    dies: int
    governors: [str]
    maxFrequencyMHz: float
    clockBoost: bool
    architecture: str
    byteOrder: str
```
- contains base information relative to the CPU

### SchedulerPolicy
```python3
class SchedulerPolicy:
    name: str
    scalingGovernor: str
    scalingDriver: str
    minimumScalingMHz: float
    maximumScalingMHz: float
```
- contains scheduler information relative to a processor in your system

### RamSize
```python3
class RamSize:
    gb: float
    gib: float
```
- contains total ram size, both in GB (1000^3 bytes) and GiB (1024^3 bytes)

### NetworkRate
```python3
class NetworkRate:
    download: float
    upload: float
```
- contains total upload and download network rate (in bytes)

### TemperatureSensor
```python3
class TemperatureSensor:
    label: str
    temperature: float
```
- contains sensor name (label) and the recorded temperature

### Battery
```python3
class Battery:
    capacity: int
    status: str
```
- contains capacity and status of battery

### VramSize
```python3
class VramSize:
    gb: float
    gib: float
```
- contains total gpu's vram size, both in GB (1000^3 bytes) and GiB (1024^3 bytes)

### RouteType
```python3
class RouteType:
    TCP = 'tcp'
    TCP6 = 'tcp6'
    UDP = 'udp'
    UDP6 = 'udp6'
```

### RouteStatus
```python3
class RouteStatus:
    ESTABLISHED = 'established'
    SYN_SENT = 'syn sent'
    SYN_RECEIVED = 'syn received'
    FIN_WAIT1 = 'fin wait 1'
    FIN_WAIT2 = 'fin wait 2'
    TIME_WAIT = 'time wait'
    CLOSED = 'closed'
    CLOSE_WAIT = 'close wait'
    LAST_ACKNOWLEDGEMENT = 'last acknowledgement'
    LISTENING = 'listening'
    CLOSING = 'closing'
    NEW_SYN_RECEIVED = 'new syn received'
```

### NetworkRoute
```python3
class NetworkRoute:
    routeType: str
    localAddress: str
    localPort: int
    remoteAddress: str
    remotePort: int
    routeStatus: str 
```
- represents a network route

### CPU
```python3
class CPU:
    info: CpuInfo
    averageUsage: ProcessorUsage
    perProcessorUsage: [ProcessorUsage]
    schedulerPolicies: [SchedulerPolicy]
```
- encloses all cpu data available in the library

#### Methods
```ptyhon3
cpu = CPU()
```
- standard constructor

```python3
cpu = CPU()

cpu.update()
```
- `update()` method updates usages and scheduler status

### Clocksource
```python
class ClockSource:
    current: str
    available: [str]
```
- contains current clock source and the available ones

### Bios
```python
class Bios:
    vendor: str
    release: str
    version: str
    date: str
```
- contains information relative to the installed bios

### Motherboard
```python
class Motherboard:
    name: str
    vendor: str
    version: str
    bios: Bios
```
- contains information relative to the motherboard and the installed bios

### GpuMetrics
```python
class GpuMetrics:
    temperatureEdge: int
    temperatureHotspot: int
    temperatureMem: int
    temperatureVrgfx: int
    temperatureVrsoc: int
    temperatureVrmem: int
    averageSocketPower: int
    averageGfxclkFrequency: int
    averageSockclkFrequency: int
    averageUclkFrequency: int
    currentGfxclk: int
    currentSockclk: int
    throttleStatus: int
    currentFanSpeed: int
    pcieLinkWidth: int
    pcieLinkSpeed: int
```
- encloses gpu metrics parameters

### Bytesize
```python
class ByteSize:
    __bytes: int
```
- Bytes size data structure implementing methods to convert in various size orders

### NvmeDevice
```python
class NvmeDevice:
    device: str
    pcieAddress: str
    model: str
    linkSpeedGTs: float
    pcieLanes: int
    size: ByteSize
    partitions: [StoragePartition]
```
- Contains NVME device information

### StoragePartition
```python
class StoragePartition:
    device: str
    mountPoint: str
    filesystem: str
    size: ByteSize
    startPoint: str
```
- Encloses device name, size and startpoint relative to a partition

### StorageDevice
```python
class StorageDevice:
    model: str
    device: str
    size: ByteSize
    partitions: [StoragePartition]
```
- Contains information relative to a storage device in the system

### Frequency
```python
class Frequency:
    _khz: float
```
- Contains frequency value in kHz, implements conversion methods for kHz, MHz, GHz

### ProcessorFrequency
```python
class ProcessorFrequency:
    processorID: str
    frequency: Frequency
```
- Contains processor id and its frequency 

### CpuFrequency
```python
class CpuFrequency:
    average: Frequency
    processors: [ProcessorFrequency]
```
- Contains cpu frequency, both average and processor wise

### Backlight
```python
class Backlight:
    brightness: int
    maxBrightness: int
```
- Holds information about backlight

### Load
```python
class Load:
    oneMinute: int
    fiveMinutes: int
    fifteenMinutes: int
```
- holds load values 

### IPv4
```python
class IPv4:
    address: str
    interface: str
    broadcast: str
    cidr: int
    netmask: str
```
- contains the various parameters for an IPv4 address in the system

### BusInput
```python3
class BusInput:
    bus: int
    vendor: int
    product: int
    version: int
    name: str
    physicalPath: str
    sysfsPath: str
    uniqueIdentifier: str
    handles: [str]
    properties: int
    events: int
    keys: [str]
    miscellaneousEvents: int
    led: int
```
- contains the information regarding a bus input


## Functions
```python3
def cpuUsage() -> CpuUsage
```
- returns the cpu usage, both average and processor-wise, all the values are percentage

```python3
def cpuFrequency() -> CpuFrequency
```
- returns CPU frequency, both average and processor wise

```python3
def ramUsage() -> float
```
- returns ram usage percentage

```python3
def networkRate() -> NetworkRate
```
- returns network rate (download and upload), expressed in bytes

```python3
def temperatureSensors() -> [TemperatureSensor]
```
- returns every temperature sensor in `TemperatureSensor` format

```python3
def cpuInfo() -> CpuInfo
```
- returns the cpu base information, enclosed in the `CpuInfo` data structure

```python3
def ramSize() -> RamSize
```
- returns ram size as specified in the `RamSize` data structure

```python3
def schedulerInfo() -> [SchedulerPolicy]
```
- returns scheduler information for each processor

```python3
def gpuUsage() -> float
```
- returns gpu usage percentage
- yet tested only on AMD 7000 series GPUs, returns `None` in case it's not capable to retrieve information

```python3
def batteryInfo() -> Battery 
```
- returns battery status and capacity

```python3
def vramSize() -> VramSize
```
- returns vram size as specified in the `VramSize` data structure

```python3
def vramUsage() -> float
```
- returns vram usage percentage

```python3
def networkRoutes() -> [NetworkRoute]
```
- returns a list containing each internal network route

```python
def clockSource() -> ClockSource
```
- returns the currently active clock source and the different ones available, enclosed in `ClockSource` struct

```python
def biosInfo() -> Bios
```
- returns information about the currently installed BIOS

```python
def motherboardInfo() -> Motherboard
```
- returns information about the motherboard

```python
def gpuMetrics() -> GpuMetrics
```
- returns metrics parameters from the amdgpu driver

```python
def nvmeDevices() -> [NvmeDevices]
```
- Returns a vector containing all NVME devices found in the system

```python
def storageDevices() -> [StorageDevices]
```
- Returns a vector containing all storage devices (NVME excluded) in the system

```python
def getBacklight() -> Backlight
```
- Returns the current backlight brightness and the maximum possible value or `None` if it's not possible to retrieve data

```python
def getLoad() -> Load 
```
- returns the load for the past one, five and fifteen minutes 

```python
def getIPv4() -> [IPv4]
```
- returns a list of `IPv4` object; each one is related to an IPv4 address in the system

```python 
def busInput() -> [BusInput]
```
- returns a list of `BusInput` objects, representing the bus inputs found in procfs

```python
def exportJson() -> dict
```
- returns a `dict` containing all the information which `sysutil` can provide 