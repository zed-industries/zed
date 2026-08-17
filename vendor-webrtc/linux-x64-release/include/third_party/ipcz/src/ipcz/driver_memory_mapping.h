// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/393091624): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef IPCZ_SRC_IPCZ_DRIVER_MEMORY_MAPPING_H_
#define IPCZ_SRC_IPCZ_DRIVER_MEMORY_MAPPING_H_

#include "ipcz/ipcz.h"
#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz {

// Scoped wrapper around a driver-controlled shared memory region mapping.
class DriverMemoryMapping {
 public:
  DriverMemoryMapping();

  // Tracks the driver-produced handle and base address of an active memory
  // mapping.
  DriverMemoryMapping(const IpczDriver& driver,
                      IpczDriverHandle mapping_handle,
                      void* address,
                      size_t size);

  DriverMemoryMapping(DriverMemoryMapping&& other);
  DriverMemoryMapping(const DriverMemoryMapping&) = delete;
  DriverMemoryMapping& operator=(DriverMemoryMapping&& other);
  DriverMemoryMapping& operator=(const DriverMemoryMapping&) = delete;
  ~DriverMemoryMapping();

  bool is_valid() const { return mapping_ != IPCZ_INVALID_DRIVER_HANDLE; }

  // Returns the base address of this mapping. Returns null if the mapping is
  // invalid.
  void* address() const { return address_; }

  // Returns the address at `offset` bytes from this mapping's base address. iT
  // is an error to call this with an `offset` greater than or equal to the
  // mapped region's size.
  void* address_at(size_t offset) const {
    ABSL_ASSERT(offset < size_);
    return static_cast<uint8_t*>(address_) + offset;
  }

  absl::Span<uint8_t> bytes() const {
    return {static_cast<uint8_t*>(address_), size_};
  }

 private:
  void Unmap();

  IpczDriver driver_;
  IpczDriverHandle mapping_ = IPCZ_INVALID_DRIVER_HANDLE;
  void* address_ = nullptr;
  size_t size_ = 0;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_DRIVER_MEMORY_MAPPING_H_
