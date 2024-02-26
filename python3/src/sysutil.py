import dataclasses
import os
import sys
import time

class BatteryStatus:
    Charging = 'charging'
    Discharging = 'discharging'
    Full = 'full'

@dataclasses.dataclass
class Battery:
    capacity: int
    status: str

@dataclasses.dataclass
class ProcessorUsage:
    total: float
    user: float
    nice: float
    system: float
    idle: float
    iowait: float
    interrupt: float
    soft_interrupt: float

@dataclasses.dataclass
class CpuUsage:
    average: ProcessorUsage
    processors: [ProcessorUsage]

@dataclasses.dataclass
class NetworkRate:
    download: float
    upload: float

@dataclasses.dataclass
class TemperatureSensor:
    label: str
    temperature: float

@dataclasses.dataclass
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

@dataclasses.dataclass
class SchedulerPolicy:
    name: str
    scalingGovernor: str
    scalingDriver: str
    minimumScalingMHz: float
    maximumScalingMHz: float

@dataclasses.dataclass
class CPU:
    info: CpuInfo
    averageUsage: ProcessorUsage
    perProcessorUsage: [ProcessorUsage]
    schedulerPolicies: [SchedulerPolicy]

    def __init__(self):
        self.info = cpuInfo()

        usage = cpuUsage()
        self.averageUsage = usage.average
        self.perProcessorUsage = usage.processors

        self.schedulerPolicies = schedulerInfo()


    def update(self):
        usage = cpuUsage()
        self.averageUsage = usage.average
        self.perProcessorUsage = usage.processors
        self.schedulerPolicies = schedulerInfo()

@dataclasses.dataclass
class RamSize:
    gb: float
    gib: float

@dataclasses.dataclass
class VramSize:
    gb: float
    gib: float

class RouteType:
    TCP = 'tcp'
    TCP6 = 'tcp6'
    UDP = 'udp'
    UDP6 = 'udp6'

@dataclasses.dataclass
class NetworkRoute:
    routeType: str
    localAddress: str
    localPort: int
    remoteAddress: str
    remotePort: int

@dataclasses.dataclass
class ClockSource:
    current: str
    available: [str]

@dataclasses.dataclass
class Bios:
    vendor: str
    release: str
    version: str
    date: str

@dataclasses.dataclass
class Motherboard:
    name: str
    vendor: str
    version: str
    bios: Bios

@dataclasses.dataclass
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

@dataclasses.dataclass
class ByteSize:
    __bytes: int

    def b(self):
        return self.__bytes

    def kb(self):
        return self.__bytes / 1000

    def mb(self):
        return self.__bytes / 1000 / 1000

    def gb(self):
        return self.__bytes / 1000 / 1000 / 1000

    def tb(self):
        return self.__bytes / 1000 / 1000 / 1000 / 1000

    def kib(self):
        return self.__bytes / 1024

    def mib(self):
        return self.__bytes / 1024 / 1024

    def gib(self):
        return self.__bytes / 1024 / 1024 / 1024

    def tib(self):
        return self.__bytes / 1024 / 1024 / 1024 / 1024

@dataclasses.dataclass
class StoragePartition:
    device: str
    mountPoint: str
    filesystem: str
    size: ByteSize
    startPoint: str

@dataclasses.dataclass
class NvmeDevice:
    device: str
    pcieAddress: str
    model: str
    linkSpeedGTs: float
    pcieLanes: int
    size: ByteSize
    partitions: [StoragePartition]

@dataclasses.dataclass
class StorageDevice:
    model: str
    device: str
    size: ByteSize
    partitions: [StoragePartition]

def __linuxCheck():
    if not os.path.exists('/sys') or not os.path.exists('/proc'):
        raise Exception('Detected non-Linux system')

def __readFile(filePath):
    try:
        with open(filePath, 'r') as file:
            return file.read()

    except:
        return ''

