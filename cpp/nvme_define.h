#pragma once

#include <windows.h>
#include <winioctl.h>
#include <cstdint>
#include <vector>

// The nvme.h header is part of the Windows Driver Kit (WDK) and some recent SDKs.
// It contains the standard definitions for NVMe structures like NVME_COMMAND,
// NVME_IDENTIFY_CONTROLLER_DATA, etc. By including this, we avoid redefining
// standard structures and align with the Windows SDK as requested.
#if __has_include(<nvme.h>)
#include <nvme.h>
#else
#pragma message("Warning: <nvme.h> not found. Using custom definitions. Please ensure the Windows SDK/WDK is installed and configured correctly.")
// Define essential structures if nvme.h is not available.
// This is a minimal set for the identified functionality.
// A full implementation would require translating more from the original Rust file.

// Based on NVME_COMMAND_DWORD0 from Rust
#pragma pack(push, 1)
typedef struct {
    uint32_t OPC : 8;
    uint32_t FUSE : 2;
    uint32_t Reserved0 : 5;
    uint32_t PSDT : 1;
    uint32_t CID : 16;
} NVME_COMMAND_DWORD0;

// Based on NVME_CDW10_IDENTIFY from Rust
typedef struct {
    uint32_t CNS : 8;
    uint32_t Reserved : 8;
    uint32_t CNTID : 16;
} NVME_CDW10_IDENTIFY;

// Based on NVME_CDW10_GET_FEATURES from Rust
typedef struct {
    uint32_t FID : 8;
    uint32_t SEL : 3;
    uint32_t Reserved0 : 21;
} NVME_CDW10_GET_FEATURES;

// Based on NVME_COMMAND from Rust
typedef struct {
    NVME_COMMAND_DWORD0 CDW0;
    uint32_t NSID;
    uint32_t Reserved0[2];
    uint64_t MPTR;
    uint64_t PRP1;
    uint64_t PRP2;
    uint32_t CDW10;
    uint32_t CDW11;
    uint32_t CDW12;
    uint32_t CDW13;
    uint32_t CDW14;
    uint32_t CDW15;
} NVME_COMMAND;

#pragma pack(pop)

#endif // __has_include(<nvme.h>)


namespace nvme {

// This enum corresponds to NVME_IDENTIFY_CNS_CODES in the Rust code.
// While nvme.h defines some CNS values, it may not be as an enum.
// We define it here for clarity and strong typing.
enum class IdentifyCnsCode : uint8_t {
    SpecificNamespace = 0x0,
    Controller = 0x1,
    ActiveNamespaces = 0x2,
    DescriptorNamespace = 0x3,
    NvmSet = 0x4,
    SpecificNamespaceIoCommandSet = 0x5,
    SpecificControllerIoCommandSet = 0x6,
    ActiveNamespaceListIoCommandSet = 0x7,
    AllocatedNamespaceList = 0x10,
    AllocatedNamespace = 0x11,
    ControllerListOfNsId = 0x12,
    ControllerListOfNvmSubsystem = 0x13,
    PrimaryControllerCapabilities = 0x14,
    SecondaryControllerList = 0x15,
    NamespaceGranularityList = 0x16,
    UuidList = 0x17,
    DomainList = 0x18,
    EnduranceGroupList = 0x19,
    AllocatedNamespaceListIoCommandSet = 0x1A,
    AllocatedNamespaceIoCommandSet = 0x1B,
    IoCommandSet = 0x1C,
};

const size_t NVME_IDENTIFY_BUFFER_SIZE = 4096;

} // namespace nvme
