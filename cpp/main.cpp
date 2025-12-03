#include <iostream>
#include <string>
#include <vector>
#include <optional>
#include "nvme_device.h"
#include "nvme_print.h"

// Basic argument parsing to replace clap
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
};

struct Args
{
    Command command = Command::None;
    int disk_number = -1;
    uint32_t nsid = 1;
    uint32_t fid = 0;
    uint32_t sel = 0;
    uint32_t feature_value = 0;
    uint32_t log_id = 0;
    bool all_ns = false;
};

void print_usage()
{
    std::cout << "Usage: nvme-cpp.exe --disk <num> [command]" << std::endl;
    std::cout << "Commands:" << std::endl;
    std::cout << "  list                      List NVMe devices" << std::endl;
    std::cout << "  id-ctrl                   Identify Controller" << std::endl;
    std::cout << "  id-ns [--nsid <id>]       Identify Namespace" << std::endl;
    std::cout << "  list-ns [--all]           List Namespaces" << std::endl;
    std::cout << "  get-feature --fid <id> [--sel <val>]" << std::endl;
    std::cout << "  set-feature --fid <id> --value <val>" << std::endl;
    std::cout << "  get-log --lid <id>        Get Log Page" << std::endl;
}

bool parse_args(int argc, char *argv[], Args &args)
{
    for (int i = 1; i < argc; ++i)
    {
        std::string arg = argv[i];
        if (arg == "--disk")
        {
            if (i + 1 < argc)
            {
                args.disk_number = std::stoi(argv[++i]);
            }
        }
        else if (arg == "--nsid")
        {
            if (i + 1 < argc)
            {
                args.nsid = std::stoul(argv[++i]);
            }
        }
        else if (arg == "--fid")
        {
            if (i + 1 < argc)
            {
                args.fid = std::stoul(argv[++i], nullptr, 0);
            }
        }
        else if (arg == "--sel")
        {
            if (i + 1 < argc)
            {
                args.sel = std::stoul(argv[++i], nullptr, 0);
            }
        }
        else if (arg == "--value")
        {
            if (i + 1 < argc)
            {
                args.feature_value = std::stoul(argv[++i], nullptr, 0);
            }
        }
        else if (arg == "--lid")
        {
            if (i + 1 < argc)
            {
                args.log_id = std::stoul(argv[++i], nullptr, 0);
            }
        }
        else if (arg == "--all")
        {
            args.all_ns = true;
        }
        else if (arg == "list")
        {
            args.command = Command::List;
        }
        else if (arg == "id-ctrl")
        {
            args.command = Command::IdCtrl;
        }
        else if (arg == "id-ns")
        {
            args.command = Command::IdNs;
        }
        else if (arg == "list-ns")
        {
            args.command = Command::ListNs;
        }
        else if (arg == "get-feature")
        {
            args.command = Command::GetFeature;
        }
        else if (arg == "set-feature")
        {
            args.command = Command::SetFeature;
        }
        else if (arg == "get-log")
        {
            args.command = Command::GetLog;
        }
    }
    return args.command != Command::None;
}

int main(int argc, char *argv[])
{
    Args args;
    if (argc < 2 || !parse_args(argc, argv, args))
    {
        print_usage();
        return 1;
    }

    if (args.command == Command::List)
    {
        nvme::NvmeDeviceDiscovery discovery;
        discovery.enumerate_devices();
        std::cout << "NVMe Drives:" << std::endl;
        for (const auto &info : discovery.get_devices())
        {
            std::wcout << L"  PhysicalDrive" << info.physical_drive_number
                       << L": " << info.model_number
                       << L" (" << info.serial_number << L")" << std::endl;
        }
        return 0;
    }

    if (args.disk_number < 0)
    {
        std::cerr << "Error: --disk <num> is required for this command." << std::endl;
        return 1;
    }

    std::wstring path = L"\\.\PhysicalDrive" + std::to_wstring(args.disk_number);
    auto device = nvme::NvmeDevice::create(path);
    if (!device)
    {
        std::cerr << "Error: Could not open NVMe device " << args.disk_number << std::endl;
        return 1;
    }

    bool success = false;

    switch (args.command)
    {
    case Command::IdCtrl:
    {
        std::vector<uint8_t> buffer;
        success = device->nvme_identify_controller(buffer);
        if (success)
        {
            auto *data = reinterpret_cast<NVME_IDENTIFY_CONTROLLER_DATA *>(buffer.data());
            nvme::print::print_nvme_identify_controller_data(*data);
        }
        break;
    }
    case Command::IdNs:
    {
        std::vector<uint8_t> buffer;
        success = device->nvme_identify_namespace(args.nsid, buffer);
        if (success)
        {
            auto *data = reinterpret_cast<NVME_IDENTIFY_NAMESPACE_DATA *>(buffer.data());
            nvme::print::print_nvme_identify_namespace_data(*data);
        }
        break;
    }
    case Command::ListNs:
    {
        std::vector<uint8_t> buffer(4096);
        NVME_CDW10_IDENTIFY cdw10 = {};
        cdw10.CNS = static_cast<uint8_t>(
            args.all_ns ? nvme::IdentifyCnsCode::AllocatedNamespaceList : nvme::IdentifyCnsCode::ActiveNamespaces);
        success = device->nvme_identify_query(cdw10, buffer, 0);
        if (success)
        {
            std::vector<uint32_t> ns_list;
            for (size_t i = 0; i < 1024; ++i)
            { // NVMe spec lists up to 1024 namespaces
                uint32_t nsid = *reinterpret_cast<uint32_t *>(buffer.data() + i * 4);
                if (nsid == 0)
                    break;
                ns_list.push_back(nsid);
            }
            nvme::print::print_nvme_ns_list(ns_list);
        }
        break;
    }
    case Command::GetFeature:
    {
        NVME_CDW10_GET_FEATURES cdw10 = {};
        cdw10.FID = args.fid;
        cdw10.SEL = args.sel;
        uint32_t result = 0;
        success = device->nvme_get_feature(cdw10, result);
        if (success)
        {
            nvme::print::print_nvme_get_feature(args.fid, result);
        }
        break;
    }
    case Command::SetFeature:
    {
        uint32_t result = 0;
        success = device->nvme_set_feature(args.fid, args.feature_value, result);
        if (success)
        {
            nvme::print::print_nvme_set_feature(args.fid, result);
        }
        break;
    }
    case Command::GetLog:
    {
        std::vector<uint8_t> buffer(4096);
        success = device->nvme_logpage_query(args.log_id, buffer);
        if (success)
        {
            std::cout << "Get Log Page (LID: 0x" << std::hex << args.log_id << std::dec << ") success." << std::endl;
            // Basic hex dump for now
            for (size_t i = 0; i < 256 && i < buffer.size(); ++i)
            {
                if (i % 16 == 0)
                    std::cout << std::endl
                              << std::setfill('0') << std::setw(4) << i << ": ";
                std::cout << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(buffer[i]) << " " << std::dec;
            }
            std::cout << std::endl;
        }
        break;
    }
    default:
        std::cerr << "Command not implemented yet." << std::endl;
        break;
    }

    if (!success)
    {
        std::cerr << "Command failed to execute." << std::endl;
        return 1;
    }

    return 0;
}
