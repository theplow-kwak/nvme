#pragma once

#include "nvme_device.h"
#include "disk.h"

#include <windows.h>
#include <cfgmgr32.h>
#include <string>
#include <vector>
#include <optional>
#include <memory>
#include <iostream>
#include <compare>

namespace dev_utils
{
    class PciBdf
    {
    public:
        int segment = 0;
        int bus = 0;
        int device = 0;
        int function = 0;

        static std::optional<PciBdf> parse(const std::string &location_info);

        std::string to_string() const;
        auto operator<=>(const PciBdf &) const = default;
    };
    std::ostream &operator<<(std::ostream &os, const PciBdf &bdf);

    class DevInstance
    {
    public:
        static std::optional<DevInstance> create(DEVINST devinst);

        std::optional<std::wstring> get_property_string(const DEVPROPKEY &prop_key) const;
        std::optional<std::vector<uint8_t>> get_property_binary(const DEVPROPKEY &prop_key) const;

        std::optional<std::string> service() const;
        std::optional<std::string> location_info() const;
        std::optional<PciBdf> pcibdf() const;
        std::optional<std::wstring> instance_id() const;

        bool enable();
        bool disable();
        bool remove();
        bool restart();
        static bool refresh();

        std::optional<DevInstance> parent() const;
        DEVINST handle() const;

    private:
        DevInstance(DEVINST devinst, ULONG status, ULONG problem);

        DEVINST devinst_;
        ULONG status_;
        ULONG problem_;
    };
    std::ostream &operator<<(std::ostream &os, const DevInstance &dev);

    class PhysicalDisk; // Fwd decl

    class NvmeController
    {
    public:
        static std::optional<NvmeController> create(DEVINST devinst, const std::wstring &interface_path);

        void inspect();
        void enum_child_disks();
        const PhysicalDisk *by_num(int driveno) const;

        bool enable();
        bool disable();
        bool remove();
        bool restart();
        bool refresh();
        bool rescan();

        const PciBdf &bdf() const;
        const DevInstance &devinst() const;
        const std::vector<PhysicalDisk> &disks() const;

    private:
        NvmeController(DevInstance devinst, std::wstring interface_path);

        DevInstance devinst_;
        std::wstring interface_path_;
        PciBdf bdf_;
        std::vector<PhysicalDisk> disks_;
    };
    std::ostream &operator<<(std::ostream &os, const NvmeController &controller);

    class PhysicalDisk
    {
    public:
        static std::optional<PhysicalDisk> create(DEVINST devinst);

        void inspect();
        bool disable();

        const std::string &path() const;
        int disk_number() const;
        int nsid() const;
        const DevInstance &devinst() const;

        nvme::NvmeDevice *get_driver() const;

    private:
        PhysicalDisk(DevInstance devinst);

        void get_nsid();
        void get_interface_path();
        void get_device_number();
        void enum_child_volumes();
        void open_driver();

        DevInstance devinst_;
        std::string interface_path_; // Note: ANSI string in C++ version
        std::string device_path_;    // Note: ANSI string in C++ version
        int disk_number_ = -1;
        int nsid_ = -1;
        std::vector<std::string> drives_;
        std::unique_ptr<nvme::NvmeDevice> driver_;

        friend std::ostream &operator<<(std::ostream &os, const PhysicalDisk &disk);
        friend class NvmeController;
    };
    std::ostream &operator<<(std::ostream &os, const PhysicalDisk &disk);

    class NvmeControllerList
    {
    public:
        NvmeControllerList() = default;

        void enumerate();
        const PhysicalDisk *by_num(int driveno) const;
        const NvmeController *by_bus(int bus) const;

        const std::vector<NvmeController> &controllers() const;

    private:
        std::vector<NvmeController> controllers_;
    };
    std::ostream &operator<<(std::ostream &os, const NvmeControllerList &list);

} // namespace dev_utils
