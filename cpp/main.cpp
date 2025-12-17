#include "argparser.hpp"
#include "dev_utils.h"
#include "nvme_device.h"
#include "nvme_print.h"
#include <iomanip>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>
#include <vector>

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

// Forward declarations for command handlers
void handle_disk_command(Command command, argparse::ArgParser &parser, const dev_utils::PhysicalDisk &disk);
void handle_controller_command(Command command, argparse::ArgParser &parser, dev_utils::NvmeController &ctrl);

int main(int argc, char *argv[])
{
    argparse::ArgParser parser("A command-line tool for interacting with NVMe devices.");

    parser.add_positional("command", "The command to execute (e.g., list, id-ctrl, id-ns).", true);
    parser.add_option("--disk", "-d", "Target a specific disk number.", false, "-1");
    parser.add_option("--bus", "-b", "Target a specific controller bus number.", false, "-1");
    parser.add_option("--nsid", "", "Namespace ID for commands like id-ns.", false, "1");
    parser.add_option("--fid", "", "Feature ID for get/set-feature (hex or dec).", false, "0");
    parser.add_option("--sel", "", "Select value for get-feature (hex or dec).", false, "0");
    parser.add_option("--value", "", "Value for set-feature (hex or dec).", false, "0");
    parser.add_option("--lid", "", "Log ID for get-log (hex or dec).", false, "");
    parser.add_option("--size", "", "Size for create.", false, "0");
    parser.add_flag("--all", "-a", "Apply to all namespaces (e.g., with list-ns).");

    if (!parser.parse(argc, argv))
    {
        return 1;
    }

    Command command = Command::None;
    auto cmd_str_opt = parser.get_positional("command");
    if (!cmd_str_opt.has_value())
    {
        std::cerr << "Error: No command specified." << std::endl;
        parser.print_help(argv[0]);
        return 1;
    }
    const auto &cmd_str = cmd_str_opt.value();

    if (cmd_str == "list")
        command = Command::List;
    else if (cmd_str == "id-ctrl")
        command = Command::IdCtrl;
    else if (cmd_str == "id-ns")
        command = Command::IdNs;
    else if (cmd_str == "list-ns")
        command = Command::ListNs;
    else if (cmd_str == "get-feature")
        command = Command::GetFeature;
    else if (cmd_str == "set-feature")
        command = Command::SetFeature;
    else if (cmd_str == "get-log")
        command = Command::GetLog;
    else if (cmd_str == "create")
        command = Command::Create;
    else if (cmd_str == "delete")
        command = Command::Delete;
    else if (cmd_str == "attach")
        command = Command::Attach;
    else if (cmd_str == "detach")
        command = Command::Detach;
    else
    {
        std::cerr << "Unknown command: " << cmd_str << std::endl;
        parser.print_help(argv[0]);
        return 1;
    }

    // Command-specific argument validation
    if (command == Command::GetFeature && !parser.is_set("fid"))
    {
        std::cerr << "Error: --fid is required for get-feature." << std::endl;
        return 1;
    }
    if (command == Command::SetFeature && (!parser.is_set("fid") || !parser.is_set("value")))
    {
        std::cerr << "Error: --fid and --value are required for set-feature." << std::endl;
        return 1;
    }
    if (command == Command::GetLog && !parser.is_set("lid"))
    {
        std::cerr << "Error: --lid is required for get-log." << std::endl;
        return 1;
    }
    if (command == Command::Create && !parser.is_set("size"))
    {
        std::cerr << "Error: --size is required for create." << std::endl;
        return 1;
    }

    dev_utils::NvmeControllerList controller_list;
    controller_list.enumerate();

    auto disk_number = parser.get<int>("disk").value_or(-1);
    auto bus_number = parser.get<int>("bus").value_or(-1);

    if (command == Command::List)
    {
        if (bus_number == -1)
        {
            std::cout << controller_list;
        }
        else
        {
            if (auto *ctrl = controller_list.by_bus(bus_number))
            {
                std::cout << *ctrl;
            }
        }
        return 0;
    }

    if (disk_number == -1 && bus_number == -1)
    {
        std::cerr << "Error: --disk <num> or --bus <num> is required for this command." << std::endl;
        parser.print_help(argv[0]);
        return 1;
    }

    if (disk_number != -1)
    {
        if (auto *disk = controller_list.by_num(disk_number))
        {
            handle_disk_command(command, parser, *disk);
        }
        else
        {
            std::cerr << "Error: Disk " << disk_number << " not found." << std::endl;
            return 1;
        }
    }
    else if (bus_number != -1)
    {
        if (auto *ctrl = controller_list.by_bus(bus_number))
        {
            handle_controller_command(command, parser, *ctrl);
        }
        else
        {
            std::cerr << "Error: Controller on bus " << bus_number << " not found." << std::endl;
            return 1;
        }
    }

    return 0;
}

