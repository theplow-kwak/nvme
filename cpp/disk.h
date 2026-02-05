#pragma once

#define NOMINMAX
#include "scsi.h"
#include <windows.h>
#include <string>
#include <optional>
#include <iostream>
#include <span>
#include <vector>

namespace disk
{
    class Disk
    {
    public:
        // Factory method to create a Disk object
        static std::optional<Disk> open(const std::string &path, char rw, std::optional<bool> fua);

        // Destructor closes the handle
        ~Disk();

        // Move semantics
        Disk(Disk &&other) noexcept;
        Disk &operator=(Disk &&other) noexcept;

        // No copy
        Disk(const Disk &) = delete;
        Disk &operator=(const Disk &) = delete;

        // Public interface
        [[nodiscard]] size_t get_size() const;
        void scsi_open(const std::string &path);

        uint8_t get_scsi_address();
        void get_cache_information();
        void storage_query_property();
        void storage_set_property();

        size_t security_recv(uint8_t protocol, uint16_t com_id, std::span<uint8_t> buf);
        size_t security_send(uint8_t protocol, uint16_t com_id, std::span<const uint8_t> buf);
        size_t discovery0();

        size_t scsi_read(uint64_t offset, std::span<uint8_t> buf);
        size_t scsi_write(std::span<const uint8_t> buf);

        // Read/WriteFile wrappers
        size_t read(std::span<uint8_t> buf);
        size_t write(std::span<const uint8_t> buf);
        void flush();

        [[nodiscard]] HANDLE get_handle() const;
        [[nodiscard]] const std::string &get_path() const;

    private:
        // Private constructor, used by the factory
        Disk(const std::string &path, char rw, std::optional<bool> fua, HANDLE handle);

        void close();
        size_t scsi_pass_through_direct(scsi::SCSI_PASS_THROUGH_DIRECT_WITH_BUFFER &sptdwb);

        std::string path_;
        char rw_;
        HANDLE handle_;
        size_t size_;
        uint8_t lba_shift_;
        uint64_t write_offset_;
        std::optional<bool> fua_;
    };

    std::ostream &operator<<(std::ostream &os, const Disk &disk);

    int get_physical_drv_number_from_logical_drv(const std::string &drive_name);

} // namespace disk
