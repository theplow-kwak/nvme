#include "dev_utils.h"
#include "nvme_device.h"
#include "nvme_print.h"
#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <optional>
#include <stdexcept>

// Extended command set from Rust version
enum class Command
{
    None,
    List,
    IdCtrl,
    IdNs,
    ListNs,
    GetFeature,
    SetFeature,
    GetLog,
    Create,
    Delete,
    Attach,
    Detach
};

using enum Command;

struct Args
{
    Command command = Command::None;
    std::optional<int> disk_number;
    std::optional<int> bus_number;
    uint32_t nsid = 1;
    uint32_t fid = 0;
    uint32_t sel = 0;
    uint32_t feature_value = 0;
    std::string log_id;
    bool all_ns = false;
    int create_size = 0;
};

void print_usage()
{
    std::cout << "Usage: nvme-cpp.exe [--disk <num> | --bus <num>] [command]" << std::endl;
    std::cout << "Commands:" << std::endl;
    std::cout << "  list                      List NVMe devices and controllers" << std::endl;
    std::cout << "  id-ctrl                   Identify Controller" << std::endl;
    std::cout << "  id-ns [--nsid <id>]       Identify Namespace" << std::endl;
    std::cout << "  list-ns [--all]           List Namespaces" << std::endl;
    std::cout << "  get-feature --fid <id> [--sel <val>]" << std::endl;
    std::cout << "  set-feature --fid <id> --value <val>" << std::endl;
    std::cout << "  get-log --lid <id_str>    Get Log Page (e.g., 0x02)" << std::endl;
    std::cout << "  create --size <val>       Rescan controller (emulates create)" << std::endl;
    std::cout << "  delete                    Remove/delete controller" << std::endl;
    std::cout << "  attach                    Enable controller" << std::endl;
    std::cout << "  detach                    Disable controller" << std::endl;
}

bool parse_args(int argc, char *argv[], Args &args)
{
    for (int i = 1; i < argc; ++i)
    {
        std::string arg = argv[i];

        // Parameters
        if (arg == "--disk")
        {
            if (++i < argc)
                try
                {
                    args.disk_number = std::stoi(argv[i]);
                }
                catch (const std::invalid_argument &)
                {
                    return false;
                }
        }
        else if (arg == "--bus")
        {
            if (++i < argc)
                try
                {
                    args.bus_number = std::stoi(argv[i]);
                }
                catch (const std::invalid_argument &)
                {
                    return false;
                }
        }
        else if (arg == "--nsid")
        {
            if (++i < argc)
                try
                {
                    args.nsid = std::stoul(argv[i]);
                }
                catch (const std::invalid_argument &)
                {
                    return false;
                }
        }
        else if (arg == "--fid")
        {
            if (++i < argc)
                try
                {
                    args.fid = std::stoul(argv[i], nullptr, 0);
                }
                catch (const std::invalid_argument &)
                {
                    return false;
                }
        }
        else if (arg == "--sel")
        {
            if (++i < argc)
                try
                {
                    args.sel = std::stoul(argv[i], nullptr, 0);
                }
                catch (const std::invalid_argument &)
                {
                    return false;
                }
        }
        else if (arg == "--value")
        {
            if (++i < argc)
                try
                {
                    args.feature_value = std::stoul(argv[i], nullptr, 0);
                }
                catch (const std::invalid_argument &)
                {
                    return false;
                }
        }
        else if (arg == "--lid")
        {
            if (++i < argc)
                args.log_id = argv[i];
        }
        else if (arg == "--all")
        {
            args.all_ns = true;
        }
        else if (arg == "--size")
        {
            if (++i < argc)
                try
                {
                    args.create_size = std::stoi(argv[i]);
                }
                catch (const std::invalid_argument &)
                {
                    return false;
                }
        }
        // Commands
        else if (arg == "list")
            args.command = List;
        else if (arg == "id-ctrl")
            args.command = IdCtrl;
        else if (arg == "id-ns")
            args.command = IdNs;
        else if (arg == "list-ns")
            args.command = ListNs;
        else if (arg == "get-feature")
            args.command = GetFeature;
        else if (arg == "set-feature")
            args.command = SetFeature;
        else if (arg == "get-log")
            args.command = GetLog;
        else if (arg == "create")
            args.command = Create;
        else if (arg == "delete")
            args.command = Delete;
        else if (arg == "attach")
            args.command = Attach;
        else if (arg == "detach")
            args.command = Detach;
        else
        {
            // Unknown argument
            return false;
        }
    }

    // Validate required arguments for specific commands
    switch (args.command)
    {
    case GetFeature:
        return args.fid != 0;
    case SetFeature:
        return args.fid != 0 && args.feature_value != 0;
    case GetLog:
        return !args.log_id.empty();
    case Create:
        return args.create_size > 0;
    case None:
        return false;
    default:
        return true;
    }
}

// Forward declarations for command handlers
void handle_disk_command(const Args &args, const dev_utils::PhysicalDisk &disk);
void handle_controller_command(const Args &args, const dev_utils::NvmeController &ctrl);

