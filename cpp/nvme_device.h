#pragma once

#include <string>
#include <vector>
#include <memory>
#include <optional>
#include <windows.h>
#include <winioctl.h>
#include <nvme.h>

namespace nvme
{
    // Enums and Constants from nvme_commands.rs
    constexpr size_t NVME_IDENTIFY_BUFFER_SIZE = 4096;
    constexpr size_t NVME_DATA_BUFFER_SIZE = 4096;
    constexpr uint32_t VS_STD_NVME_CMD_TYPE_READ = 0x83061400;
    constexpr uint32_t VS_STD_NVME_CMD_TYPE_WRITE = 0x83061401;
    constexpr uint32_t VS_STD_NVME_CMD_TYPE_NON_DATA = 0x83061402;

    enum class NvmeOpcodeType : uint8_t
    {
        NOBUFFER,
        WRITE,
        READ,
        READWRITE,
    };

    enum class NvmeVscOpcode : uint8_t
    {
        None = 0xf0,
        Write = 0xf1,
        Read = 0xf2,
    };

    // Forward declaration
    class NvmeDevice;

    // Holds basic information about a discovered NVMe device
    struct NvmeDeviceInfo
    {
        int physical_drive_number = -1;
        std::wstring device_path;
        std::wstring model_number;
        std::wstring serial_number;
        std::wstring firmware_revision;

        // Factory method to create an NvmeDevice from this info
        std::unique_ptr<NvmeDevice> create_device() const;
    };

    // Handles discovery of NVMe devices on the system
    class NvmeDeviceDiscovery
    {
    public:
        NvmeDeviceDiscovery() = default;
        bool enumerate_devices();
        const std::vector<NvmeDeviceInfo> &get_devices() const;
        std::optional<NvmeDeviceInfo> find_by_drive_number(int number) const;

    private:
        std::vector<NvmeDeviceInfo> devices_;
    };

    // Represents an opened NVMe device and provides methods for sending commands
    class NvmeDevice
    {
    public:
        NvmeDevice(const NvmeDevice &) = delete;
        NvmeDevice(NvmeDevice &&other) noexcept;
        NvmeDevice &operator=(NvmeDevice &&other) noexcept;
        NvmeDevice &operator=(const NvmeDevice &) = delete;
        ~NvmeDevice();

        static std::unique_ptr<NvmeDevice> create(const std::wstring &path);
        bool is_open() const;

        // --- High-level command wrappers ---
        bool identify_controller_raw(std::vector<uint8_t> &buffer) const;
        bool identify_namespace_raw(uint32_t nsid, std::vector<uint8_t> &buffer) const;

        // Deserializing versions
        std::optional<NVME_IDENTIFY_CONTROLLER_DATA> identify_controller() const;
        std::optional<NVME_IDENTIFY_NAMESPACE_DATA> identify_namespace(uint32_t nsid) const;

        // User-friendly Get/Set Feature
        bool get_feature(uint8_t fid, uint8_t sel, uint32_t cdw11, uint32_t &value) const;
        bool set_feature(uint8_t fid, uint32_t value, uint32_t &result) const;

        bool get_log_page(uint32_t nsid, uint8_t lid, std::vector<uint8_t> &buffer) const;

        // Namespace list identification
        std::optional<std::vector<uint32_t>> identify_ns_list(uint32_t nsid, bool all) const;

        // --- Vendor-Specific Commands ---
        bool send_vsc2_passthrough(
            uint32_t sub_opcode,
            uint8_t direction,
            std::vector<uint8_t> &param_buf,
            std::vector<uint8_t> &data_buf,
            uint32_t &completion_dw0,
            uint32_t nsid) const;

        bool send_vsc_admin_passthrough(
            const NVME_COMMAND &admin_cmd,
            std::vector<uint8_t> &data_buf,
            uint32_t &completion_dw0) const;

        // --- Raw Passthrough command ---
        bool issue_nvme_passthrough(
            const NVME_COMMAND &nvme_cmd,
            std::vector<uint8_t> &data_buffer,
            bool is_read_command,
            uint32_t &completion_dw0,
            uint16_t &status_code) const;

    private:
        NvmeDevice(std::wstring path);

        bool open();
        void close();

        // Low-level wrappers from previous step (renamed identify for clarity)
        bool issue_identify_query(uint8_t cns, uint32_t nsid, std::vector<uint8_t> &buffer) const;
        bool issue_get_feature_query(NVME_CDW10_GET_FEATURES cdw10, NVME_CDW11_FEATURES cdw11, uint32_t &value) const;
        bool issue_set_feature_query(NVME_CDW10_SET_FEATURES cdw10, NVME_CDW11_FEATURES cdw11, uint32_t &result) const;

        bool issue_query_property(
            STORAGE_PROPERTY_ID property_id,
            STORAGE_PROTOCOL_SPECIFIC_DATA &protocol_data,
            std::vector<uint8_t> &output_buffer) const;

        bool issue_set_property(
            STORAGE_PROPERTY_ID property_id,
            STORAGE_PROTOCOL_SPECIFIC_DATA &protocol_data) const;

        bool issue_protocol_command(
            const NVME_COMMAND &nvme_cmd,
            void *data_buffer,
            DWORD data_buffer_size,
            bool is_read_command,
            uint32_t &completion_dw0,
            uint16_t &status_code) const;

        std::wstring path_;
        HANDLE device_handle_ = INVALID_HANDLE_VALUE;
    };

} // namespace nvme
