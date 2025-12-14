#include "nvme_print.h"
#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <bitset>

namespace
{
    // Helper to print a string from a char array of a specific length
    void print_char_array(const char *name, const unsigned char *array, size_t len)
    {
        std::string str(reinterpret_cast<const char *>(array), len);
        // Trim trailing spaces and nulls
        size_t last = str.find_last_not_of(" \0");
        if (std::string::npos != last)
        {
            str = str.substr(0, last + 1);
        }
        else
        {
            str.clear();
        }
        std::cout << "  " << std::left << std::setw(10) << name << ": " << str << std::endl;
    }

    // Helper to print a hex value
    template <typename T>
    void print_hex(const char *name, T value)
    {
        std::cout << "  " << std::left << std::setw(10) << name << ": 0x" << std::hex << static_cast<uint64_t>(value) << std::dec << std::endl;
    }

    // Helper to print a decimal value
    template <typename T>
    void print_dec(const char *name, T value)
    {
        std::cout << "  " << std::left << std::setw(10) << name << ": " << static_cast<uint64_t>(value) << std::endl;
    }

    template <size_t N>
    void print_bitset(const char *name, const std::bitset<N> &b)
    {
        std::cout << "  " << std::left << std::setw(10) << name << ": " << b.to_string() << std::endl;
    }

} // anonymous namespace

namespace nvme
{
    namespace print
    {
        void print_nvme_identify_controller_data(const NVME_IDENTIFY_CONTROLLER_DATA &data)
        {
            std::cout << "NVMe Identify Controller Data:" << std::endl;
            std::cout << "------------------------------" << std::endl;
            print_hex("vid", data.VID);
            print_hex("ssvid", data.SSVID);
            print_char_array("sn", data.SN, sizeof(data.SN));
            print_char_array("mn", data.MN, sizeof(data.MN));
            print_char_array("fr", data.FR, sizeof(data.FR));
            print_dec("rab", data.RAB);
            print_dec("mdts", data.MDTS);
            print_hex("cntlid", data.CNTLID);
            print_hex("ver", data.VER);
            print_hex("oacs", data.OACS);
            print_dec("acl", data.ACL);
            print_dec("aerl", data.AERL);
            print_hex("frmw", data.FRMW);
            print_hex("lpa", data.LPA);
            print_dec("elpe", data.ELPE);
            print_dec("npss", data.NPSS);
            print_hex("avscc", data.AVSCC);
            print_hex("apsta", data.APSTA);
            print_hex("wctemp", data.WCTEMP);
            print_hex("cctemp", data.CCTEMP);
            print_dec("mtfa", data.MTFA);
            print_dec("hmpre", data.HMPRE);
            print_dec("hmmin", data.HMMIN);
            print_hex("sqes", data.SQES);
            print_hex("cqes", data.CQES);
            print_dec("maxcmd", data.MAXCMD);
            print_dec("nn", data.NN);
            print_hex("oncs", data.ONCS);
            print_hex("fuses", data.FUSES);
            print_hex("fna", data.FNA);
            print_hex("vwc", data.VWC);
            print_dec("awun", data.AWUN);
            print_dec("awupf", data.AWUPF);
            print_hex("nvscc", data.NVSCC);
            print_hex("acwu", data.ACWU);
            print_hex("sgls", data.SGLS);
            print_char_array("subnqn", data.SUBNQN, sizeof(data.SUBNQN));
            std::cout << std::endl;
        }

        void print_nvme_identify_namespace_data(const NVME_IDENTIFY_NAMESPACE_DATA &data)
        {
            std::cout << "NVMe Identify Namespace Data:" << std::endl;
            std::cout << "-------------------------------" << std::endl;
            print_dec("nsze", data.NSZE);
            print_dec("ncap", data.NCAP);
            print_dec("nuse", data.NUSE);
            print_hex("nsfeat", data.NSFEAT);
            print_dec("nlbaf", data.NLBAF);
            print_hex("flbas", data.FLBAS);
            print_hex("mc", data.MC);
            print_hex("dpc", data.DPC);
            print_hex("dps", data.DPS);

            uint32_t lba_format_idx = data.FLBAS.LbaFormatIndex;
            if (lba_format_idx < 16)
            {
                const auto &lba_format = data.LBAF[lba_format_idx];
                uint32_t lba_size = 1 << lba_format.DUMMYSTRUCTNAME.LBADS;
                std::cout << "  LBAF " << lba_format_idx << "  : "
                          << "LBA Size=" << lba_size << " bytes, "
                          << "Metadata Size=" << static_cast<int>(lba_format.DUMMYSTRUCTNAME.MS) << " bytes" << std::endl;
            }
            std::cout << std::endl;
        }

