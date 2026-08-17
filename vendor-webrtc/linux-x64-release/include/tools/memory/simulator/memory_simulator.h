// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_MEMORY_SIMULATOR_H_
#define TOOLS_MEMORY_SIMULATOR_MEMORY_SIMULATOR_H_

#include <stdint.h>
#include <cstdint>
#include <memory>

#include "base/rand_util.h"
#include "base/time/time.h"
#include "base/timer/timer.h"
#include "tools/memory/simulator/memory_holder.h"

namespace memory_simulator {

// Allocates, reads and writes memory pages at a configurable frequency.
class MemorySimulator {
 public:
  MemorySimulator();
  ~MemorySimulator();

  void Start(int64_t page_alloc_per_second,
             int64_t page_read_per_second,
             int64_t page_write_per_second,
             int64_t max_pages_allocated,
             base::TimeTicks read_deadline,
             base::TimeTicks write_deadline);

  void StopAndFree();

  int64_t GetPagesAllocated() const;
  int64_t GetPagesRead() const;
  int64_t GetPagesWritten() const;

 private:
  void Allocate();
  void Read();
  void Write();

  std::unique_ptr<MemoryHolder> memory_holder_;
  int64_t pages_allocated_ = 0;
  int64_t max_pages_allocated_ = 0;
  int64_t pages_read_ = 0;
  int64_t pages_written_ = 0;

  base::MetronomeTimer alloc_timer_;
  base::MetronomeTimer read_timer_;
  base::MetronomeTimer write_timer_;

  base::TimeTicks read_deadline_;
  base::TimeTicks write_deadline_;
};

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_MEMORY_SIMULATOR_H_
