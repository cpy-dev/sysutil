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

@dataclasses.dataclass
class RamSize:
    gb: float
    gib: float

@dataclasses.dataclass
class SchedulerPolicy:
    name: str
    scalingGovernor: str
    scalingDriver: str
    minimumScalingMHz: float
    maximumScalingMHz: float

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
    downBefore, upBefore = __getRate()
    time.sleep(0.5)
    downAfter, upAfter = __getRate()

    return NetworkRate (
        download=(downAfter - downBefore) / 0.5,
        upload=(upAfter - upBefore) / 0.5
    )

def temperatureSensors():
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
                None

        sensors.append(
            TemperatureSensor(
                label=label,
                temperature=temperature
            )
        )

    return sensors

def cpuInfo():
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

    if sys.maxsize == 2 ** 64 - 1:
        arch = '64 bit'

    elif sys.maxsize == 2 ** 32 - 1:
        arch = '32 bit'

    byteOrder = ''
    if sys.byteorder == 'little':
        byteOrder = 'Little Endian'

    elif sys.byteorder == 'big':
        byteOrder = 'Big Endian'

    return Cpu(
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
    DRIVER_DIR = '/sys/devices/system/cpu/cpufreq'
    policies = []

    for dir in os.listdir(DRIVER_DIR):
        if 'policy' not in dir:
            continue

        policyName = dir
        scalingGovernor = ''

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