        void print_nvme_ns_list(const std::vector<uint32_t> &ns_list)
        {
            if (ns_list.empty())
            {
                std::cout << "No active/allocated namespaces found." << std::endl;
                return;
            }
            std::cout << "Active/Allocated Namespace IDs:" << std::endl;
            for (uint32_t nsid : ns_list)
            {
                if (nsid > 0)
                {
                    std::cout << "  - " << nsid << std::endl;
                }
            }
            std::cout << std::endl;
        }

        void print_nvme_get_feature(uint32_t fid, uint32_t value)
        {
            std::cout << "Get Feature (FID: 0x" << std::hex << fid << std::dec << ")" << std::endl;
            std::cout << "  Raw Value: 0x" << std::hex << value << std::dec << std::endl;

            switch (fid)
            {
            case NVME_FEATURE_ARBITRATION:
            {
                NVME_CDW11_FEATURE_ARBITRATION info;
                info.AsUlong = value;
                std::cout << "  Arbitration:" << std::endl;
                std::cout << "    Arbitration Burst (AB) : " << static_cast<int>(info.DUMMYSTRUCTNAME.AB) << std::endl;
                std::cout << "    Low Priority Weight (LPW) : " << static_cast<int>(info.DUMMYSTRUCTNAME.LPW) << std::endl;
                std::cout << "    Medium Priority Weight (MPW): " << static_cast<int>(info.DUMMYSTRUCTNAME.MPW) << std::endl;
                std::cout << "    High Priority Weight (HPW): " << static_cast<int>(info.DUMMYSTRUCTNAME.HPW) << std::endl;
                break;
            }
            case NVME_FEATURE_POWER_MANAGEMENT:
            {
                NVME_CDW11_FEATURE_POWER_MANAGEMENT info;
                info.AsUlong = value;
                std::cout << "  Power Management:" << std::endl;
                std::cout << "    Power State (PS): " << static_cast<int>(info.DUMMYSTRUCTNAME.PS) << std::endl;
                break;
            }
            case NVME_FEATURE_TEMPERATURE_THRESHOLD:
            {
                NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD info;
                info.AsUlong = value;
                std::cout << "  Temperature Threshold:" << std::endl;
                std::cout << "    Temp Threshold (TMPTH) : " << static_cast<int>(info.DUMMYSTRUCTNAME.TMPTH) << std::endl;
                break;
            }
            case NVME_FEATURE_ERROR_RECOVERY:
            {
                NVME_CDW11_FEATURE_ERROR_RECOVERY info;
                info.AsUlong = value;
                std::cout << "  Error Recovery:" << std::endl;
                std::cout << "    Time Limited Error Recovery (TLER): " << static_cast<int>(info.DUMMYSTRUCTNAME.TLER) << std::endl;
                break;
            }
            case NVME_FEATURE_VOLATILE_WRITE_CACHE:
            {
                NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE info;
                info.AsUlong = value;
                std::cout << "  Volatile Write Cache:" << std::endl;
                std::cout << "    Write Cache Enabled (WCE): " << static_cast<int>(info.DUMMYSTRUCTNAME.WCE) << std::endl;
                break;
            }
            case NVME_FEATURE_NUMBER_OF_QUEUES:
            {
                NVME_CDW11_FEATURE_NUMBER_OF_QUEUES info;
                info.AsUlong = value;
                std::cout << "  Number of Queues:" << std::endl;
                std::cout << "    Num Submission Queues (NSQ): " << static_cast<int>(info.DUMMYSTRUCTNAME.NSQ) << std::endl;
                std::cout << "    Num Completion Queues (NCQ): " << static_cast<int>(info.DUMMYSTRUCTNAME.NCQ) << std::endl;
                break;
            }
            case NVME_FEATURE_ASYNC_EVENT_CONFIG:
            {
                NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG info;
                info.AsUlong = value;
                std::cout << "  Asynchronous Event Configuration:" << std::endl;
                std::cout << "    Critical Warnings      : " << static_cast<int>(info.DUMMYSTRUCTNAME.CriticalWarnings) << std::endl;
                std::cout << "    Namespace Attributes   : " << static_cast<int>(info.DUMMYSTRUCTNAME.NsAttributeNotices) << std::endl;
                std::cout << "    Firmware Activation    : " << static_cast<int>(info.DUMMYSTRUCTNAME.FwActivationNotices) << std::endl;
                break;
            }
            default:
                std::cout << "  (No detailed print for this FID)" << std::endl;
                break;
            }
            std::cout << std::endl;
        }

        void print_nvme_set_feature(uint32_t fid, uint32_t result)
        {
            std::cout << "Set Feature (FID: 0x" << std::hex << fid << std::dec << ")" << std::endl;
            // The 'result' from a Set Feature is the value of the feature *before* the change.
            std::cout << "  Value Before Change: 0x" << std::hex << result << std::dec << std::endl;
            std::cout << std::endl;
        }

    } // namespace print
} // namespace nvme