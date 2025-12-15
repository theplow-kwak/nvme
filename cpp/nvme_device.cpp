#include "nvme_device.h"

#include <iostream>
#include <vector>
#include <optional>
#include <stdexcept>
#include <system_error>

#include <SetupAPI.h>
#include <ntddstor.h>

#pragma comment(lib, "SetupAPI.lib")

namespace nvme
{
    // (Keep existing helpers: trim_string)
    static std::wstring trim_string(const char *buffer, size_t len)
    {
        std::string s(buffer, len);
        s.erase(s.find_last_not_of(" \n\r\t\0") + 1);
        s.erase(0, s.find_first_not_of(" \n\r\t\0"));
        return std::wstring(s.begin(), s.end());
    }

    // Helper to get identify controller data to retrieve firmware revision
    static std::optional<NVME_IDENTIFY_CONTROLLER_DATA> get_identify_controller_data(HANDLE hDevice)
    {
        STORAGE_PROTOCOL_SPECIFIC_DATA protocol_data = {};
        protocol_data.ProtocolType = ProtocolTypeNvme;
        protocol_data.DataType = NVMeDataTypeIdentify;
        protocol_data.ProtocolDataRequestValue = NVME_IDENTIFY_CNS_CONTROLLER;

        std::vector<uint8_t> buffer(sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR) + sizeof(NVME_IDENTIFY_CONTROLLER_DATA));
        auto *descriptor = reinterpret_cast<STORAGE_PROTOCOL_DATA_DESCRIPTOR *>(buffer.data());
        descriptor->Version = sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR);
        descriptor->Size = sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR);
        memcpy(&descriptor->ProtocolSpecificData, &protocol_data, sizeof(protocol_data));

        DWORD bytesReturned = 0;
        if (DeviceIoControl(hDevice, IOCTL_STORAGE_QUERY_PROPERTY, buffer.data(), buffer.size(), buffer.data(), buffer.size(), &bytesReturned, nullptr) && bytesReturned >= sizeof(NVME_IDENTIFY_CONTROLLER_DATA))
        {
            uint8_t *data = reinterpret_cast<uint8_t *>(descriptor) + descriptor->ProtocolSpecificData.ProtocolDataOffset;
            return *reinterpret_cast<NVME_IDENTIFY_CONTROLLER_DATA *>(data);
        }
        return std::nullopt;
    }

    // --- NvmeDeviceInfo & NvmeDeviceDiscovery Implementation (Unchanged) ---
    std::unique_ptr<NvmeDevice> NvmeDeviceInfo::create_device() const
    {
        return NvmeDevice::create(this->device_path);
    }

    bool NvmeDeviceDiscovery::enumerate_devices()
    {
        devices_.clear();
        HDEVINFO devInfo = SetupDiGetClassDevsW(&GUID_DEVINTERFACE_DISK, nullptr, nullptr, DIGCF_PRESENT | DIGCF_DEVICEINTERFACE);
        if (devInfo == INVALID_HANDLE_VALUE)
            return false;

        SP_DEVICE_INTERFACE_DATA interfaceData;
        interfaceData.cbSize = sizeof(SP_DEVICE_INTERFACE_DATA);

        for (DWORD i = 0; SetupDiEnumDeviceInterfaces(devInfo, nullptr, &GUID_DEVINTERFACE_DISK, i, &interfaceData); ++i)
        {
            DWORD detailSize = 0;
            SetupDiGetDeviceInterfaceDetailW(devInfo, &interfaceData, nullptr, 0, &detailSize, nullptr);
            if (detailSize == 0)
                continue;

            std::vector<uint8_t> detailBuffer(detailSize);
            auto *interfaceDetail = reinterpret_cast<SP_DEVICE_INTERFACE_DETAIL_DATA_W *>(detailBuffer.data());
            interfaceDetail->cbSize = sizeof(SP_DEVICE_INTERFACE_DETAIL_DATA_W);

            if (!SetupDiGetDeviceInterfaceDetailW(devInfo, &interfaceData, interfaceDetail, detailSize, nullptr, nullptr))
                continue;

            HANDLE hDevice = CreateFileW(interfaceDetail->DevicePath, 0, FILE_SHARE_READ | FILE_SHARE_WRITE, nullptr, OPEN_EXISTING, 0, nullptr);
            if (hDevice == INVALID_HANDLE_VALUE)
                continue;

            STORAGE_PROPERTY_QUERY query{};
            query.PropertyId = StorageDeviceProperty;
            query.QueryType = PropertyStandardQuery;

            std::vector<uint8_t> propBuffer(sizeof(STORAGE_DEVICE_DESCRIPTOR) + 512);
            DWORD bytesReturned = 0;

            if (DeviceIoControl(hDevice, IOCTL_STORAGE_QUERY_PROPERTY, &query, sizeof(query), propBuffer.data(), static_cast<DWORD>(propBuffer.size()), &bytesReturned, nullptr) && bytesReturned > 0)
            {
                auto *desc = reinterpret_cast<STORAGE_DEVICE_DESCRIPTOR *>(propBuffer.data());
                if (desc->BusType == BusTypeNvme)
                {
                    NvmeDeviceInfo info;
                    info.device_path = interfaceDetail->DevicePath;
                    STORAGE_DEVICE_NUMBER sdn = {0};
                    if (DeviceIoControl(hDevice, IOCTL_STORAGE_GET_DEVICE_NUMBER, NULL, 0, &sdn, sizeof(sdn), &bytesReturned, NULL))
                    {
                        info.physical_drive_number = sdn.DeviceNumber;
                    }
                    if (desc->ProductIdOffset > 0)
                        info.model_number = trim_string((char *)desc + desc->ProductIdOffset, 20);
                    if (desc->SerialNumberOffset > 0)
                        info.serial_number = trim_string((char *)desc + desc->SerialNumberOffset, 40);
                    if (auto identify_data = get_identify_controller_data(hDevice))
                    {
                        info.firmware_revision = trim_string(reinterpret_cast<const char *>(identify_data->FR), sizeof(identify_data->FR));
                    }

                    devices_.push_back(std::move(info));
                }
            }
            CloseHandle(hDevice);
        }
        SetupDiDestroyDeviceInfoList(devInfo);
        return true;
    }

    const std::vector<NvmeDeviceInfo> &NvmeDeviceDiscovery::get_devices() const { return devices_; }
    std::optional<NvmeDeviceInfo> NvmeDeviceDiscovery::find_by_drive_number(int number) const
    {
        for (const auto &device : devices_)
        {
            if (device.physical_drive_number == number)
                return device;
        }
        return std::nullopt;
    }

    // --- NvmeDevice Implementation (with new methods) ---
    NvmeDevice::NvmeDevice(std::wstring path) : path_(std::move(path)) { open(); }
    NvmeDevice::~NvmeDevice() { close(); }

    NvmeDevice::NvmeDevice(NvmeDevice &&other) noexcept
        : path_(std::move(other.path_)), device_handle_(other.device_handle_)
    {
        // Prevent the moved-from object's destructor from closing the handle
        other.device_handle_ = INVALID_HANDLE_VALUE;
    }

    NvmeDevice &NvmeDevice::operator=(NvmeDevice &&other) noexcept
    {
        if (this != &other)
        {
            // Close our own handle before taking ownership of the other's
            close();

            path_ = std::move(other.path_);
            device_handle_ = other.device_handle_;

            // Prevent the moved-from object's destructor from closing the handle
            other.device_handle_ = INVALID_HANDLE_VALUE;
        }
        return *this;
    }

    std::unique_ptr<NvmeDevice> NvmeDevice::create(const std::wstring &path)
    {
        auto device = std::unique_ptr<NvmeDevice>(new NvmeDevice(path));
        return (device && device->is_open()) ? std::move(device) : nullptr;
    }

    [[nodiscard]] bool NvmeDevice::is_open() const { return device_handle_ != INVALID_HANDLE_VALUE; }

    bool NvmeDevice::open()
    {
        if (is_open())
            return true;
        device_handle_ = CreateFileW(path_.c_str(), GENERIC_READ | GENERIC_WRITE, FILE_SHARE_READ | FILE_SHARE_WRITE, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);
        return device_handle_ != INVALID_HANDLE_VALUE;
    }
    void NvmeDevice::close()
    {
        if (is_open())
        {
            CloseHandle(device_handle_);
            device_handle_ = INVALID_HANDLE_VALUE;
        }
    }

    // High-level wrappers
    bool NvmeDevice::identify_controller_raw(std::vector<uint8_t> &buffer) const
    {
        return issue_identify_query(NVME_IDENTIFY_CNS_CONTROLLER, 0, buffer);
    }

    bool NvmeDevice::identify_namespace_raw(uint32_t nsid, std::vector<uint8_t> &buffer) const
    {
        return issue_identify_query(NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE, nsid, buffer);
    }

    std::optional<NVME_IDENTIFY_CONTROLLER_DATA> NvmeDevice::identify_controller_struct() const
    {
        std::vector<uint8_t> buffer(sizeof(NVME_IDENTIFY_CONTROLLER_DATA));
        if (identify_controller_raw(buffer) && buffer.size() >= sizeof(NVME_IDENTIFY_CONTROLLER_DATA))
        {
            return *reinterpret_cast<NVME_IDENTIFY_CONTROLLER_DATA *>(buffer.data());
        }
        return std::nullopt;
    }

    std::optional<NVME_IDENTIFY_NAMESPACE_DATA> NvmeDevice::identify_namespace_struct(uint32_t nsid) const
    {
        std::vector<uint8_t> buffer(sizeof(NVME_IDENTIFY_NAMESPACE_DATA));
        if (identify_namespace_raw(nsid, buffer) && buffer.size() >= sizeof(NVME_IDENTIFY_NAMESPACE_DATA))
        {
            return *reinterpret_cast<NVME_IDENTIFY_NAMESPACE_DATA *>(buffer.data());
        }
        return std::nullopt;
    }

    bool NvmeDevice::get_feature(uint8_t fid, uint8_t sel, uint32_t cdw11, uint32_t &value) const
    {
        NVME_CDW10_GET_FEATURES cdw10 = {};
        cdw10.FID = fid;
        cdw10.SEL = sel;
        NVME_CDW11_FEATURES cdw11_feat = {};
        cdw11_feat.AsUlong = cdw11;
        return issue_get_feature_query(cdw10, cdw11_feat, value);
    }

    bool NvmeDevice::set_feature(uint8_t fid, uint32_t value, uint32_t &result) const
    {
        NVME_CDW10_SET_FEATURES cdw10 = {};
        cdw10.FID = fid;
        NVME_CDW11_FEATURES cdw11 = {};
        cdw11.AsUlong = value;
        return issue_set_feature_query(cdw10, cdw11, result);
    }

    bool NvmeDevice::get_log_page(uint32_t nsid, uint8_t lid, std::vector<uint8_t> &buffer) const
    {
        if (buffer.empty())
            buffer.resize(NVME_MAX_LOG_SIZE);
        STORAGE_PROTOCOL_SPECIFIC_DATA protocol_data = {};
        protocol_data.ProtocolType = ProtocolTypeNvme;
        protocol_data.DataType = NVMeDataTypeLogPage;
        protocol_data.ProtocolDataRequestValue = lid;
        protocol_data.ProtocolDataRequestSubValue = nsid;
        protocol_data.ProtocolDataOffset = 0;
        protocol_data.ProtocolDataLength = static_cast<DWORD>(buffer.size());

        std::vector<uint8_t> output_buffer(sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR) + buffer.size());
        if (issue_query_property(StorageDeviceProtocolSpecificProperty, protocol_data, output_buffer))
        {
            auto *desc = reinterpret_cast<STORAGE_PROTOCOL_DATA_DESCRIPTOR *>(output_buffer.data());
            auto *data = reinterpret_cast<uint8_t *>(desc) + desc->ProtocolSpecificData.ProtocolDataOffset;
            memcpy(buffer.data(), data, buffer.size());
            return true;
        }
        return false;
    }

    std::optional<std::vector<uint32_t>> NvmeDevice::identify_ns_list(uint32_t nsid, bool all) const
    {
        NVME_COMMAND cmd = {};
        cmd.CDW0.OPC = NVME_ADMIN_COMMAND_IDENTIFY;
        cmd.NSID = nsid;
        cmd.u.IDENTIFY.CDW10.CNS = all ? NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_LIST : NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES;

        std::vector<uint8_t> buffer(4096, 0);
        uint32_t completion_dw0 = 0;
        if (send_vsc_admin_passthrough(cmd, buffer, completion_dw0))
        {
            std::vector<uint32_t> ns_list;
            for (size_t i = 0; i < 1024; ++i)
            {
                uint32_t id = *reinterpret_cast<uint32_t *>(buffer.data() + i * 4);
                if (id == 0)
                    break;
                ns_list.push_back(id);
            }
            return ns_list;
        }
        return std::nullopt;
    }

    // VSC Methods
    bool NvmeDevice::send_vsc2_passthrough(uint32_t sub_opcode, uint8_t direction, std::vector<uint8_t> &param_buf, std::vector<uint8_t> &data_buf, uint32_t &completion_dw0, uint32_t nsid) const
    {
        NVME_COMMAND nc = {};
        nc.CDW0.OPC = static_cast<uint8_t>(NvmeVscOpcode::Write);
        nc.NSID = nsid;
        nc.u.GENERAL.CDW10 = static_cast<uint32_t>(param_buf.size() / sizeof(uint32_t));
        nc.u.GENERAL.CDW12 = sub_opcode;

        uint16_t status_code = 0;
        bool result = issue_nvme_passthrough(nc, param_buf, false, completion_dw0, status_code);

        NVME_COMMAND_STATUS ncs;
        ncs.AsUshort = status_code;

        if (!result || (ncs.SCT != 0 && ncs.SC != 0) || direction == 0)
        {
            return result;
        }

        nc.CDW0.OPC = static_cast<uint8_t>(NvmeVscOpcode::None) | direction;
        nc.u.GENERAL.CDW10 = static_cast<uint32_t>(data_buf.size() / sizeof(uint32_t));
        nc.u.GENERAL.CDW12 = sub_opcode;
        nc.u.GENERAL.CDW14 = 1; // Data phase

        return issue_nvme_passthrough(nc, data_buf, direction == 2, completion_dw0, status_code);
    }

    bool NvmeDevice::send_vsc_admin_passthrough(const NVME_COMMAND &admin_cmd, std::vector<uint8_t> &data_buf, uint32_t &completion_dw0) const
    {
        uint8_t direction = admin_cmd.CDW0.OPC & 3;
        if (data_buf.empty())
            direction = 0;

        uint32_t sub_opcode;
        switch (direction)
        {
        case 0:
            sub_opcode = VS_STD_NVME_CMD_TYPE_NON_DATA;
            break;
        case 1:
            sub_opcode = VS_STD_NVME_CMD_TYPE_WRITE;
            break;
        case 2:
            sub_opcode = VS_STD_NVME_CMD_TYPE_READ;
            break;
        default:
            return false; // Not supported
        }

        std::vector<uint8_t> param_buffer(NVME_DATA_BUFFER_SIZE, 0);
        memcpy(param_buffer.data(), &admin_cmd, sizeof(NVME_COMMAND));

        return send_vsc2_passthrough(sub_opcode, direction, param_buffer, data_buf, completion_dw0, admin_cmd.NSID);
    }

    // Low-level IOCTL wrappers
    bool NvmeDevice::issue_identify_query(uint8_t cns, uint32_t nsid, std::vector<uint8_t> &buffer) const
    {
        if (buffer.size() < NVME_IDENTIFY_BUFFER_SIZE)
            buffer.resize(NVME_IDENTIFY_BUFFER_SIZE);
        STORAGE_PROTOCOL_SPECIFIC_DATA proto_data = {};
        proto_data.ProtocolType = ProtocolTypeNvme;
        proto_data.DataType = NVMeDataTypeIdentify;
        proto_data.ProtocolDataRequestValue = cns;
        proto_data.ProtocolDataRequestSubValue = nsid;
        proto_data.ProtocolDataOffset = 0;
        proto_data.ProtocolDataLength = static_cast<DWORD>(buffer.size());

        std::vector<uint8_t> output_buffer(sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR) + buffer.size());
        if (issue_query_property(nsid == 0 ? StorageAdapterProtocolSpecificProperty : StorageDeviceProtocolSpecificProperty, proto_data, output_buffer))
        {
            auto *desc = reinterpret_cast<STORAGE_PROTOCOL_DATA_DESCRIPTOR *>(output_buffer.data());
            auto *data = reinterpret_cast<uint8_t *>(desc) + desc->ProtocolSpecificData.ProtocolDataOffset;
            memcpy(buffer.data(), data, buffer.size());
            return true;
        }
        return false;
    }

    bool NvmeDevice::issue_get_feature_query(NVME_CDW10_GET_FEATURES cdw10, NVME_CDW11_FEATURES cdw11, uint32_t &value) const
    {
        STORAGE_PROTOCOL_SPECIFIC_DATA proto_data = {};
        proto_data.ProtocolType = ProtocolTypeNvme;
        proto_data.DataType = NVMeDataTypeFeature;
        proto_data.ProtocolDataRequestValue = cdw10.AsUlong;
        proto_data.ProtocolDataRequestSubValue = cdw11.AsUlong;

        std::vector<uint8_t> output_buffer(sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR));
        if (issue_query_property(StorageAdapterProtocolSpecificProperty, proto_data, output_buffer))
        {
            value = proto_data.FixedProtocolReturnData;
            return true;
        }
        return false;
    }

    bool NvmeDevice::issue_set_feature_query(NVME_CDW10_SET_FEATURES cdw10, NVME_CDW11_FEATURES cdw11, uint32_t &result) const
    {
        STORAGE_PROTOCOL_SPECIFIC_DATA proto_data = {};
        proto_data.ProtocolType = ProtocolTypeNvme;
        proto_data.DataType = NVMeDataTypeFeature;
        proto_data.ProtocolDataRequestValue = cdw10.AsUlong;
        proto_data.ProtocolDataRequestSubValue = cdw11.AsUlong;

        if (issue_set_property(StorageAdapterProtocolSpecificProperty, proto_data))
        {
            result = proto_data.FixedProtocolReturnData;
            return true;
        }
        return false;
    }

    bool NvmeDevice::issue_query_property(STORAGE_PROPERTY_ID property_id, STORAGE_PROTOCOL_SPECIFIC_DATA &protocol_data, std::vector<uint8_t> &output_buffer) const
    {
        std::vector<uint8_t> query_buffer(offsetof(STORAGE_PROPERTY_QUERY, AdditionalParameters) + sizeof(STORAGE_PROTOCOL_SPECIFIC_DATA));
        auto *query = reinterpret_cast<STORAGE_PROPERTY_QUERY *>(query_buffer.data());
        query->PropertyId = property_id;
        query->QueryType = PropertyStandardQuery;
        memcpy(query->AdditionalParameters, &protocol_data, sizeof(STORAGE_PROTOCOL_SPECIFIC_DATA));

        DWORD returned_length = 0;
        if (!DeviceIoControl(device_handle_, IOCTL_STORAGE_QUERY_PROPERTY, query_buffer.data(), static_cast<DWORD>(query_buffer.size()), output_buffer.data(), static_cast<DWORD>(output_buffer.size()), &returned_length, nullptr))
        {
            return false;
        }
        auto *desc = reinterpret_cast<STORAGE_PROTOCOL_DATA_DESCRIPTOR *>(output_buffer.data());
        if (desc->Version != sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR) || desc->Size != sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR))
            return false;
        memcpy(&protocol_data, &desc->ProtocolSpecificData, sizeof(STORAGE_PROTOCOL_SPECIFIC_DATA));
        return true;
    }

    bool NvmeDevice::issue_set_property(STORAGE_PROPERTY_ID property_id, const STORAGE_PROTOCOL_SPECIFIC_DATA &protocol_data) const
    {
        std::vector<uint8_t> set_buffer(offsetof(STORAGE_PROPERTY_SET, AdditionalParameters) + sizeof(STORAGE_PROTOCOL_SPECIFIC_DATA));
        auto *property_set = reinterpret_cast<STORAGE_PROPERTY_SET *>(set_buffer.data());
        property_set->PropertyId = property_id;
        property_set->SetType = PropertyStandardSet;
        memcpy(property_set->AdditionalParameters, &protocol_data, sizeof(STORAGE_PROTOCOL_SPECIFIC_DATA));

        DWORD returned_length = 0;
        return DeviceIoControl(device_handle_, IOCTL_STORAGE_SET_PROPERTY, set_buffer.data(), static_cast<DWORD>(set_buffer.size()), nullptr, 0, &returned_length, nullptr);
    }

    // Raw passthrough command
    bool NvmeDevice::issue_nvme_passthrough(const NVME_COMMAND &nvme_cmd, std::vector<uint8_t> &data_buffer, bool is_read_command, uint32_t &completion_dw0, uint16_t &status_code) const
    {
        return issue_protocol_command(nvme_cmd, data_buffer.data(), static_cast<DWORD>(data_buffer.size()), is_read_command, completion_dw0, status_code);
    }

    bool NvmeDevice::issue_protocol_command(const NVME_COMMAND &nvme_cmd, void *data_buffer, DWORD data_buffer_size, bool is_read_command, uint32_t &completion_dw0, uint16_t &status_code) const
    {
        if (!is_open())
            return false;
        std::vector<uint8_t> buffer(sizeof(STORAGE_PROTOCOL_COMMAND) + data_buffer_size);
        auto *cmd = reinterpret_cast<STORAGE_PROTOCOL_COMMAND *>(buffer.data());

        cmd->Version = STORAGE_PROTOCOL_STRUCTURE_VERSION;
        cmd->Length = sizeof(STORAGE_PROTOCOL_COMMAND);
        cmd->ProtocolType = ProtocolTypeNvme;
        cmd->Flags = is_read_command ? STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST : 0;
        cmd->CommandLength = STORAGE_PROTOCOL_COMMAND_LENGTH_NVME;
        cmd->ErrorInfoLength = sizeof(NVME_ERROR_INFO_LOG);
        cmd->TimeOutValue = 10;
        cmd->CommandSpecific = STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND;
        cmd->DataFromDeviceBufferOffset = is_read_command ? sizeof(STORAGE_PROTOCOL_COMMAND) : 0;
        cmd->DataToDeviceBufferOffset = is_read_command ? 0 : sizeof(STORAGE_PROTOCOL_COMMAND);
        cmd->DataFromDeviceTransferLength = is_read_command ? data_buffer_size : 0;
        cmd->DataToDeviceTransferLength = is_read_command ? 0 : data_buffer_size;

        memcpy(cmd->Command, &nvme_cmd, sizeof(NVME_COMMAND));
        if (!is_read_command && data_buffer_size > 0)
        {
            memcpy(buffer.data() + cmd->DataToDeviceBufferOffset, data_buffer, data_buffer_size);
        }

        DWORD bytesReturned = 0;
        if (!DeviceIoControl(device_handle_, IOCTL_STORAGE_PROTOCOL_COMMAND, buffer.data(), static_cast<DWORD>(buffer.size()), buffer.data(), static_cast<DWORD>(buffer.size()), &bytesReturned, nullptr))
        {
            return false;
        }

        if (cmd->ReturnStatus != STORAGE_PROTOCOL_STATUS_SUCCESS)
        {
            auto *error_log = reinterpret_cast<NVME_ERROR_INFO_LOG *>(buffer.data() + cmd->ErrorInfoOffset);
            status_code = error_log->Status.AsUshort;
            return false;
        }
        if (is_read_command && data_buffer_size > 0)
        {
            memcpy(data_buffer, buffer.data() + cmd->DataFromDeviceBufferOffset, data_buffer_size);
        }

        completion_dw0 = cmd->FixedProtocolReturnData;
        status_code = 0;
        return true;
    }
}