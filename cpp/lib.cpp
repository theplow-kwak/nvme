#include "lib.h"
#include <conio.h> // For _kbhit, _getch
#include <chrono>
#include <thread>
#include <optional>
#include <fstream>
#include <vector>
#include <string>
#include <algorithm>
#include <iostream>

namespace nvme_lib
{
    // This can be set via build flags if needed, but for now, a placeholder is fine.
    const char *VERSION = "0.1.0-converted";

    std::optional<char> getch(uint64_t secs_timeout)
    {
        auto start = HighResClock::now();
        while (std::chrono::duration_cast<std::chrono::seconds>(HighResClock::now() - start).count() < secs_timeout)
        {
            if (_kbhit())
            {
                char c = _getch();
                if (c == '\r')
                {
                    return '\n'; // Normalize Enter key
                }
                return c;
            }
            // Sleep for a short duration to avoid pegging the CPU
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
        return std::nullopt;
    }

    DiskLatency::DiskLatency(size_t data_size)
        : start_(HighResClock::now()), end_(start_), size_(data_size) {}

    void DiskLatency::end()
    {
        end_ = HighResClock::now();
    }

    TimePoint DiskLatency::get_start() const { return start_; }
    TimePoint DiskLatency::get_end() const { return end_; }
    size_t DiskLatency::get_size() const { return size_; }

    Duration DiskLatency::elapsed() const
    {
        // The original rust code returns duration from start to now, not start to end
        return HighResClock::now() - start_;
    }

    void save_trace(const std::string &filename, std::vector<TraceEvent> &events, TimePoint start_time)
    {
        std::sort(events.begin(), events.end(), [](const auto &a, const auto &b)
                  { return a.start < b.start; });

        std::ofstream outfile(filename);
        if (!outfile.is_open())
        {
            std::cerr << "Failed to open trace file: " << filename << std::endl;
            return;
        }

        outfile << "io_type,start,end,latency\n";
        for (const auto &val : events)
        {
            auto start_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(val.start - start_time).count();
            auto end_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(val.end - start_time).count();
            outfile << val.name << "," << start_ns << "," << end_ns << "," << (end_ns - start_ns) << "\n";
        }
    }

} // namespace nvme_lib
