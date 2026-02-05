#pragma once

#include <cstdint>
#include <chrono>
#include <vector>
#include <string>
#include <tuple>
#include <optional>

namespace nvme_lib
{
    // --- Constants ---
    constexpr bool RUNNING = true;
    constexpr bool ENDLOOP = false;

    constexpr size_t MI_BYTES = 1024 * 1024;
    constexpr size_t SECTOR_SIZE = 512;
    constexpr size_t CHUNK_SIZE = 512 * SECTOR_SIZE;
    constexpr size_t READ_CHUNK = 16 * CHUNK_SIZE;
    constexpr size_t MAX_BUFFER_SIZE = 128 * CHUNK_SIZE;
    constexpr size_t MAX_READ_PIPE = 1024 * MI_BYTES;
    constexpr size_t MAX_WRITE_PIPE = 128 * MI_BYTES;

    constexpr uint32_t BLOCK_SIZE = 1456;
    constexpr size_t UDP_PACK_SIZE = 2048;

    constexpr uint32_t MAX_CLIENTS = 128;
    constexpr uint32_t MAX_SLICE_SIZE = 2048;
    constexpr uint32_t BITS_PER_CHAR = 8;

    constexpr uint32_t CAP_NEW_GEN = 0x0001;
    constexpr uint32_t CAP_BIG_ENDIAN = 0x0008;
    constexpr uint32_t CAP_LITTLE_ENDIAN = 0x0010;
    constexpr uint32_t CAP_ASYNC = 0x0020;
    constexpr uint32_t SENDER_CAPABILITIES = CAP_NEW_GEN | CAP_BIG_ENDIAN;
    constexpr uint32_t RECEIVER_CAPABILITIES = CAP_NEW_GEN | CAP_BIG_ENDIAN;

    constexpr uint16_t FLAG_PASSIVE = 0x0010;
    constexpr uint16_t FLAG_NOSYNC = 0x0040;
    constexpr uint16_t FLAG_NOKBD = 0x0080;
    constexpr uint16_t FLAG_SYNC = 0x0100;
    constexpr uint16_t FLAG_STREAMING = 0x0200;
    constexpr uint16_t FLAG_IGNORE_LOST_DATA = 0x400;

    constexpr uint16_t PORTBASE = 9000;

    // The version will be defined in the .cpp file as it's a string literal from build env
    extern const char *VERSION;

    // --- Functions ---
    // Returns a character if a key is pressed within the timeout, otherwise std::nullopt
    std::optional<char> getch(uint64_t secs_timeout);

    // --- Classes ---
    using HighResClock = std::chrono::high_resolution_clock;
    using TimePoint = std::chrono::time_point<HighResClock>;
    using Duration = std::chrono::duration<double>;

    class DiskLatency
    {
    public:
        DiskLatency(size_t data_size);

        void end();

        // Getters
        TimePoint get_start() const;
        TimePoint get_end() const;
        size_t get_size() const;
        Duration elapsed() const;

    private:
        TimePoint start_;
        TimePoint end_;
        size_t size_;
    };

    // Using a struct for the trace event for clarity
    struct TraceEvent
    {
        std::string name;
        TimePoint start;
        TimePoint end;
    };

    void save_trace(const std::string &filename, std::vector<TraceEvent> &events, TimePoint start_time);

} // namespace nvme_lib