void handle_disk_command(Command command, argparse::ArgParser &parser, const dev_utils::PhysicalDisk &disk)
{
    auto *device = disk.get_driver();
    if (!device)
    {
        std::cerr << "Error: Could not get driver for disk " << disk.disk_number() << std::endl;
        return;
    }

    switch (command)
    {
    case Command::IdCtrl:
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
    case Command::IdNs:
    {
        auto nsid = parser.get<uint32_t>("nsid").value_or(1);
        if (auto data = device->identify_namespace_struct(nsid))
        {
            nvme::print::print_nvme_identify_namespace_data(*data);
        }
        else
        {
            std::cerr << "Identify Namespace failed for NSID " << nsid << std::endl;
        }
        break;
    }
    case Command::GetLog:
    {
        uint32_t lid = 0;
        auto log_id_str = parser.get<std::string>("lid").value();
        try
        {
            if (log_id_str.starts_with("0x") || log_id_str.starts_with("0X"))
            {
                lid = std::stoul(log_id_str.substr(2), nullptr, 16);
            }
            else
            {
                lid = std::stoul(log_id_str);
            }
        }
        catch (const std::invalid_argument &)
        {
            std::cerr << "Invalid log ID format: " << log_id_str << std::endl;
            break;
        }
        auto nsid = parser.get<uint32_t>("nsid").value_or(1);
        std::vector<uint8_t> buffer(4096);
        if (device->get_log_page(nsid, lid, buffer))
        {
            std::cout << "Get Log Page (LID: 0x" << std::hex << lid << std::dec << ") success." << std::endl;
            for (size_t i = 0; i < 256 && i < buffer.size(); ++i)
            {
                if (i % 16 == 0)
                    std::cout << "\n"
                              << std::setfill('0') << std::setw(4) << std::hex << i << ": ";
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
    case Command::GetFeature:
    {
        uint32_t result = 0;
        auto fid = parser.get<uint32_t>("fid").value();
        auto sel = parser.get<uint32_t>("sel").value_or(0);
        if (device->get_feature(fid, sel, 0, result))
        {
            nvme::print::print_nvme_get_feature(fid, result);
        }
        else
        {
            std::cerr << "Get Feature failed." << std::endl;
        }
        break;
    }
    case Command::SetFeature:
    {
        uint32_t result = 0;
        auto fid = parser.get<uint32_t>("fid").value();
        auto value = parser.get<uint32_t>("value").value();
        if (device->set_feature(fid, value, result))
        {
            nvme::print::print_nvme_set_feature(fid, result);
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

void handle_controller_command(Command command, argparse::ArgParser &parser, dev_utils::NvmeController &ctrl)
{
    switch (command)
    {
    case Command::ListNs:
    {
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
        bool all_ns = parser.is_set("all");
        if (auto ns_list = driver->identify_ns_list(0, all_ns))
        {
            nvme::print::print_nvme_ns_list(*ns_list);
        }
        else
        {
            std::cerr << "List Namespaces failed." << std::endl;
        }
        break;
    }
    case Command::Create:
    {
        std::cout << "Rescanning controller to emulate create..." << std::endl;
        if (ctrl.rescan())
            std::cout << "Rescan successful." << std::endl;
        else
            std::cerr << "Rescan failed." << std::endl;
        break;
    }
    case Command::Delete:
    {
        std::cout << "Removing controller..." << std::endl;
        if (ctrl.remove())
            std::cout << "Remove successful." << std::endl;
        else
            std::cerr << "Remove failed." << std::endl;
        break;
    }
    case Command::Attach:
    {
        std::cout << "Enabling controller..." << std::endl;
        if (ctrl.enable())
            std::cout << "Enable successful." << std::endl;
        else
            std::cerr << "Enable failed." << std::endl;
        break;
    }
    case Command::Detach:
    {
        std::cout << "Disabling controller..." << std::endl;
        if (ctrl.disable())
            std::cout << "Disable successful." << std::endl;
        else
            std::cerr << "Disable failed." << std::endl;
        break;
    }
    default:
        if (!ctrl.disks().empty())
        {
            handle_disk_command(command, parser, ctrl.disks()[0]);
        }
        else
        {
            std::cerr << "No disks on this controller to target for the command." << std::endl;
        }
        break;
    }
}