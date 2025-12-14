#include "disk.h"
#include <winioctl.h>
#include <stdexcept>
#include <system_error>
#include <malloc.h> // For _aligned_malloc
#include <span>
#include <cstring> // For memcpy

namespace
{ // Anonymous namespace for internal linkage
    constexpr int SECTOR_SIZE = 512;

    DWORD last_error()
    {
        return GetLastError();
    }

    HANDLE open_handle(const std::string &path, char rw)
    {
        DWORD access = (rw == 'w') ? (GENERIC_WRITE | GENERIC_READ) : GENERIC_READ;
        DWORD creation = (rw == 'w' && path.find("\\\\.\\.") == std::string::npos) ? CREATE_ALWAYS : OPEN_EXISTING;

        return CreateFileA(
            path.c_str(),
            access,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            nullptr,
            creation,
            FILE_FLAG_NO_BUFFERING | FILE_FLAG_WRITE_THROUGH,
            NULL);
    }

    size_t ioctl_wrapper(
        HANDLE handle,
        DWORD control_code,
        const void *in_buffer,
        size_t in_buffer_size,
        void *out_buffer,
        size_t out_buffer_size)
    {
        DWORD bytes_returned = 0;
        BOOL ok = DeviceIoControl(
            handle,
            control_code,
            const_cast<void *>(in_buffer),
            static_cast<DWORD>(in_buffer_size),
            out_buffer,
            static_cast<DWORD>(out_buffer_size),
            &bytes_returned,
            nullptr);

        if (!ok)
        {
            throw std::system_error(last_error(), std::system_category(), "DeviceIoControl failed");
        }
        return bytes_returned;
    }

    size_t get_drive_geometry(HANDLE drive)
    {
        DISK_GEOMETRY_EX geo = {};
        try
        {
            ioctl_wrapper(drive, IOCTL_DISK_GET_DRIVE_GEOMETRY_EX, nullptr, 0, &geo, sizeof(geo));
            return geo.DiskSize.QuadPart;
        }
        catch (const std::system_error &)
        {
            return 0;
        }
    }

    size_t get_file_size(HANDLE drive)
    {
        LARGE_INTEGER size = {};
        if (GetFileSizeEx(drive, &size) == 0)
        {
            return get_drive_geometry(drive);
        }
        return size.QuadPart;
    }

} // anonymous namespace

namespace disk
{

    std::optional<Disk> Disk::open(const std::string &path, char rw, std::optional<bool> fua)
    {
        HANDLE handle = open_handle(path, rw);
        if (handle == INVALID_HANDLE_VALUE)
        {
            std::cerr << "Can't open file!! '" << path << "'\n";
            return std::nullopt;
        }
        return Disk(path, rw, fua, handle);
    }

    Disk::Disk(const std::string &path, char rw, std::optional<bool> fua, HANDLE handle)
        : path_(path),
          rw_(rw),
          handle_(handle),
          size_(get_file_size(handle)),
          lba_shift_(9),
          write_offset_(0),
          sptdwb_(SCSI_IOCTL_DATA_OUT),
          fua_(fua) {}

    Disk::~Disk()
    {
        close();
    }

    Disk::Disk(Disk &&other) noexcept
        : path_(std::move(other.path_)),
          rw_(other.rw_),
          handle_(other.handle_),
          size_(other.size_),
          lba_shift_(other.lba_shift_),
          write_offset_(other.write_offset_),
          sptdwb_(other.sptdwb_),
          fua_(other.fua_)
    {
        other.handle_ = INVALID_HANDLE_VALUE; // Prevent double-close
    }

    Disk &Disk::operator=(Disk &&other) noexcept
    {
        if (this != &other)
        {
            close();
            path_ = std::move(other.path_);
            rw_ = other.rw_;
            handle_ = other.handle_;
            size_ = other.size_;
            lba_shift_ = other.lba_shift_;
            write_offset_ = other.write_offset_;
            sptdwb_ = other.sptdwb_;
            fua_ = other.fua_;
            other.handle_ = INVALID_HANDLE_VALUE;
        }
        return *this;
    }

    void Disk::close()
    {
        if (handle_ != INVALID_HANDLE_VALUE)
        {
            CloseHandle(handle_);
            handle_ = INVALID_HANDLE_VALUE;
        }
    }

    size_t Disk::get_size() const { return size_; }
    HANDLE Disk::get_handle() const { return handle_; }
    const std::string &Disk::get_path() const { return path_; }

    void Disk::scsi_open(const std::string &path)
    {
        close();
        handle_ = open_handle(path, rw_);
        if (handle_ == INVALID_HANDLE_VALUE)
        {
            throw std::system_error(last_error(), std::system_category(), "Failed to open SCSI device");
        }
    }