def __batteryPath():
    DRIVER_DIR = '/sys/class/power_supply'
    batteries = []

    for dir in os.listdir(DRIVER_DIR):
        path = f'{DRIVER_DIR}/{dir}'

        if 'type' not in os.listdir(path):
            continue

        with open(f'{path}/type', 'r') as type:
            if type != 'Battery':
                continue

        if os.path.exists(f'{path}/status') and os.path.exists(f'{path}/capacity'):
            batteries.append(path)

    try:
        battery = batteries[0]

    except:
        return None

    return battery

def batteryInfo():
    __linuxCheck()

    batteryPath = __batteryPath()

    if batteryPath is None:
        return None

    try:
        with open(f'{batteryPath}/capacity', 'r') as file:
            capacity = file.read().strip()

    except:
        return None

    else:
        if not capacity:
            return None

    try:
        capacity = int(capacity)
    except:
        capacity = None

    try:
        with open(f'{batteryPath}/status', 'r') as file:
            status = file.read().strip()

    except:
        return None

    else:
        if not status:
            return None

    if status == 'Charging':
        status = BatteryStatus.Charging

    elif status == 'Discharging':
        status = BatteryStatus.Discharging

    elif status == 'Full':
        status = BatteryStatus.Full

    else:
        status = None

    return Battery(
        capacity=capacity,
        status=status
    )

def gpuUsage():
    __linuxCheck()

    try:
        with open('/sys/class/drm/card0/device/gpu_busy_percent', 'r') as file:
            return float(file.read().strip())

    except:
        return None

def __getStats():
    with open('/proc/stat', 'r') as file:
        statFile = file.read()

    lines = statFile.split('\n')
    intLines = []

    for line in lines:
        if 'cpu' not in line:
            continue

        splittedLine = line.split(' ')
        intLine = []

        for chunk in splittedLine:
            if chunk and 'cpu' not in chunk:

                intLine.append(int(chunk))
        intLines.append(intLine)

    return intLines

def cpuUsage():
    __linuxCheck()

    before = __getStats()
    time.sleep(0.25)
    after = __getStats()

    processors = []

    for i in range(len(before)):
        beforeLine = before[i]
        afterLine = after[i]

        beforeSum = sum(beforeLine)
        afterSum = sum(afterLine)

        delta = afterSum - beforeSum
        processors.append(
            ProcessorUsage(
                total=100 - (afterLine[3] - beforeLine[3]) * 100 / delta,
                user=(afterLine[0] - beforeLine[0]) * 100 / delta,
                nice=(afterLine[1] - beforeLine[1]) * 100 / delta,
                system=(afterLine[2] - beforeLine[2]) * 100 / delta,
                idle=(afterLine[3] - beforeLine[3]) * 100 / delta,
                iowait=(afterLine[4] - beforeLine[4]) * 100 / delta,
                interrupt=(afterLine[5] - beforeLine[5]) * 100 / delta,
                soft_interrupt=(afterLine[6] - beforeLine[6]) * 100 / delta,
            )
        )

    return CpuUsage(
        average=processors[0],
        processors=processors[1:]
    )

def cpuFrequency():
    __linuxCheck()

    with open('/proc/cpuinfo', 'r') as file:
        fileContent = file.read()

    frequencies = 0
    count = 0

    for line in fileContent.split('\n'):
        if 'cpu MHz' in line:

            frequencies += float(line.split(' ')[-1])
            count += 1

    if frequencies:
        return frequencies / count
    return None

def ramUsage():
    __linuxCheck()

    with open('/proc/meminfo', 'r') as file:
        fileContent = file.read()

    memTotal = 0
    memAvailable = 0

    for element in fileContent.split('\n'):
        if 'MemTotal' in element:
            memTotal = int(element.split(' ')[-2])

        elif 'MemAvailable' in element:
            memAvailable = int(element.split(' ')[-2])

    return 100 - memAvailable * 100 / memTotal

def __getRate():
    with open('/proc/net/dev', 'r') as file:
        stats = file.read()

    downloadRate = 0
    uploadRate = 0

    for line in stats.split('\n'):
        if ':' in line:

            data = []
            for chunk in line.split(' '):
                if chunk and ':' not in chunk:
                    data.append(chunk)

            downloadRate += int(data[0])
            uploadRate += int(data[8])

    return downloadRate, uploadRate

