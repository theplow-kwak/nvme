#include "dev_utils.h"
#include <windows.h>
#include <cfgmgr32.h>
#include <devpkey.h>
#include <cstdio>
#include <algorithm>
#include <mutex>
#include <vector>
#include <system_error>

#pragma comment(lib, "Cfgmgr32.lib")

namespace
{
    // Helper to convert wstring to string
    std::string to_string(const std::wstring &wstr)
    {
        if (wstr.empty())
            return std::string();
        int size_needed = WideCharToMultiByte(CP_UTF8, 0, &wstr[0], (int)wstr.size(), NULL, 0, NULL, NULL);
        std::string strTo(size_needed, 0);
        WideCharToMultiByte(CP_UTF8, 0, &wstr[0], (int)wstr.size(), &strTo[0], size_needed, NULL, NULL);
        return strTo;
    }
}

namespace dev_utils
{

    // --- PciBdf Implementation ---
    std::optional<PciBdf> PciBdf::parse(const std::string &location_info)
    {
        PciBdf bdf;
        // "PCI bus 1, device 0, function 0"
        int matches = sscanf_s(location_info.c_str(), "PCI bus %d, device %d, function %d", &bdf.bus, &bdf.device, &bdf.function);
        if (matches == 3)
        {
            // Segment is not usually in this string, default to 0
            bdf.segment = 0;
            return bdf;
        }
        return std::nullopt;
    }

    std::string PciBdf::to_string() const
    {
        char buffer[20];
        snprintf(buffer, sizeof(buffer), "%04X:%02X:%02X.%X", segment, bus, device, function);
        return std::string(buffer);
    }

    std::ostream &operator<<(std::ostream &os, const PciBdf &bdf)
    {
        os << bdf.to_string();
        return os;
    }

    // --- DevInstance Implementation ---
    std::optional<DevInstance> DevInstance::create(DEVINST devinst)
    {
        ULONG status = 0, problem = 0;
        if (CM_Get_DevNode_Status(&status, &problem, devinst, 0) == CR_SUCCESS)
        {
            return DevInstance(devinst, status, problem);
        }
        return std::nullopt;
    }

    DevInstance::DevInstance(DEVINST devinst, ULONG status, ULONG problem)
        : devinst_(devinst), status_(status), problem_(problem) {}

    std::optional<std::wstring> DevInstance::get_property_string(const DEVPROPKEY &prop_key) const
    {
        DEVPROPTYPE prop_type;
        ULONG prop_size = 0;
        if (CM_Get_DevNode_PropertyW(devinst_, &prop_key, &prop_type, nullptr, &prop_size, 0) != CR_BUFFER_SMALL)
        {
            return std::nullopt;
        }
        if (prop_type != DEVPROP_TYPE_STRING)
        {
            return std::nullopt;
        }

        std::vector<wchar_t> buffer(prop_size / sizeof(wchar_t));
        if (CM_Get_DevNode_PropertyW(devinst_, &prop_key, &prop_type, (PBYTE)buffer.data(), &prop_size, 0) == CR_SUCCESS)
        {
            return std::wstring(buffer.data());
        }
        return std::nullopt;
    }

    std::optional<std::string> DevInstance::service() const
    {
        if (auto prop = get_property_string(DEVPKEY_Device_Service))
        {
            return to_string(*prop);
        }
        return std::nullopt;
    }

    std::optional<std::string> DevInstance::location_info() const
    {
        if (auto prop = get_property_string(DEVPKEY_Device_LocationInfo))
        {
            return to_string(*prop);
        }
        return std::nullopt;
    }

    std::optional<PciBdf> DevInstance::pcibdf() const
    {
        if (auto loc_info = location_info())
        {
            return PciBdf::parse(*loc_info);
        }
        return std::nullopt;
    }

    std::optional<std::wstring> DevInstance::instance_id() const
    {
        return get_property_string(DEVPKEY_Device_InstanceId);
    }

    bool DevInstance::enable()
    {
        return CM_Enable_DevNode(devinst_, 0) == CR_SUCCESS;
    }

    bool DevInstance::disable()
    {
        return CM_Disable_DevNode(devinst_, CM_DISABLE_HARDWARE | CM_DISABLE_UI_NOT_OK) == CR_SUCCESS;
    }

