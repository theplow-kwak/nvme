#include "nvme_device.h"
#include <iostream>
#include <optional>
#include <vector>
#include <SetupAPI.h>
#include <ntddstor.h>

// Link against SetupAPI.lib
#pragma comment(lib, "SetupAPI.lib")

namespace nvme {

// Helper function to trim trailing nulls from C-style strings in buffers
static std::wstring trim_string(const char* buffer, size_t len) {
    std::string s(buffer, len);
    s.erase(s.find_last_not_of(" \n\r\t") + 1);
    s.erase(0, s.find_first_not_of(" \n\r\t"));
    return std::wstring(s.begin(), s.end());
}


// --- NvmeDeviceInfo Implementation ---

std::unique_ptr<NvmeDevice> NvmeDeviceInfo::create_device() const {
    if (this->device_path.empty()) {
        return nullptr;
    }
    return NvmeDevice::create(this->device_path);
}


// --- NvmeDeviceDiscovery Implementation ---

bool NvmeDeviceDiscovery::enumerate_devices() {
    devices_.clear();

    HDEVINFO devInfo = SetupDiGetClassDevsW(&GUID_DEVINTERFACE_DISK, nullptr, nullptr, DIGCF_PRESENT | DIGCF_DEVICEINTERFACE);
    if (devInfo == INVALID_HANDLE_VALUE) {
        std::wcerr << L"Failed to get device info set. Error: " << GetLastError() << std::endl;
        return false;
    }

    SP_DEVICE_INTERFACE_DATA interfaceData;
    interfaceData.cbSize = sizeof(SP_DEVICE_INTERFACE_DATA);

    for (DWORD i = 0; SetupDiEnumDeviceInterfaces(devInfo, nullptr, &GUID_DEVINTERFACE_DISK, i, &interfaceData); ++i) {
        DWORD detailSize = 0;
        SetupDiGetDeviceInterfaceDetailW(devInfo, &interfaceData, nullptr, 0, &detailSize, nullptr);

        if (detailSize == 0) continue;

        std::vector<uint8_t> detailBuffer(detailSize);
        auto* interfaceDetail = reinterpret_cast<SP_DEVICE_INTERFACE_DETAIL_DATA_W*>(detailBuffer.data());
        interfaceDetail->cbSize = sizeof(SP_DEVICE_INTERFACE_DETAIL_DATA_W);

        if (!SetupDiGetDeviceInterfaceDetailW(devInfo, &interfaceData, interfaceDetail, detailSize, nullptr, nullptr)) {
            continue;
        }

        std::wstring devicePath = interfaceDetail->DevicePath;
        HANDLE hDevice = CreateFileW(devicePath.c_str(), 0, FILE_SHARE_READ | FILE_SHARE_WRITE, nullptr, OPEN_EXISTING, 0, nullptr);

        if (hDevice == INVALID_HANDLE_VALUE) {
            continue;
        }

        STORAGE_PROPERTY_QUERY query{};
        query.PropertyId = StorageDeviceProperty;
        query.QueryType = PropertyStandardQuery;

        std::vector<uint8_t> propertyBuffer(sizeof(STORAGE_DEVICE_DESCRIPTOR) + 512);
        DWORD bytesReturned = 0;

        if (DeviceIoControl(hDevice, IOCTL_STORAGE_QUERY_PROPERTY, &query, sizeof(query), propertyBuffer.data(), propertyBuffer.size(), &bytesReturned, nullptr) && bytesReturned > 0) {
            auto* desc = reinterpret_cast<STORAGE_DEVICE_DESCRIPTOR*>(propertyBuffer.data());
            if (desc->BusType == BusTypeNvme) {
                NvmeDeviceInfo info;
                info.device_path = devicePath;

                STORAGE_DEVICE_NUMBER sdn = { 0 };
                if (DeviceIoControl(hDevice, IOCTL_STORAGE_GET_DEVICE_NUMBER, NULL, 0, &sdn, sizeof(sdn), &bytesReturned, NULL)) {
                    info.physical_drive_number = sdn.DeviceNumber;
                } else {
                    info.physical_drive_number = -1;
                }

                if (desc->ProductIdOffset > 0) {
                    info.model_number = trim_string((char*)desc + desc->ProductIdOffset, 20);
                }
                if (desc->SerialNumberOffset > 0) {
                    info.serial_number = trim_string((char*)desc + desc->SerialNumberOffset, 40);
                }
                
                devices_.push_back(std::move(info));
            }
        }
        CloseHandle(hDevice);
    }

    SetupDiDestroyDeviceInfoList(devInfo);
    return true;
}

const std::vector<NvmeDeviceInfo>& NvmeDeviceDiscovery::get_devices() const {
    return devices_;
}

std::optional<NvmeDeviceInfo> NvmeDeviceDiscovery::find_by_drive_number(int number) const {
    for (const auto& device : devices_) {
        if (device.physical_drive_number == number) {
            return device;
        }
    }
    // Fallback to build path manually if not found by interface
    try {
        std::wstring path = L"\\\\.\\PhysicalDrive" + std::to_wstring(number);
        auto dev = NvmeDevice::create(path);
        if (dev && dev->is_open()) {
            NvmeDeviceInfo info;
            info.physical_drive_number = number;
            info.device_path = path;
            return info;
        }
    } catch (...) {}
    return std::nullopt;
}


// --- NvmeDevice Implementation ---

NvmeDevice::NvmeDevice(std::wstring path) : path_(std::move(path)) {
    open();
}

NvmeDevice::~NvmeDevice() {
    close();
}

std::unique_ptr<NvmeDevice> NvmeDevice::create(const std::wstring& path) {
    // Using `new` because make_unique can't access private constructor
    auto device = std::unique_ptr<NvmeDevice>(new NvmeDevice(path));
    if (!device->is_open()) {
        return nullptr;
    }
    return device;
}

bool NvmeDevice::is_open() const {
    return device_handle_ != INVALID_HANDLE_VALUE;
}

bool NvmeDevice::open() {
    if (is_open()) {
        return true;
    }
    device_handle_ = CreateFileW(path_.c_str(),
        GENERIC_READ | GENERIC_WRITE,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        nullptr,
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        nullptr);

    if (!is_open()) {
        // Suppress error message for enumeration purposes
        // std::wcerr << L"Failed to open device " << path_ << ". Error: " << GetLastError() << std::endl;
        return false;
    }
    return true;
}

void NvmeDevice::close() {
    if (is_open()) {
        CloseHandle(device_handle_);
        device_handle_ = INVALID_HANDLE_VALUE;
    }
}

bool NvmeDevice::issue_protocol_command(
    NVME_COMMAND& nvme_cmd,
    void* data_buffer,
    DWORD data_buffer_size,
    bool is_read_command,
    uint32_t& completion_dw0
) {
    if (!is_open()) return false;

    const DWORD buffer_size = sizeof(STORAGE_PROTOCOL_COMMAND) + data_buffer_size;
    std::vector<uint8_t> buffer(buffer_size, 0);

    auto* protocol_command = reinterpret_cast<STORAGE_PROTOCOL_COMMAND*>(buffer.data());

    protocol_command->Version = STORAGE_PROTOCOL_VERSION;
    protocol_command->Length = sizeof(STORAGE_PROTOCOL_COMMAND);
    protocol_command->ProtocolType = ProtocolTypeNvme;
    protocol_command->AtaPath.Flags = is_read_command ? STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST : 0;
    protocol_command->TransferLength = data_buffer_size;
    protocol_command->ProtocolDataOffset = sizeof(STORAGE_PROTOCOL_COMMAND);
    protocol_command->ProtocolDataLength = data_buffer_size;

    memcpy(&protocol_command->AtaPath.Command.Nvme.Command, &nvme_cmd, sizeof(NVME_COMMAND));

    if (!is_read_command && data_buffer && data_buffer_size > 0) {
        memcpy(buffer.data() + sizeof(STORAGE_PROTOCOL_COMMAND), data_buffer, data_buffer_size);
    }
    
    DWORD bytesReturned = 0;
    if (!DeviceIoControl(
        device_handle_,
        IOCTL_STORAGE_PROTOCOL_COMMAND,
        buffer.data(),
        buffer_size,
        buffer.data(),
        buffer_size,
        &bytesReturned,
        nullptr
    )) {
        std::wcerr << L"DeviceIoControl failed. Error: " << GetLastError() << std::endl;
        return false;
    }

    if (protocol_command->ReturnStatus != STORAGE_PROTOCOL_STATUS_SUCCESS) {
        std::wcerr << L"NVMe command failed. Return Status: " << protocol_command->ReturnStatus << std::endl;
        return false;
    }

    completion_dw0 = protocol_command->AtaPath.Command.Nvme.Completion.DW0;

    if (is_read_command && data_buffer && data_buffer_size > 0) {
        memcpy(data_buffer, buffer.data() + sizeof(STORAGE_PROTOCOL_COMMAND), data_buffer_size);
    }

    return true;
}


bool NvmeDevice::nvme_identify_controller(std::vector<uint8_t>& buffer) {
    NVME_CDW10_IDENTIFY cdw10 = {};
    cdw10.CNS = static_cast<uint8_t>(IdentifyCnsCode::Controller);
    return nvme_identify_query(cdw10, buffer, 0);
}

bool NvmeDevice::nvme_identify_namespace(uint32_t nsid, std::vector<uint8_t>& buffer) {
    NVME_CDW10_IDENTIFY cdw10 = {};
    cdw10.CNS = static_cast<uint8_t>(IdentifyCnsCode::SpecificNamespace);
    return nvme_identify_query(cdw10, buffer, nsid);
}

bool NvmeDevice::nvme_identify_query(NVME_CDW10_IDENTIFY cdw10, std::vector<uint8_t>& buffer, uint32_t nsid) {
    if (buffer.size() < NVME_IDENTIFY_BUFFER_SIZE) {
        buffer.resize(NVME_IDENTIFY_BUFFER_SIZE);
    }

    NVME_COMMAND nvme_cmd = {};
    nvme_cmd.CDW0.OPC = NVME_ADMIN_COMMAND_IDENTIFY;
    nvme_cmd.NSID = nsid;
    nvme_cmd.CDW10 = *reinterpret_cast<uint32_t*>(&cdw10);
    
    uint32_t completion_dw0;
    return issue_protocol_command(nvme_cmd, buffer.data(), buffer.size(), true, completion_dw0);
}

bool NvmeDevice::nvme_get_feature(NVME_CDW10_GET_FEATURES cdw10, uint32_t& value) {
    NVME_COMMAND nvme_cmd = {};
    nvme_cmd.CDW0.OPC = NVME_ADMIN_COMMAND_GET_FEATURES;
    nvme_cmd.CDW10 = *reinterpret_cast<uint32_t*>(&cdw10);

    return issue_protocol_command(nvme_cmd, nullptr, 0, false, value);
}

bool NvmeDevice::nvme_set_feature(uint32_t fid, uint32_t value, uint32_t& result) {
    NVME_COMMAND nvme_cmd = {};
    nvme_cmd.CDW0.OPC = NVME_ADMIN_COMMAND_SET_FEATURES;
    
    NVME_CDW10_SET_FEATURES cdw10 = {};
    cdw10.FID = fid;
    nvme_cmd.CDW10 = *reinterpret_cast<uint32_t*>(&cdw10);
    nvme_cmd.CDW11 = value;

    return issue_protocol_command(nvme_cmd, nullptr, 0, false, result);
}

bool NvmeDevice::nvme_logpage_query(uint32_t lid, std::vector<uint8_t>& buffer) {
    if (buffer.size() == 0) {
        buffer.resize(NVME_MAX_LOG_SIZE);
    }

    NVME_COMMAND nvme_cmd = {};
    nvme_cmd.CDW0.OPC = NVME_ADMIN_COMMAND_GET_LOG_PAGE;
    
    uint32_t numd = (static_cast<uint32_t>(buffer.size()) / 4) - 1;
    nvme_cmd.CDW10 = (numd << 16) | lid;

    uint32_t completion_dw0;
    return issue_protocol_command(nvme_cmd, buffer.data(), buffer.size(), true, completion_dw0);
}

} // namespace nvme
