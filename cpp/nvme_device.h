#pragma once

#include "nvme_define.h"
#include <string>
#include <vector>
#include <memory>

namespace nvme {

// Forward declaration
class NvmeDevice; 

// Holds basic information about a discovered NVMe device
struct NvmeDeviceInfo {
    int physical_drive_number = -1;
    std::wstring device_path;
    std::wstring model_number;
    std::wstring serial_number;
    std::wstring firmware_revision;

    // Factory method to create an NvmeDevice from this info
    std::unique_ptr<NvmeDevice> create_device() const;
};

// Handles discovery of NVMe devices on the system
class NvmeDeviceDiscovery {
public:
    NvmeDeviceDiscovery() = default;

    // Finds all physical drives and filters for NVMe devices
    bool enumerate_devices();

    const std::vector<NvmeDeviceInfo>& get_devices() const;

    // Find a device by its physical drive number (e.g., 1 for PhysicalDrive1)
    std::optional<NvmeDeviceInfo> find_by_drive_number(int number) const;

private:
    std::vector<NvmeDeviceInfo> devices_;
};


// Represents an opened NVMe device and provides methods for sending commands
class NvmeDevice {
public:
    // Deleted copy constructor and assignment operator
    NvmeDevice(const NvmeDevice&) = delete;
    NvmeDevice& operator=(const NvmeDevice&) = delete;

    ~NvmeDevice();

    static std::unique_ptr<NvmeDevice> create(const std::wstring& path);

    bool is_open() const;
    
    // Wrappers for NVMe Admin Commands, mirroring the Rust implementation
    bool nvme_identify_controller(std::vector<uint8_t>& buffer);
    bool nvme_identify_namespace(uint32_t nsid, std::vector<uint8_t>& buffer);
    bool nvme_identify_query(NVME_CDW10_IDENTIFY cdw10, std::vector<uint8_t>& buffer);
    bool nvme_get_feature(NVME_CDW10_GET_FEATURES cdw10, uint32_t& value);
    bool nvme_set_feature(uint32_t fid, uint32_t value, uint32_t& result);
    bool nvme_logpage_query(uint32_t lid, std::vector<uint8_t>& buffer);

private:
    // Private constructor, use factory method NvmeDevice::create
    NvmeDevice(std::wstring path);

    bool open();
    void close();

    // The core function to send a pass-through command to the device
    bool issue_protocol_command(
        STORAGE_PROTOCOL_COMMAND& command,
        void* buffer,
        DWORD buffer_size
    );

    std::wstring path_;
    HANDLE device_handle_ = INVALID_HANDLE_VALUE;
};

} // namespace nvme
