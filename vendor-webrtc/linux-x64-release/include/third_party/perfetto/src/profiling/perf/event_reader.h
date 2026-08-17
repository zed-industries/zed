/*
 * Copyright (C) 2019 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_PROFILING_PERF_EVENT_READER_H_
#define SRC_PROFILING_PERF_EVENT_READER_H_

#include <linux/perf_event.h>
#include <stdint.h>
#include <sys/mman.h>
#include <sys/types.h>
#include <optional>

#include "perfetto/ext/base/scoped_file.h"
#include "src/profiling/perf/common_types.h"
#include "src/profiling/perf/event_config.h"

namespace perfetto {
namespace profiling {

class PerfRingBuffer {
 public:
  static std::optional<PerfRingBuffer> Allocate(int perf_fd,
                                                size_t data_page_count);

  ~PerfRingBuffer();

  // move-only
  PerfRingBuffer(const PerfRingBuffer&) = delete;
  PerfRingBuffer& operator=(const PerfRingBuffer&) = delete;
  PerfRingBuffer(PerfRingBuffer&& other) noexcept;
  PerfRingBuffer& operator=(PerfRingBuffer&& other) noexcept;

  char* ReadRecordNonconsuming();
  void Consume(size_t bytes);

 private:
  PerfRingBuffer() = default;

  bool valid() const { return metadata_page_ != nullptr; }

  // Points at the start of the mmap'd region.
  perf_event_mmap_page* metadata_page_ = nullptr;

  // Size of the mmap'd region (1 metadata page + data_buf_sz_).
  size_t mmap_sz_ = 0;

  // mmap'd ring buffer
  char* data_buf_ = nullptr;
  size_t data_buf_sz_ = 0;

  // When a record wraps around the ring buffer boundary, it is reconstructed in
  // a contiguous form in this buffer. This allows us to always return a pointer
  // to a contiguous record.
  constexpr static size_t kMaxPerfRecordSize = 1 << 16;  // max size 64k
  alignas(uint64_t) char reconstructed_record_[kMaxPerfRecordSize];
};

class EventReader {
 public:
  static std::optional<EventReader> ConfigureEvents(
      uint32_t cpu,
      const EventConfig& event_cfg);

  // Snapshots the counter values using the |read| syscall.
  // The sample will always be timestamped ourselves, using CLOCK_BOOTTIME.
  // Note: compatible with ring buffer mode, but not used in practice.
  std::optional<CommonSampleData> ReadCounters();

  // Consumes records from the ring buffer until either encountering a sample,
  // or catching up to the writer. The other record of interest
  // (PERF_RECORD_LOST) is handled via the given callback.
  //
  // Valid only if using the ring buffer, i.e. the EventReader was constructed
  // with EventConfig::RecordingMode::kSampling option.
  std::optional<ParsedSample> ReadUntilSample(
      std::function<void(uint64_t)> lost_events_callback);

  void EnableEvents();
  // Pauses the event counting, without invalidating existing samples.
  void DisableEvents();

  uint32_t cpu() const { return cpu_; }

  ~EventReader() = default;

  // move-only
  EventReader(const EventReader&) = delete;
  EventReader& operator=(const EventReader&) = delete;
  EventReader(EventReader&&) noexcept = default;
  EventReader& operator=(EventReader&&) noexcept;

 private:
  EventReader(uint32_t cpu,
              perf_event_attr event_attr,
              base::ScopedFile perf_fd,
              std::vector<base::ScopedFile> followers_fds,
              std::optional<PerfRingBuffer> ring_buffer);

  ParsedSample ParseSampleRecord(uint32_t cpu, const char* record_start);

  // All events are cpu-bound (thread-scoped events not supported).
  const uint32_t cpu_;
  const perf_event_attr event_attr_;
  base::ScopedFile perf_fd_;
  std::vector<base::ScopedFile> follower_fds_;
  // Ring buffer is absent if and only if we're polling counters.
  std::optional<PerfRingBuffer> ring_buffer_;
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_PERF_EVENT_READER_H_
