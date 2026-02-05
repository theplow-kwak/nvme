#include "scsi.h"
#include <malloc.h> // For _aligned_malloc
#include <stdexcept>
#include <intrin.h> // For byte swap

namespace scsi
{

    // --- ScsiDataBuffer implementation ---
    ScsiDataBuffer::ScsiDataBuffer(size_t size) : size_(size)
    {
        aligned_buffer_ = _aligned_malloc(size, SCSI_DATA_BUFFER_ALIGNMENT);
        if (!aligned_buffer_)
        {
            throw std::bad_alloc();
        }
    }

    ScsiDataBuffer::~ScsiDataBuffer()
    {
        if (aligned_buffer_)
        {
            _aligned_free(aligned_buffer_);
        }
    }

    uint8_t *ScsiDataBuffer::data()
    {
        return static_cast<uint8_t *>(aligned_buffer_);
    }

    const uint8_t *ScsiDataBuffer::data() const
    {
        return static_cast<const uint8_t *>(aligned_buffer_);
    }

    size_t ScsiDataBuffer::size() const
    {
        return size_;
    }

    uint8_t &ScsiDataBuffer::operator[](size_t index)
    {
        if (index >= size_)
        {
            throw std::out_of_range("Index out of range");
        }
        return data()[index];
    }

    const uint8_t &ScsiDataBuffer::operator[](size_t index) const
    {
        if (index >= size_)
        {
            throw std::out_of_range("Index out of range");
        }
        return data()[index];
    }

    // --- SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER implementation ---
    SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER::SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER(int dir)
        : sptd(), Filler(0), ucSenseBuf()
    {
        sptd.Length = sizeof(SCSI_PASS_THROUGH_DIRECT);
        sptd.ScsiStatus = 0;
        sptd.PathId = 0;
        sptd.TargetId = 0;
        sptd.Lun = 0;
        sptd.CdbLength = 0; // Must be set by user
        sptd.DataIn = static_cast<UCHAR>(dir);
        sptd.DataBuffer = nullptr;
        sptd.DataTransferLength = 0;
        sptd.SenseInfoOffset = offsetof(SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER, ucSenseBuf);
        sptd.SenseInfoLength = sizeof(SenseBuffer);
        sptd.TimeOutValue = 10;
        // Zero the CDB
        memset(sptd.Cdb, 0, sizeof(sptd.Cdb));
    }

    void SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER::set_buffer(int dir, ScsiDataBuffer &data_src)
    {
        set_buffer(dir, data_src.data(), data_src.size());
    }

    void SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER::set_buffer(int dir, void *data_ptr, size_t data_len)
    {
        sptd.DataIn = static_cast<UCHAR>(dir);
        sptd.DataBuffer = data_ptr;
        sptd.DataTransferLength = static_cast<ULONG>(data_len);
    }

    void SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER::set_buffer(int dir, std::span<const uint8_t> data_src)
    {
        sptd.DataIn = static_cast<UCHAR>(dir);
        // The Windows API expects a non-const pointer. We are promising not to write to it when dir is SCSI_IOCTL_DATA_OUT.
        sptd.DataBuffer = const_cast<uint8_t *>(data_src.data());
        sptd.DataTransferLength = static_cast<ULONG>(data_src.size());
    }

    std::ostream &operator<<(std::ostream &os, const SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER &s)
    {
        os << "ScsiPassThrough: ScsiStatus " << static_cast<int>(s.sptd.ScsiStatus)
           << ", DataTransferLength " << s.sptd.DataTransferLength
           << ", CDB: opcode " << static_cast<int>(s.sptd.Cdb[0])
           << ", flag " << static_cast<int>(s.sptd.Cdb[1]);
        return os;
    }

    // --- Byte swap implementation ---
    template <>
    uint16_t swap_bytes<uint16_t>(uint16_t value)
    {
        return _byteswap_ushort(value);
    }

    template <>
    uint32_t swap_bytes<uint32_t>(uint32_t value)
    {
        return _byteswap_ulong(value);
    }

    template <>
    uint64_t swap_bytes<uint64_t>(uint64_t value)
    {
        return _byteswap_uint64(value);
    }

    // --- CDB constructors ---
    ScsiRwCdb16::ScsiRwCdb16(ScsiOpcode op, uint64_t lba_in, uint32_t len_in, uint8_t flags_in)
        : opcode(static_cast<uint8_t>(op)), flags(flags_in), lba(swap_bytes(lba_in)) // Big Endian
          ,
          len(swap_bytes(len_in)) // Big Endian
          ,
          group(0), control(0)
    {
    }

    ScsiSecCdb12::ScsiSecCdb12(ScsiOpcode op, uint8_t protocol_in, uint16_t com_id_in, uint32_t len_in)
        : opcode(static_cast<uint8_t>(op)), protocol(protocol_in), com_id(swap_bytes(com_id_in)) // Big Endian
          ,
          reserved(0), len(swap_bytes(len_in)) // Big Endian
          ,
          reserved2(0), control(0)
    {
    }

} // namespace scsi