    bool DevInstance::remove()
    {
        return CM_Query_And_Remove_SubTreeA(devinst_, nullptr, nullptr, 0, CM_REMOVE_NO_RESTART) == CR_SUCCESS;
    }

    bool DevInstance::restart()
    {
        return CM_Setup_DevNode(devinst_, CM_SETUP_DEVNODE_READY) == CR_SUCCESS;
    }

    bool DevInstance::refresh()
    {
        DEVINST root_devinst = 0;
        if (CM_Locate_DevNodeA(&root_devinst, nullptr, CM_LOCATE_DEVNODE_NORMAL) == CR_SUCCESS)
        {
            return CM_Reenumerate_DevNode(root_devinst, 0) == CR_SUCCESS;
        }
        return false;
    }

    std::optional<DevInstance> DevInstance::parent() const
    {
        DEVINST parent_inst;
        if (CM_Get_Parent(&parent_inst, devinst_, 0) == CR_SUCCESS)
        {
            return DevInstance::create(parent_inst);
        }
        return std::nullopt;
    }

    DEVINST DevInstance::handle() const
    {
        return devinst_;
    }

    std::ostream &operator<<(std::ostream &os, const DevInstance &dev)
    {
        os << "DevInst(" << dev.handle() << ")";
        return os;
    }

    // --- LogicalDrive Implementation ---
    namespace
    {
        static std::mutex g_logical_drive_cache_mutex;
        static std::vector<std::pair<int, std::string>> g_logical_drive_cache;
    }

    class LogicalDrive
    {
    public:
        static void enumerate()
        {
            std::lock_guard<std::mutex> lock(g_logical_drive_cache_mutex);
            if (!g_logical_drive_cache.empty())
            {
                return;
            }

            std::vector<char> buffer(1024);
            if (GetLogicalDriveStringsA(static_cast<DWORD>(buffer.size()), buffer.data()) > 0)
            {
                for (const char *p = buffer.data(); *p; p += strlen(p) + 1)
                {
                    std::string drive_path = p;
                    if (!drive_path.empty() && drive_path.back() == '\\')
                    {
                        drive_path.pop_back();
                    }
                    int disk_no = disk::get_physical_drv_number_from_logical_drv(drive_path);
                    g_logical_drive_cache.emplace_back(disk_no, drive_path);
                }
            }
        }

        static std::vector<std::string> get_drives(int number)
        {
            enumerate();
            std::lock_guard<std::mutex> lock(g_logical_drive_cache_mutex);
            std::vector<std::string> drives;
            for (const auto &pair : g_logical_drive_cache)
            {
                if (number >= 0 && pair.first == number)
                {
                    drives.push_back(pair.second);
                }
            }
            return drives;
        }
    };

    // --- PhysicalDisk Implementation ---
    std::optional<PhysicalDisk> PhysicalDisk::create(DEVINST devinst)
    {
        if (auto di = DevInstance::create(devinst))
        {
            return PhysicalDisk(*di);
        }
        return std::nullopt;
    }

    PhysicalDisk::PhysicalDisk(DevInstance devinst) : devinst_(devinst) {}

    void PhysicalDisk::inspect()
    {
        get_interface_path();
        get_device_number();
        get_nsid();
        enum_child_volumes();
        open_driver();
    }

    void PhysicalDisk::get_interface_path()
    {
        if (auto inst_id = devinst_.instance_id())
        {
            ULONG iface_list_size = 0;
            if (CM_Get_Device_Interface_List_SizeW(&iface_list_size, (GUID *)&GUID_DEVINTERFACE_DISK, (DEVINSTID_W)inst_id->c_str(), CM_GET_DEVICE_INTERFACE_LIST_PRESENT) == CR_SUCCESS)
            {
                std::vector<wchar_t> iface_list(iface_list_size);
                if (CM_Get_Device_Interface_ListW((GUID *)&GUID_DEVINTERFACE_DISK, (DEVINSTID_W)inst_id->c_str(), iface_list.data(), iface_list_size, CM_GET_DEVICE_INTERFACE_LIST_PRESENT) == CR_SUCCESS)
                {
                    interface_path_ = to_string(iface_list.data());
                }
            }
        }
    }

