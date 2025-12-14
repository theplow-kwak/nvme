#pragma once

// Prevent <windows.h> from defining min() and max() macros,
// which conflict with std::min and std::max.
#define NOMINMAX
#include <windows.h>
#include <ntddscsi.h>
#include <cstdint>
#include <array>
#include <vector>
#include <ostream>
#include <cstddef>
#include <span>

// From cpp/nvme_define.h - assuming it's available. If not, I'll define it.
// I see `nvme_define.h` in the file tree. I will assume it contains basic type definitions.
// The Rust code doesn't depend on nvme_define.rs, so I might not need it for scsi.

namespace scsi
{
    // SenseBuffer: type SenseBuffer = [u8; 32];
    using SenseBuffer = std::array<uint8_t, 32>;

    // ScsiDataBuffer: A wrapper for an aligned buffer.
    // Alignment must match that of the device. We use a high alignment value.
    constexpr size_t SCSI_DATA_BUFFER_ALIGNMENT = 64;
    class ScsiDataBuffer
    {
    public:
        explicit ScsiDataBuffer(size_t size);
        ~ScsiDataBuffer();

        // Disable copy and move to avoid ownership issues with aligned memory
        ScsiDataBuffer(const ScsiDataBuffer &) = delete;
        ScsiDataBuffer &operator=(const ScsiDataBuffer &) = delete;
        ScsiDataBuffer(ScsiDataBuffer &&) = delete;
        ScsiDataBuffer &operator=(ScsiDataBuffer &&) = delete;

        uint8_t *data();
        const uint8_t *data() const;
        size_t size() const;

        uint8_t &operator[](size_t index);
        const uint8_t &operator[](size_t index) const;

    private:
        void *aligned_buffer_ = nullptr;
        size_t size_ = 0;
    };

// Combined structure for SCSI pass-through
// #[repr(C)] struct SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER
#pragma pack(push, 8)
    struct SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER
    {
        SCSI_PASS_THROUGH_DIRECT sptd;
        ULONG Filler; // For alignment
        SenseBuffer ucSenseBuf;

        SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER(int dir);

        void set_buffer(int dir, ScsiDataBuffer &data_src);
        void set_buffer(int dir, void *data_ptr, size_t data_len);
        void set_buffer(int dir, std::span<const uint8_t> data_src);
    };
#pragma pack(pop)

    std::ostream &operator<<(std::ostream &os, const SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER &s);

    // Enums
    // #[repr(C)] enum ScsiOpcode
    enum class ScsiOpcode : uint8_t
    {
        READ_10 = 0x28,
        READ_16 = 0x88,
        WRITE_10 = 0x2a,
        WRITE_16 = 0x8a,
        READ_CAPACITY_10 = 0x25,
        SERVICE_ACTION_IN = 0x9e,
        SECURITY_RECV = 0xa2,
        SECURITY_SEND = 0xb5,
        TEST_UNIT_READY = 0x00,
    };

    // A constant for the service action, not in the enum in rust code
    const uint8_t SCSI_SERVICE_ACTION_READ_CAPACITY_16 = 0x10;

    // #[repr(C)] enum ScsiCdbFlag
    enum class ScsiCdbFlag : uint8_t
    {
        FUA_NV = 0x02,
        FUA = 0x08,
        DPO = 0x10,
    };

    // Helper for byte swapping
    template <typename T>
    T swap_bytes(T value);

// CDB structures
#pragma pack(push, 1)
    // #[repr(C)] struct ScsiRwCdb16
    struct ScsiRwCdb16
    {
        uint8_t opcode;
        uint8_t flags;
        uint64_t lba;
        uint32_t len;
        uint8_t group;
        uint8_t control;

        ScsiRwCdb16(ScsiOpcode op, uint64_t lba, uint32_t len, uint8_t flags);
    };

    // #[repr(C)] struct ScsiSecCdb12
    struct ScsiSecCdb12
    {
        uint8_t opcode;
        uint8_t protocol;
        uint16_t com_id;
        uint16_t reserved;
        uint32_t len;
        uint8_t reserved2;
        uint8_t control;

        ScsiSecCdb12(ScsiOpcode op, uint8_t protocol, uint16_t com_id, uint32_t len);
    };
#pragma pack(pop)

} // namespace scsi
