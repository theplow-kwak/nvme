#include "nvme_print.h"
#include <iostream>
#include <iomanip>
#include <string>
#include <vector>

namespace {
// Helper to print a string from a char array of a specific length
void print_char_array(const char* name, const unsigned char* array, size_t len) {
    std::string str(reinterpret_cast<const char*>(array), len);
    // Trim trailing spaces and nulls
    str.erase(str.find_last_not_of(" \0") + 1);
    std::cout << std::left << std::setw(10) << name << ": " << str << std::endl;
}

// Helper to print a hex value
template <typename T>
void print_hex(const char* name, T value) {
    std::cout << std::left << std::setw(10) << name << ": 0x" << std::hex << value << std::dec << std::endl;
}

// Helper to print a decimal value
template <typename T>
void print_dec(const char* name, T value) {
    std::cout << std::left << std::setw(10) << name << ": " << value << std::endl;
}

} // anonymous namespace

namespace nvme {
namespace print {

void print_nvme_identify_controller_data(const NVME_IDENTIFY_CONTROLLER_DATA& data) {
    std::cout << "NVMe Identify Controller Data:" << std::endl;
    std::cout << "------------------------------" << std::endl;
    print_hex("VID", data.VID);
    print_char_array("SN", data.SN, sizeof(data.SN));
    print_char_array("MN", data.MN, sizeof(data.MN));
    print_char_array("FR", data.FR, sizeof(data.FR));
    print_dec("NN", data.NN);
    std::cout << "SQES      : Min=" << static_cast<int>(data.SQES.MinEntrySize) << ", Max=" << static_cast<int>(data.SQES.MaxEntrySize) << std::endl;
    std::cout << "CQES      : Min=" << static_cast<int>(data.CQES.MinEntrySize) << ", Max=" << static_cast<int>(data.CQES.MaxEntrySize) << std::endl;
    print_dec("MAXCMD", data.MAXCMD);
    std::cout << std::endl;
}

void print_nvme_identify_namespace_data(const NVME_IDENTIFY_NAMESPACE_DATA& data) {
    std::cout << "NVMe Identify Namespace Data:" << std::endl;
    std::cout << "-------------------------------" << std::endl;
    print_dec("NSZE", data.NSZE);
    print_dec("NCAP", data.NCAP);
    print_dec("NUSE", data.NUSE);

    uint32_t lba_format_idx = data.FLBAS.LbaFormatIndex;
    if (lba_format_idx < 16) {
        const auto& lba_format = data.LBAF[lba_format_idx];
        uint32_t lba_size = 1 << lba_format.LBADataSize;
        std::cout << "LBA Size  : " << lba_size << " bytes" << std::endl;
        std::cout << "Metadata  : " << lba_format.MetadataSize << " bytes" << std::endl;
    }
    std::cout << std::endl;
}

void print_nvme_ns_list(const std::vector<uint32_t>& ns_list) {
    if (ns_list.empty()) {
        std::cout << "No active namespaces found." << std::endl;
        return;
    }
    std::cout << "Active/Allocated Namespace IDs:" << std::endl;
    for (uint32_t nsid : ns_list) {
        if (nsid > 0) {
            std::cout << "  - " << nsid << std::endl;
        }
    }
    std::cout << std::endl;
}

void print_nvme_get_feature(uint32_t fid, uint32_t value) {
    std::cout << "Get Feature (FID: 0x" << std::hex << fid << std::dec << ")" << std::endl;
    std::cout << "  Result: 0x" << std::hex << value << std::dec << std::endl;
    std::cout << std::endl;
}

void print_nvme_set_feature(uint32_t fid, uint32_t result) {
    std::cout << "Set Feature (FID: 0x" << std::hex << fid << std::dec << ")" << std::endl;
    // The 'result' from a Set Feature is the value of the feature *before* the change.
    // The completion status in the IOCTL indicates success or failure.
    std::cout << "  Value before change: 0x" << std::hex << result << std::dec << std::endl;
    std::cout << std::endl;
}


} // namespace print
} // namespace nvme
