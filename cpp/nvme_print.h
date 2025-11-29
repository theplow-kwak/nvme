#pragma once

#include "nvme_define.h"
#include <vector>
#include <cstdint>

// Forward-declare standard NVMe structures to avoid full include in header if possible
// These should be defined in <nvme.h>
struct _NVME_IDENTIFY_CONTROLLER_DATA;
typedef _NVME_IDENTIFY_CONTROLLER_DATA NVME_IDENTIFY_CONTROLLER_DATA;

struct _NVME_IDENTIFY_NAMESPACE_DATA;
typedef _NVME_IDENTIFY_NAMESPACE_DATA NVME_IDENTIFY_NAMESPACE_DATA;


namespace nvme {
namespace print {

void print_nvme_identify_controller_data(const NVME_IDENTIFY_CONTROLLER_DATA& data);
void print_nvme_identify_namespace_data(const NVME_IDENTIFY_NAMESPACE_DATA& data);
void print_nvme_ns_list(const std::vector<uint32_t>& ns_list);
void print_nvme_get_feature(uint32_t fid, uint32_t value);
void print_nvme_set_feature(uint32_t fid, uint32_t result);

} // namespace print
} // namespace nvme