    size_t Disk::scsi_pass_through_direct()
    {
        return ioctl_wrapper(handle_, IOCTL_SCSI_PASS_THROUGH_DIRECT, &sptdwb_, sizeof(sptdwb_), &sptdwb_, sizeof(sptdwb_));
    }

    uint8_t Disk::get_scsi_address()
    {
        SCSI_ADDRESS scsi_addr = {};
        ioctl_wrapper(handle_, IOCTL_SCSI_GET_ADDRESS, nullptr, 0, &scsi_addr, sizeof(scsi_addr));
        std::cout << "get_scsi_address: port " << scsi_addr.PortNumber << " bus " << scsi_addr.PathId << std::endl;
        return scsi_addr.PathId;
    }

    void Disk::get_cache_information()
    {
        DISK_CACHE_INFORMATION cache_info = {};
        ioctl_wrapper(handle_, IOCTL_DISK_GET_CACHE_INFORMATION, nullptr, 0, &cache_info, sizeof(cache_info));
        std::cout << "get_cache_information: WriteCacheEnabled = " << (cache_info.WriteCacheEnabled ? "true" : "false") << std::endl;
    }

    void Disk::storage_query_property()
    {
        STORAGE_PROPERTY_QUERY spq = {};
        spq.PropertyId = StorageDeviceWriteCacheProperty;
        spq.QueryType = PropertyStandardQuery;
        STORAGE_WRITE_CACHE_PROPERTY cache_prop = {};
        ioctl_wrapper(handle_, IOCTL_STORAGE_QUERY_PROPERTY, &spq, sizeof(spq), &cache_prop, sizeof(cache_prop));
        std::cout << "storage_query_property: WriteCacheEnabled = " << static_cast<int>(cache_prop.WriteCacheEnabled) << std::endl;
    }

    size_t Disk::security_recv(uint8_t protocol, uint16_t com_id, std::span<uint8_t> buf)
    {
        scsi::ScsiSecCdb12 cdb(scsi::ScsiOpcode::SECURITY_RECV, protocol, com_id, static_cast<uint32_t>(buf.size()));
        sptdwb_.set_buffer(SCSI_IOCTL_DATA_IN, buf.data(), buf.size());
        sptdwb_.sptd.CdbLength = 12;
        memcpy(sptdwb_.sptd.Cdb, &cdb, sizeof(cdb));

        return scsi_pass_through_direct();
    }

    size_t Disk::security_send(uint8_t protocol, uint16_t com_id, std::span<const uint8_t> buf)
    {
        scsi::ScsiSecCdb12 cdb(scsi::ScsiOpcode::SECURITY_SEND, protocol, com_id, static_cast<uint32_t>(buf.size()));
        sptdwb_.set_buffer(SCSI_IOCTL_DATA_OUT, std::span<const uint8_t>(buf));
        sptdwb_.sptd.CdbLength = 12;
        memcpy(sptdwb_.sptd.Cdb, &cdb, sizeof(cdb));

        return scsi_pass_through_direct();
    }

    size_t Disk::discovery0()
    {
        std::vector<uint8_t> buff(4096, 0);
        auto res = security_recv(0x01, 0x0001, buff);
        // Maybe print just a portion
        std::cout << "discovery0 received " << res << " bytes" << std::endl;
        return res;
    }

    void Disk::storage_set_property()
    {
        // Caution: This is a complex operation. The Rust code is also incomplete.
        // This is a placeholder showing the IOCTL call.
        STORAGE_PROPERTY_SET sps = {};
        sps.PropertyId = StorageDeviceWriteCacheProperty;
        sps.SetType = PropertyStandardSet;
        // The buffer to set follows the sps structure in memory.
        // This is a simplified and likely incorrect version.
        char buffer[sizeof(STORAGE_PROPERTY_SET) + sizeof(STORAGE_WRITE_CACHE_PROPERTY)] = {};
        memcpy(buffer, &sps, sizeof(sps));

        STORAGE_WRITE_CACHE_PROPERTY cache_prop_to_set = {}; // Set properties here
        memcpy(buffer + sizeof(STORAGE_PROPERTY_SET), &cache_prop_to_set, sizeof(cache_prop_to_set));

        ioctl_wrapper(handle_, IOCTL_STORAGE_SET_PROPERTY, buffer, sizeof(buffer), nullptr, 0);
        std::cout << "storage_set_property called." << std::endl;
    }

