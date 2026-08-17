// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_CONTIGUOUS_MEMORY_HOLDER_H_
#define TOOLS_MEMORY_SIMULATOR_CONTIGUOUS_MEMORY_HOLDER_H_

#include <cstddef>
#include <cstdint>

#include "tools/memory/simulator/memory_holder.h"

namespace memory_simulator {

// A class that holds memory pages at contiguous virtual addresses.
class ContiguousMemoryHolder : public MemoryHolder {
 public:
  // `max_pages` is the maximum number of pages that can be allocated through
  // this object (exceeding it will result in a crash).
  explicit ContiguousMemoryHolder(size_t max_pages);
  ~ContiguousMemoryHolder() override;

  // MemoryHolder:
  void Allocate() override;
  void Read() override;
  void Write() override;

 private:
  const size_t memory_length_;
  const uintptr_t memory_;

  uint64_t* alloc_position_ = nullptr;
  uint64_t* read_position_ = nullptr;
  uint64_t* write_position_ = nullptr;
};

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_CONTIGUOUS_MEMORY_HOLDER_H_