    void PhysicalDisk::get_device_number()
    {
        if (interface_path_.empty())
            return;

        HANDLE handle = CreateFileA(interface_path_.c_str(), GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE, NULL, OPEN_EXISTING, 0, NULL);
        if (handle == INVALID_HANDLE_VALUE)
            return;

        STORAGE_DEVICE_NUMBER sdn = {};
        DWORD bytes_returned = 0;
        if (DeviceIoControl(handle, IOCTL_STORAGE_GET_DEVICE_NUMBER, nullptr, 0, &sdn, sizeof(sdn), &bytes_returned, nullptr))
        {
            disk_number_ = sdn.DeviceNumber;
            device_path_ = "\\\\.\\PhysicalDrive" + std::to_string(disk_number_);
        }
        CloseHandle(handle);
    }

    void PhysicalDisk::get_nsid()
    {
        if (auto inst_id_w = devinst_.instance_id())
        {
            std::string inst_id = to_string(*inst_id_w);
            size_t last_amp = inst_id.rfind('&');
            if (last_amp != std::string::npos)
            {
                try
                {
                    nsid_ = std::stoi(inst_id.substr(last_amp + 1)) + 1;
                }
                catch (...)
                {
                    nsid_ = -1;
                }
            }
        }
    }

    void PhysicalDisk::enum_child_volumes()
    {
        drives_ = LogicalDrive::get_drives(disk_number_);
    }

    void PhysicalDisk::open_driver()
    {
        if (!device_path_.empty())
        {
            driver_ = nvme::NvmeDevice::create(std::wstring(device_path_.begin(), device_path_.end()));
        }
    }

    bool PhysicalDisk::disable()
    {
        for (const auto &drive : drives_)
        {
            if (drive == "C:")
            { // Cannot disable C: drive
                return false;
            }
        }
        return devinst_.disable();
    }

    const std::string &PhysicalDisk::path() const { return device_path_; }
    int PhysicalDisk::disk_number() const { return disk_number_; }
    int PhysicalDisk::nsid() const { return nsid_; }
    const DevInstance &PhysicalDisk::devinst() const { return devinst_; }
    nvme::NvmeDevice *PhysicalDisk::get_driver() { return driver_.get(); }

    std::ostream &operator<<(std::ostream &os, const PhysicalDisk &disk)
    {
        os << "  L PhyDisk " << disk.disk_number_ << " (" << disk.devinst_ << "): nsid " << disk.nsid_ << " - Drives: [";
        for (size_t i = 0; i < disk.drives_.size(); ++i)
        {
            os << disk.drives_[i] << (i == disk.drives_.size() - 1 ? "" : ", ");
        }
        os << "]";
        return os;
    }

    // --- NvmeController Implementation ---
    std::optional<NvmeController> NvmeController::create(DEVINST devinst, const std::wstring &interface_path)
    {
        if (auto di = DevInstance::create(devinst))
        {
            if (auto service = di->service(); service && *service == "stornvme")
            {
                return NvmeController(*di, interface_path);
            }
        }
        return std::nullopt;
    }

    NvmeController::NvmeController(DevInstance devinst, std::wstring interface_path)
        : devinst_(devinst), interface_path_(interface_path) {}

    void NvmeController::inspect()
    {
        if (auto bdf_opt = devinst_.pcibdf())
        {
            bdf_ = *bdf_opt;
        }
    }

    void NvmeController::enum_child_disks()
    {
        DEVINST child_inst = 0;
        if (CM_Get_Child(&child_inst, devinst_.handle(), 0) == CR_SUCCESS)
        {
            do
            {
                if (auto disk = PhysicalDisk::create(child_inst))
                {
                    disk->inspect();
                    disks_.push_back(std::move(*disk));
                }
            } while (CM_Get_Sibling(&child_inst, child_inst, 0) == CR_SUCCESS);
        }
    }

    PhysicalDisk *NvmeController::by_num(int driveno)
    {
        for (auto &disk : disks_)
        {
            if (disk.disk_number() == driveno)
            {
                return &disk;
            }
        }
        return nullptr;
    }

