// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_MEMORY_HOLDER_H_
#define TOOLS_MEMORY_SIMULATOR_MEMORY_HOLDER_H_

#include <stdint.h>

#include "base/rand_util.h"

namespace memory_simulator {

// Interface for a class that supports these operations:
// - Allocate a new memory page
// - Read the next memory page
// - Write the next memory page
class MemoryHolder {
 public:
  MemoryHolder();
  virtual ~MemoryHolder();

  // Add a page filled with zeros to the "private footprint" of this process.
  virtual void Allocate() = 0;

  // Read the next memory page. When the end is reached, goes back to the first
  // page. No-ops if no page was allocated yet.
  virtual void Read() = 0;

  // Writes to the next memory page. When the end is reached, goes back to the
  // first page. No-ops if no page was allocated yet.
  virtual void Write() = 0;

 protected:
  // Returns a random number for use by Write() implementations.
  uint64_t Rand();

 private:
  base::InsecureRandomGenerator random_generator_;
};

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_MEMORY_HOLDER_H_