int main(int argc, char *argv[])
{
    Args args;
    if (argc < 2 || !parse_args(argc, argv, args))
    {
        print_usage();
        return 1;
    }

    dev_utils::NvmeControllerList controller_list;
    controller_list.enumerate();

    if (args.command == List)
    {
        if (!args.bus_number.has_value())
        {
            std::cout << controller_list;
        }
        else
        {
            if (auto *ctrl = controller_list.by_bus(args.bus_number.value()))
            {
                std::cout << *ctrl;
            }
        }
        return 0;
    }

    // Commands that require a specific device
    if (!args.disk_number.has_value() && !args.bus_number.has_value())
    {
        std::cerr << "Error: --disk <num> or --bus <num> is required for this command." << std::endl;
        print_usage();
        return 1;
    }

    if (args.disk_number.has_value())
    {
        if (auto *disk = controller_list.by_num(args.disk_number.value()))
        {
            handle_disk_command(args, *disk);
        }
        else
        {
            std::cerr << "Error: Disk " << args.disk_number.value() << " not found." << std::endl;
            return 1;
        }
    }
    else if (args.bus_number.has_value())
    {
        if (auto *ctrl = controller_list.by_bus(args.bus_number.value()))
        {
            handle_controller_command(args, *ctrl);
        }
        else
        {
            std::cerr << "Error: Controller on bus " << args.bus_number.value() << " not found." << std::endl;
            return 1;
        }
    }

    return 0;
}

void handle_disk_command(const Args &args, const dev_utils::PhysicalDisk &disk)
{
    auto *device = disk.get_driver();
    if (!device)
    {
        std::cerr << "Error: Could not get driver for disk " << disk.disk_number() << std::endl;
        return;
    }

    switch (args.command)
    {
    case IdCtrl:
    {
        if (auto data = device->identify_controller_struct())
        {
            nvme::print::print_nvme_identify_controller_data(*data);
        }
        else
        {
            std::cerr << "Identify Controller failed." << std::endl;
        }
        break;
    }
    case IdNs:
    {
        if (auto data = device->identify_namespace_struct(args.nsid))
        {
            nvme::print::print_nvme_identify_namespace_data(*data);
        }
        else
        {
            std::cerr << "Identify Namespace failed for NSID " << args.nsid << std::endl;
        }
        break;
    }
    case GetLog:
    {
        uint32_t lid = 0;
        try
        {
            if (args.log_id.starts_with("0x") || args.log_id.starts_with("0X"))
            {
                lid = std::stoul(args.log_id.substr(2), nullptr, 16);
            }
            else
            {
                lid = std::stoul(args.log_id);
            }
        }
        catch (const std::invalid_argument &e)
        {
            std::cerr << "Invalid log ID format: " << args.log_id << std::endl;
            break;
        }
        std::vector<uint8_t> buffer(4096);
        if (device->get_log_page(args.nsid, lid, buffer))
        {
            std::cout << "Get Log Page (LID: 0x" << std::hex << lid << std::dec << ") success." << std::endl;
            // Simple hex dump for now
            for (size_t i = 0; i < 256 && i < buffer.size(); ++i)
            {
                if (i % 16 == 0)
                    std::cout << "\n"
                              << std::setfill('0') << std::setw(4) << i << ": ";
                std::cout << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(buffer[i]) << " ";
            }
            std::cout << std::dec << std::endl;
        }
        else
        {
            std::cerr << "Get Log Page failed." << std::endl;
        }
        break;
    }
    case GetFeature:
    {
        uint32_t result = 0;
        if (device->get_feature(args.fid, args.sel, 0, result))
        {
            nvme::print::print_nvme_get_feature(args.fid, result);
        }
        else
        {
            std::cerr << "Get Feature failed." << std::endl;
        }
        break;
    }
    case SetFeature:
    {
        uint32_t result = 0;
        if (device->set_feature(args.fid, args.feature_value, result))
        {
            nvme::print::print_nvme_set_feature(args.fid, result);
        }
        else
        {
            std::cerr << "Set Feature failed." << std::endl;
        }
        break;
    }
    default:
        std::cout << "This command is not supported when targeting a disk." << std::endl;
        break;
    }
}

void handle_controller_command(const Args &args, const dev_utils::NvmeController &ctrl)
{
    switch (args.command)
    {
    case ListNs:
    {
        // Assuming the first disk's driver is representative for controller-wide commands
        if (ctrl.disks().empty())
        {
            std::cerr << "Cannot get driver from controller." << std::endl;
            break;
        }
        const auto *driver = ctrl.disks()[0].get_driver();
        if (!driver)
        {
            std::cerr << "Cannot get driver from controller." << std::endl;
            break;
        }
        if (auto ns_list = driver->identify_ns_list(0, args.all_ns))
        {
            nvme::print::print_nvme_ns_list(*ns_list);
        }
        else
        {
            std::cerr << "List Namespaces failed." << std::endl;
        }
        break;
    }
    case Create:
    {
        std::cout << "Rescanning controller to emulate create..." << std::endl;
        if (const_cast<dev_utils::NvmeController &>(ctrl).rescan())
            std::cout << "Rescan successful." << std::endl;
        else
            std::cerr << "Rescan failed." << std::endl;
        break;
    }
    case Delete:
    {
        std::cout << "Removing controller..." << std::endl;
        if (const_cast<dev_utils::NvmeController &>(ctrl).remove())
            std::cout << "Remove successful." << std::endl;
        else
            std::cerr << "Remove failed." << std::endl;
        break;
    }
    case Attach:
    {
        std::cout << "Enabling controller..." << std::endl;
        if (const_cast<dev_utils::NvmeController &>(ctrl).enable())
            std::cout << "Enable successful." << std::endl;
        else
            std::cerr << "Enable failed." << std::endl;
        break;
    }
    case Detach:
    {
        std::cout << "Disabling controller..." << std::endl;
        if (const_cast<dev_utils::NvmeController &>(ctrl).disable())
            std::cout << "Disable successful." << std::endl;
        else
            std::cerr << "Disable failed." << std::endl;
        break;
    }
    default:
        // Delegate to disk command handler, using the first disk on the controller
        if (!ctrl.disks().empty())
        {
            handle_disk_command(args, const_cast<dev_utils::NvmeController &>(ctrl).disks()[0]);
        }
        else
        {
            std::cerr << "No disks on this controller to target for the command." << std::endl;
        }
        break;
    }
}