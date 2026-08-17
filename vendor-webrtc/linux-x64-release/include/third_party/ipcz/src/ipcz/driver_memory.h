// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_DRIVER_MEMORY_H_
#define IPCZ_SRC_IPCZ_DRIVER_MEMORY_H_

#include <cstddef>

#include "ipcz/driver_memory_mapping.h"
#include "ipcz/driver_object.h"
#include "ipcz/ipcz.h"
#include "util/ref_counted.h"

namespace ipcz {

// Scoped wrapper around a shared memory region allocated and manipulated
// through an ipcz driver.
class DriverMemory {
 public:
  DriverMemory();

  // Takes ownership of an existing driver memory object.
  explicit DriverMemory(DriverObject memory);

  // Asks the node to allocate a new driver shared memory region of at least
  // `num_bytes` in size.
  DriverMemory(const IpczDriver& driver, size_t num_bytes);

  DriverMemory(DriverMemory&& other);
  DriverMemory& operator=(DriverMemory&& other);

  ~DriverMemory();

  bool is_valid() const { return memory_.is_valid(); }
  size_t size() const { return size_; }

  DriverObject& driver_object() { return memory_; }

  DriverObject TakeDriverObject() { return std::move(memory_); }

  // Asks the driver to clone this memory object and return a new one which
  // references the same underlying memory region.
  DriverMemory Clone();

  // Asks the driver to map this memory object into the process's address space
  // and returns a scoper to control the mapping's lifetime. Returns an invalid
  // mapping if mapping fails or if called on an invalid DriverMemory.
  DriverMemoryMapping Map();

 private:
  DriverObject memory_;
  size_t size_ = 0;
};

// This pairs a DriverMemory object with a mapping of that same object, for
// convenience.
struct DriverMemoryWithMapping {
  DriverMemoryWithMapping();
  DriverMemoryWithMapping(DriverMemory memory, DriverMemoryMapping mapping);
  DriverMemoryWithMapping(DriverMemoryWithMapping&&);
  DriverMemoryWithMapping& operator=(DriverMemoryWithMapping&&);
  ~DriverMemoryWithMapping();

  DriverMemory memory;
  DriverMemoryMapping mapping;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_DRIVER_MEMORY_H_
