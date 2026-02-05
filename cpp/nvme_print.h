#pragma once

#include <vector>
#include <cstdint>
#include <windows.h>
#include <winioctl.h>
#include <nvme.h>

namespace nvme
{
    namespace print
    {
        void print_nvme_identify_controller_data(const NVME_IDENTIFY_CONTROLLER_DATA &data);
        void print_nvme_identify_namespace_data(const NVME_IDENTIFY_NAMESPACE_DATA &data);
        void print_nvme_ns_list(const std::vector<uint32_t> &ns_list);
        void print_nvme_get_feature(uint32_t fid, uint32_t value);
        void print_nvme_set_feature(uint32_t fid, uint32_t result);
    } // namespace print
} // namespace nvme
