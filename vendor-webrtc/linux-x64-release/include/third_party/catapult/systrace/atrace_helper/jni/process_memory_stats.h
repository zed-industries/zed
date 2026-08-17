// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PROCESS_MEMORY_STATS_H_
#define PROCESS_MEMORY_STATS_H_

#include <stdint.h>

#include <memory>
#include <vector>

// Reads process memory stats from /proc/pid/{statm,smaps}.
class ProcessMemoryStats {
 public:
  struct MmapInfo {
    char mapped_file[128] = {};
    char prot_flags[5] = {};
    uint64_t start_addr = 0;
    uint64_t end_addr = 0;
    uint64_t virt_kb = 0;
    uint64_t pss_kb = 0;  // Proportional Set Size.
    uint64_t rss_kb = 0;  // Resident Set Size.
    uint64_t private_clean_kb = 0;
    uint64_t private_dirty_kb = 0;
    uint64_t shared_clean_kb = 0;
    uint64_t shared_dirty_kb = 0;
    uint64_t swapped_kb = 0;
  };

  ProcessMemoryStats() {}

  bool ReadLightStats(int pid);
  bool ReadFullStats(int pid);
  bool ReadGpuStats(int pid);

  // Available after ReadLightStats().
  uint64_t virt_kb() const { return virt_kb_; }
  uint64_t rss_kb() const { return rss_kb_; }

  // Available after ReadFullStats().
  bool full_stats_available() const { return full_stats_; }
  uint64_t pss_kb() const { return pss_kb_; }
  uint64_t private_clean_kb() const { return private_clean_kb_; }
  uint64_t private_dirty_kb() const { return private_dirty_kb_; }
  uint64_t shared_clean_kb() const { return shared_clean_kb_; }
  uint64_t shared_dirty_kb() const { return shared_dirty_kb_; }
  uint64_t swapped_kb() const { return swapped_kb_; }

  // Available after ReadMemtrackStats().
  bool gpu_stats_available() const { return gpu_stats_; }
  uint64_t gpu_graphics_kb() const { return gpu_graphics_kb_; }
  uint64_t gpu_graphics_pss_kb() const { return gpu_graphics_pss_kb_; }
  uint64_t gpu_gl_kb() const { return gpu_gl_kb_; }
  uint64_t gpu_gl_pss_kb() const { return gpu_gl_pss_kb_; }
  uint64_t gpu_other_kb() const { return gpu_other_kb_; }
  uint64_t gpu_other_pss_kb() const { return gpu_other_pss_kb_; }

  size_t mmaps_count() const { return mmaps_.size(); }
  const MmapInfo* mmap(size_t index) const { return mmaps_[index].get(); }

 private:
  ProcessMemoryStats(const ProcessMemoryStats&) = delete;
  void operator=(const ProcessMemoryStats&) = delete;

  // Light stats.
  uint64_t virt_kb_ = 0;
  uint64_t rss_kb_ = 0;

  // Full stats.
  bool full_stats_ = false;
  uint64_t pss_kb_ = 0;
  uint64_t private_clean_kb_ = 0;
  uint64_t private_dirty_kb_ = 0;
  uint64_t shared_clean_kb_ = 0;
  uint64_t shared_dirty_kb_ = 0;
  uint64_t swapped_kb_ = 0;

  // Graphics stats.
  bool gpu_stats_ = false;
  uint64_t gpu_graphics_kb_ = 0;
  uint64_t gpu_graphics_pss_kb_ = 0;
  uint64_t gpu_gl_kb_ = 0;
  uint64_t gpu_gl_pss_kb_ = 0;
  uint64_t gpu_other_kb_ = 0;
  uint64_t gpu_other_pss_kb_ = 0;

  std::vector<std::unique_ptr<const MmapInfo>> mmaps_;
};

#endif  // PROCESS_MEMORY_STATS_H_
