# sysutil-lib
- Linux system information library

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

### Cpu
```python3
class Cpu:
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

### NetworkRoute
```python3
class NetworkRoute:
    routeType: str
    localAddress: str
    localPort: int
    remoteAddress: str
    remotePort: int
```
- represents a network route

## Functions

## Functions
```python3
def cpuUsage() -> CpuUsage
```
- returns the cpu usage, both average and processor-wise, all the values are percentage
```python3
def cpuFrequency() -> float
```
- returns CPU frequency in MHz

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
def cpuInfo() -> Cpu
```
- returns the cpu base information, enclosed in the `Cpu` data structure

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