def networkRate():
    __linuxCheck()

    downBefore, upBefore = __getRate()
    time.sleep(0.5)
    downAfter, upAfter = __getRate()

    return NetworkRate (
        download=(downAfter - downBefore) / 0.5,
        upload=(upAfter - upBefore) / 0.5
    )

def temperatureSensors():
    __linuxCheck()

    DRIVER_DIR = '/sys/class/hwmon'
    sensorsDirectories = os.listdir(DRIVER_DIR)

    sensors = []
    for directory in sensorsDirectories:
        with open(f'{DRIVER_DIR}/{directory}/name', 'r') as labelFile:
            label = labelFile.read().strip()

        with open(f'{DRIVER_DIR}/{directory}/temp1_input', 'r') as temperatureFile:
            try:
                temperature = float(temperatureFile.read()) / 1000

            except:
                temperature = None

        sensors.append(
            TemperatureSensor(
                label=label,
                temperature=temperature
            )
        )

    return sensors

def cpuInfo():
    __linuxCheck()

    with open('/proc/cpuinfo', 'r') as file:
        infoFile = file.read()

    modelName = ''
    for line in infoFile.split('\n'):
        if 'model name' in line:
            modelName = line.split(':')[1].strip()
            break

    DRIVER_DIR = '/sys/devices/system/cpu'
    coreCount = 0
    dieCount = 0

    for processor in os.listdir(DRIVER_DIR):
        if 'cpu' not in processor:
            continue

        try:
            with open(f'{DRIVER_DIR}/{processor}/topology/core_id', 'r') as file:
                coreId = file.read()

                if int(coreId) > coreCount:
                    coreCount = int(coreId)
        except:
            pass

        try:
            with open(f'{DRIVER_DIR}/{processor}/topology/die_id', 'r') as file:
                coreId = file.read()

                if int(coreId) > coreCount:
                    coreCount = int(coreId)
        except:
            pass
    if coreCount % 2:
        coreCount += 1
    dieCount += 1

    with open('/proc/cpuinfo', 'r') as file:
        threadCount = file.read().count('processor')

    DRIVER_DIR = '/sys/devices/system/cpu/cpufreq'
    maxFrequency = 0

    governors = []
    clockBoost = None

    for policy in os.listdir(DRIVER_DIR):
        if 'boost' in policy:
            with open(f'{DRIVER_DIR}/{policy}', 'r') as boostFile:
                clockBoost = True if boostFile.read() == '1' else False

            continue

        elif 'policy' not in policy:
            continue

        with open(f'{DRIVER_DIR}/{policy}/scaling_available_governors', 'r') as file:
            localGovernors = file.read().strip().split(' ')

            for governor in localGovernors:
                if governor not in governors:
                    governors.append(governor)

        with open(f'{DRIVER_DIR}/{policy}/cpuinfo_max_freq', 'r') as file:
            if (maxFreq := int(file.read())) > maxFrequency:
                maxFrequency = maxFreq

    maxFrequency /= 1000
    arch = ''

    if sys.maxsize == 2 ** (64 - 1) - 1:
        arch = '64 bit'

    elif sys.maxsize == 2 ** (32 - 1) - 1:
        arch = '32 bit'

    byteOrder = ''
    if sys.byteorder == 'little':
        byteOrder = 'Little Endian'

    elif sys.byteorder == 'big':
        byteOrder = 'Big Endian'

    return CpuInfo(
        modelName=modelName,
        cores=coreCount,
        threads=threadCount,
        dies=dieCount,
        governors=governors,
        maxFrequencyMHz=maxFrequency,
        clockBoost=clockBoost,
        architecture=arch,
        byteOrder=byteOrder
    )

def ramSize():
    __linuxCheck()

    with open('/proc/meminfo', 'r') as file:
        memInfo = file.read().split('\n')

    memTotal = 0
    for line in memInfo:

        if 'MemTotal' in line:
            memTotal = int(line.split(' ')[-2].strip())

    GiB = memTotal * 1000 / 1024 / 1024 / 1024
    GB = memTotal / 1000 / 1000

    return RamSize(
        gb=GB,
        gib=GiB
    )