    bool NvmeController::enable() { return devinst_.enable(); }
    bool NvmeController::disable()
    {
        for (auto &disk : disks_)
            disk.disable(); // try to disable children first
        return devinst_.disable();
    }
    bool NvmeController::remove() { return devinst_.remove(); }
    bool NvmeController::restart() { return devinst_.restart(); }
    bool NvmeController::refresh() { return DevInstance::refresh(); }
    bool NvmeController::rescan()
    {
        if (auto p = devinst_.parent())
        {
            p->disable();
            // In a real scenario, might need delays
            p->enable();
            return true;
        }
        return false;
    }

    const PciBdf &NvmeController::bdf() const { return bdf_; }
    const DevInstance &NvmeController::devinst() const { return devinst_; }
    const std::vector<PhysicalDisk> &NvmeController::disks() const { return disks_; }

    std::ostream &operator<<(std::ostream &os, const NvmeController &controller)
    {
        auto parent_bdf_str = controller.devinst().parent().has_value()
                                  ? controller.devinst().parent()->pcibdf()
                                  : std::nullopt;
        os << "(" << controller.devinst() << ") "
           << (parent_bdf_str ? *parent_bdf_str : PciBdf{})
           << "/" << controller.bdf() << "\n";
        for (const auto &disk : controller.disks())
        {
            os << disk << "\n";
        }
        return os;
    }

    // --- NvmeControllerList Implementation ---
    void NvmeControllerList::enumerate()
    {
        controllers_.clear();
        ULONG iface_list_size = 0;
        if (CM_Get_Device_Interface_List_SizeW(&iface_list_size, (GUID *)&GUID_DEVINTERFACE_STORAGEPORT, nullptr, CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES) != CR_SUCCESS)
        {
            return;
        }

        std::vector<wchar_t> iface_list(iface_list_size);
        if (CM_Get_Device_Interface_ListW((GUID *)&GUID_DEVINTERFACE_STORAGEPORT, nullptr, iface_list.data(), iface_list_size, CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES) != CR_SUCCESS)
        {
            return;
        }

        for (const wchar_t *interface_str = iface_list.data(); *interface_str; interface_str += wcslen(interface_str) + 1)
        {
            DEVPROPTYPE prop_type;
            ULONG prop_size = 0;
            if (CM_Get_Device_Interface_PropertyW(interface_str, &DEVPKEY_Device_InstanceId, &prop_type, nullptr, &prop_size, 0) != CR_BUFFER_SMALL)
                continue;

            std::vector<wchar_t> dev_id_buf(prop_size / sizeof(wchar_t));
            if (CM_Get_Device_Interface_PropertyW(interface_str, &DEVPKEY_Device_InstanceId, &prop_type, (PBYTE)dev_id_buf.data(), &prop_size, 0) != CR_SUCCESS)
                continue;

            DEVINST devinst = 0;
            if (CM_Locate_DevNodeW(&devinst, dev_id_buf.data(), CM_LOCATE_DEVNODE_NORMAL) != CR_SUCCESS)
                continue;

            if (auto controller = NvmeController::create(devinst, interface_str))
            {
                controller->inspect();
                controller->enum_child_disks();
                controllers_.push_back(std::move(*controller));
            }
        }
        std::sort(controllers_.begin(), controllers_.end(), [](const auto &a, const auto &b)
                  { return a.bdf() < b.bdf(); });
    }

    PhysicalDisk *NvmeControllerList::by_num(int driveno)
    {
        for (auto &controller : controllers_)
        {
            if (auto *disk = controller.by_num(driveno))
            {
                return disk;
            }
        }
        return nullptr;
    }

    NvmeController *NvmeControllerList::by_bus(int bus)
    {
        for (auto &controller : controllers_)
        {
            if (controller.bdf().bus == bus)
            {
                return &controller;
            }
        }
        return nullptr;
    }

    const std::vector<NvmeController> &NvmeControllerList::controllers() const { return controllers_; }

    std::ostream &operator<<(std::ostream &os, const NvmeControllerList &list)
    {
        int index = 0;
        for (const auto &controller : list.controllers())
        {
            os << "NVME " << index++ << ": " << controller;
        }
        return os;
    }

} // namespace dev_utils