    size_t Disk::scsi_read(uint64_t offset, std::span<uint8_t> buf)
    {
        if (buf.empty())
            return 0;

        size_t len = (buf.size() + SECTOR_SIZE - 1) & ~(SECTOR_SIZE - 1);
        if (len > buf.size())
            len = buf.size(); // Do not overflow buffer

        uint64_t lba = offset >> lba_shift_;
        uint32_t nlb = static_cast<uint32_t>(len >> lba_shift_);

        scsi::ScsiRwCdb16 cdb(scsi::ScsiOpcode::READ_16, lba, nlb, 0);
        sptdwb_.set_buffer(SCSI_IOCTL_DATA_IN, buf.data(), len);
        sptdwb_.sptd.CdbLength = 16;
        memcpy(sptdwb_.sptd.Cdb, &cdb, sizeof(cdb));

        return scsi_pass_through_direct();
    }

    size_t Disk::scsi_write(std::span<const uint8_t> buf)
    {
        if (buf.empty())
            return 0;
        size_t len = (buf.size() + SECTOR_SIZE - 1) & ~(SECTOR_SIZE - 1);
        if (len > buf.size())
            len = buf.size();

        uint64_t lba = write_offset_ >> lba_shift_;
        uint32_t nlb = static_cast<uint32_t>(len >> lba_shift_);

        uint8_t flag = 0;
        if (fua_.has_value() && fua_.value())
        {
            flag = static_cast<uint8_t>(scsi::ScsiCdbFlag::FUA);
        }

        scsi::ScsiRwCdb16 cdb(scsi::ScsiOpcode::WRITE_16, lba, nlb, flag);
        sptdwb_.set_buffer(SCSI_IOCTL_DATA_OUT, const_cast<uint8_t *>(buf.data()), len);
        sptdwb_.sptd.CdbLength = 16;
        memcpy(sptdwb_.sptd.Cdb, &cdb, sizeof(cdb));

        scsi_pass_through_direct();
        write_offset_ += sptdwb_.sptd.DataTransferLength;
        return sptdwb_.sptd.DataTransferLength;
    }

    size_t Disk::read(std::span<uint8_t> buf)
    {
        if (buf.empty())
        {
            return 0;
        }
        size_t len = (buf.size() + SECTOR_SIZE - 1) & ~(SECTOR_SIZE - 1);
        void *aligned_buf = _aligned_malloc(len, SECTOR_SIZE);
        if (!aligned_buf)
        {
            throw std::bad_alloc();
        }

        DWORD bytes_read = 0;
        BOOL res = ReadFile(handle_, aligned_buf, static_cast<DWORD>(len), &bytes_read, nullptr);
        if (!res)
        {
            _aligned_free(aligned_buf);
            throw std::system_error(last_error(), std::system_category(), "ReadFile failed");
        }
        memcpy(buf.data(), aligned_buf, std::min(buf.size(), static_cast<size_t>(bytes_read)));
        _aligned_free(aligned_buf);
        return bytes_read;
    }

    size_t Disk::write(std::span<const uint8_t> buf)
    {
        if (buf.empty())
            return 0;

        size_t len = (buf.size() + SECTOR_SIZE - 1) & ~(SECTOR_SIZE - 1);
        void *aligned_buf = _aligned_malloc(len, SECTOR_SIZE);
        if (!aligned_buf)
        {
            throw std::bad_alloc();
        }
        memset(aligned_buf, 0, len);
        memcpy(aligned_buf, buf.data(), buf.size());

        DWORD bytes_written = 0;
        BOOL res = WriteFile(handle_, aligned_buf, static_cast<DWORD>(len), &bytes_written, nullptr);
        _aligned_free(aligned_buf);
        if (!res)
        {
            throw std::system_error(last_error(), std::system_category(), "WriteFile failed");
        }
        return bytes_written;
    }

    void Disk::flush()
    {
        // The Rust implementation is empty, so this is too.
        // Could call FlushFileBuffers(handle_);
    }

    std::ostream &operator<<(std::ostream &os, const Disk &disk)
    {
        os << "Disk path: \"" << disk.get_path() << "\", handle: " << disk.get_handle()
           << ", size: " << disk.get_size();
        return os;
    }

    int get_physical_drv_number_from_logical_drv(const std::string &drive_name)
    {
        std::string path = "\\\\.\\" + drive_name;
        HANDLE h_device = open_handle(path, 'r');
        if (h_device == INVALID_HANDLE_VALUE)
        {
            return -1;
        }

        int disk_number = -1;
        VOLUME_DISK_EXTENTS extents = {};
        try
        {
            ioctl_wrapper(h_device, IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS, nullptr, 0, &extents, sizeof(extents));
            disk_number = extents.Extents[0].DiskNumber;
        }
        catch (const std::system_error &e)
        {
            std::cerr << "ioctl_wrapper failed for get_physical_drv_number_from_logical_drv: " << e.what() << std::endl;
        }

        CloseHandle(h_device);
        return disk_number;
    }

} // namespace disk