def schedulerInfo():
    __linuxCheck()

    DRIVER_DIR = '/sys/devices/system/cpu/cpufreq'
    policies = []

    for dir in os.listdir(DRIVER_DIR):
        if 'policy' not in dir:
            continue

        policyName = dir

        with open(f'{DRIVER_DIR}/{dir}/scaling_governor', 'r') as file:
            scalingGovernor = file.read().strip()

        with open(f'{DRIVER_DIR}/{dir}/scaling_driver', 'r') as file:
            scalingDriver = file.read().strip()

        with open(f'{DRIVER_DIR}/{dir}/scaling_max_freq', 'r') as file:
            scalingMaxFreq = int(file.read().strip())

        with open(f'{DRIVER_DIR}/{dir}/scaling_min_freq', 'r') as file:
            scalingMinFreq = int(file.read().strip())

        policies.append(
            SchedulerPolicy(
                name=policyName,
                scalingGovernor=scalingGovernor,
                scalingDriver=scalingDriver,
                minimumScalingMHz=scalingMinFreq,
                maximumScalingMHz=scalingMaxFreq
            )
        )

    return policies

def vramSize():
    __linuxCheck()

    try:
        with open('/sys/class/drm/card0/device/mem_info_vram_total', 'r') as file:
            fileContent = file.read()

        intSize = int(fileContent.strip())

        return VramSize(
            gb=intSize / 1000 / 1000 / 1000,
            gib=intSize / 1024 / 1024 / 1024
        )
    except:
        return None

def vramUsage():
    __linuxCheck()

    try:
        with open('/sys/class/drm/card0/device/mem_info_vram_total', 'r') as file:
            fileContent = file.read()

        intSize = int(fileContent.strip())

        with open('/sys/class/drm/card0/device/mem_info_vram_used', 'r') as file:
            fileContent = file.read()

        intUsed = int(fileContent.strip())

        return intUsed * 100 / intSize

    except:
        return None

def __bytesToAddress(address, separator):
    chunks = []

    for index in range(0, len(address), 2):
        chunks.append(
            str(int(address[index:index+2], 16))
        )

    chunks = chunks[::-1]
    return separator.join(chunks)

def __bytesToPort(port):
    LSB = port[:2]
    MSB = port[2:]

    return (int(MSB, 16) << 8) + (int(LSB, 16))

def __getRoutes(filePath, separator, routeType):
    routes = []

    try:
        with open(filePath, 'r') as file:
            fileContent = file.read()

    except:
        return []

    for line in fileContent.split('\n'):
        if ':' not in line:
            continue

        splittedLine = line.strip().split(' ')
        local = splittedLine[1].split(':')
        remote = splittedLine[2].split(':')

        localAddress = __bytesToAddress(local[0], separator)
        localPort = __bytesToPort(local[1])

        remoteAddress = __bytesToAddress(remote[0], separator)
        remotePort = __bytesToPort(remote[1])

        routes.append(
            NetworkRoute (
                routeType=routeType,
                localAddress=localAddress,
                localPort=localPort,
                remoteAddress=remoteAddress,
                remotePort=remotePort
            )
        )

    return routes

def networkRoutes():
    __linuxCheck()

    routes = []

    routes.extend(
        __getRoutes(
            '/proc/net/tcp', '.', RouteType.TCP
        )
    )

    routes.extend(
        __getRoutes(
            '/proc/net/udp', '.', RouteType.UDP
        )
    )

    routes.extend(
        __getRoutes(
            '/proc/net/tcp6', ':', RouteType.TCP6
        )
    )

    routes.extend(
        __getRoutes(
            '/proc/net/udp6', ':', RouteType.UDP6
        )
    )

    return routes

def clockSource():
    __linuxCheck()

    currentClockSource = ''
    try:
        with open('/sys/devices/system/clocksource/clocksource0/current_clocksource', 'r') as file:
            currentClockSource = file.read().strip()
    except:
        pass

    availableClockSources = []
    try:
        with open('/sys/devices/system/clocksource/clocksource0/available_clocksource', 'r') as file:
            availableClockSources = file.read().strip().split(' ')
    except:
        pass

    return ClockSource(
        current=currentClockSource,
        available=availableClockSources
    )

def biosInfo():
    __linuxCheck()

    vendor = ''
    try:
        with open('/sys/devices/virtual/dmi/id/bios_vendor', 'r') as file:
            vendor = file.read().strip()
    except:
        pass

    release = ''
    try:
        with open('/sys/devices/virtual/dmi/id/bios_release', 'r') as file:
            release = file.read().strip()
    except:
        pass

    version = ''
    try:
        with open('/sys/devices/virtual/dmi/id/bios_version', 'r') as file:
            version = file.read().strip()
    except:
        pass

    date = ''
    try:
        with open('/sys/devices/virtual/dmi/id/bios_date', 'r') as file:
            date = file.read().strip()
    except:
        pass

    return Bios(
        vendor=vendor,
        version=version,
        release=release,
        date=date
    )

def motherboardInfo():
    __linuxCheck()

    name = ''
    try:
        with open('/sys/devices/virtual/dmi/id/board_name', 'r') as file:
            name = file.read().strip()
    except:
        pass

    vendor = ''
    try:
        with open('/sys/devices/virtual/dmi/id/board_vendor', 'r') as file:
            vendor = file.read().strip()
    except:
        pass

    version = ''
    try:
        with open('/sys/devices/virtual/dmi/id/board_version', 'r') as file:
            version = file.read().strip()
    except:
        pass

    bios = biosInfo()
    return Motherboard (
        name=name,
        version=version,
        vendor=vendor,
        bios=bios
    )

def __bytesToInt(bytes):
    res = 0
    shift = 8 * len(bytes)

    if sys.byteorder == 'little':
        bytes = bytes[::-1]

    for byte in bytes:
        shift -= 8
        res += byte << shift

    return res

def gpuMetrics():
    __linuxCheck()

    try:
        with open('/sys/class/drm/card0/device/gpu_metrics', 'rb') as file:
            bytes = file.read()

    except:
        pass

    if bytes[2] != 1:
        return None

    content = bytes[3]
    bytes = bytes[4:]

    return GpuMetrics (
        temperatureEdge=__bytesToInt(bytes[0:2] if content else bytes[8:10]),
        temperatureHotspot=__bytesToInt(bytes[2:4] if content else bytes[10:12]),
        temperatureMem=__bytesToInt(bytes[4:6] if content else bytes[12:14]),
        temperatureVrgfx=__bytesToInt(bytes[6:8] if content else bytes[14:16]),
        temperatureVrsoc=__bytesToInt(bytes[8:10] if content else bytes[16:18]),
        temperatureVrmem=__bytesToInt(bytes[10:12] if content else bytes[18:20]),
        averageSocketPower=__bytesToInt(bytes[18:20] if content else bytes[26:28]),
        averageGfxclkFrequency=__bytesToInt(bytes[36:38]),
        averageSockclkFrequency=__bytesToInt(bytes[38:40]),
        averageUclkFrequency=__bytesToInt(bytes[40:42]),
        currentGfxclk=__bytesToInt(bytes[50:52]),
        currentSockclk=__bytesToInt(bytes[52:54]),
        throttleStatus=__bytesToInt(bytes[64:68]),
        currentFanSpeed=__bytesToInt(bytes[68:70]),
        pcieLinkWidth=__bytesToInt(bytes[70:72]),
        pcieLinkSpeed=__bytesToInt(bytes[72:74]),
    )

def nvmeDevices():
    __linuxCheck()

    baseDir = '/sys/class/nvme'

    try:
        dirContent = os.listdir(baseDir)
    except:
        return []

    devices = []
    partitions = __readFile('/proc/partitions').strip()
    mountPoints = __readFile('/proc/mounts').strip()

    for device in dirContent:
        address = __readFile(f'{baseDir}/{device}/address').strip()
        model = __readFile(f'{baseDir}/{device}/model').strip()

        linkSpeed = __readFile(f'{baseDir}/{device}/device/current_link_speed').strip()
        linkSpeed = float(linkSpeed.split(' ')[0])

        pcieLanes = __readFile(f'{baseDir}/{device}/device/current_link_width').strip()
        pcieLanes = int(pcieLanes)

        size = 0
        for partitionLine in partitions.strip().split('\n')[2:]:
            if device in partitionLine:
                size = int(partitionLine.split(' ')[-2])

        localPartitions = []
        for mount in mountPoints.split('\n'):

            if device in mount:
                splitted = mount.split(' ')
                device = splitted[0]
                deviceName = device.split('/')[2]

                mountPoint = splitted[1]
                filesystem = splitted[2]

                partSize = ByteSize(0)
                startPoint = 0

                for partition in partitions.split('\n'):
                    if deviceName in partition:
                        try:
                            partSize = ByteSize(
                                int(
                                    __readFile(f'/sys/class/block/{deviceName}/size').strip()
                                )
                            )
                        except:
                            pass

                        try:
                            startPoint = int(
                                __readFile(f'/sys/class/block/{deviceName}/start').strip()
                            )
                        except:
                            pass

                        break

                localPartitions.append(StoragePartition(
                    device=device,
                    mountPoint=mountPoint,
                    filesystem=filesystem,
                    size=partSize,
                    startPoint=startPoint
                ))

        devices.append(NvmeDevice(
            device=device,
            model=model,
            pcieAddress=address,
            linkSpeedGTs=linkSpeed,
            pcieLanes=pcieLanes,
            size=ByteSize(size),
            partitions=localPartitions
        ))

    return devices

def storageDevices():
    __linuxCheck()

    baseDir = '/sys/class/block'

    try:
        dirContent = os.listdir(baseDir)
    except:
        return []

    mountPoints = __readFile('/proc/mounts').split('\n')

    devices = []
    for dir in dirContent:
        if 'sd' not in dir or len(dir) != 3:
            continue

        device = f'/dev/{dir}'
        size = __readFile(f'{baseDir}/{dir}/size').strip()

        try:
            size = ByteSize(int(size))

        except:
            size = ByteSize(0)

        model = __readFile(f'{baseDir}/{dir}/device/model').strip()
        partitions = []

        for partitionDir in dirContent:
            if dir not in partitionDir:
                continue

            if len(partitionDir) <= 3:
                continue
            
            partitionSize = __readFile(f'{baseDir}/{partitionDir}/size').strip()
            partitionSize = ByteSize(int(partitionSize) if partitionSize else 0)
            
            startByte = __readFile(f'{baseDir}/{partitionDir}/start').strip()
            startByte = ByteSize(int(startByte) if startByte else 0)

            mountPoint = ''
            fileSystem = ''

            for mount in mountPoints:
                if f'/dev/{partitionDir} ' in mount:
                    splitted = mount.split(' ')

                    mountPoint = splitted[1]
                    fileSystem = splitted[2]
                    break

            partitions.append(
                StoragePartition(
                    device=f'/dev/{partitionDir}',
                    mountPoint=mountPoint,
                    filesystem=fileSystem,
                    size=partitionSize,
                    startPoint=startByte
                )
            )

        devices.append(
            StorageDevice (
                model=model,
                device=device,
                size=size,
                partitions=partitions
            )
        )

    return devices


if __name__ == '__main__':
    print(cpuUsage())
    print(f'RAM usage:', ramUsage())

    print(networkRate())
    print(f'GPU usage:', gpuUsage())

    print(temperatureSensors())
    print(cpuInfo())

    print(ramSize())
    print(schedulerInfo())

    print(batteryInfo())
    print(vramSize())

    print('VRAM usage:', vramUsage())
    print(networkRoutes())

    print(CPU())

    print(clockSource())
    print(motherboardInfo())

    print(gpuMetrics())
    print(storageDevices())
    print(nvmeDevices())
