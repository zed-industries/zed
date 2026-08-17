// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_MEMORY_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_MEMORY_H_

#include <stdint.h>

#include "reference_drivers/file_descriptor.h"
#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz::reference_drivers {

// A basic driver memory implementation for ipcz, which uses Linux-specific
// support for an anonymous file living entirely in RAM (see memfd_create()).
class MemfdMemory {
 public:
  // An mmap'd region of a MemfdMemory's underlying file. Instances of this
  // object should be acquired from MemfdMemory::Map().
  class Mapping {
   public:
    Mapping();
    Mapping(void* base_address, size_t size);
    Mapping(Mapping&&);
    Mapping& operator=(Mapping&&);
    Mapping(const Mapping&) = delete;
    Mapping& operator=(const Mapping&) = delete;
    ~Mapping();

    bool is_valid() const { return base_address_ != nullptr; }

    size_t size() const { return size_; }
    void* base() const { return base_address_; }

    absl::Span<uint8_t> bytes() const {
      return {static_cast<uint8_t*>(base()), size_};
    }

    template <typename T>
    T* As() const {
      return static_cast<T*>(base());
    }

    void Reset();

   private:
    void* base_address_ = nullptr;
    size_t size_ = 0;
  };

  // Constructs an invalid MemfdMemory object which cannot be mapped.
  MemfdMemory();

  // Constructs a new MemfdMemory object over `descriptor`, a file descriptor
  // which should have been previously taken from some other valid MemfdMemory
  // object or otherwise acquried via memfd_create(). `size` must correspond to
  // the size of the underlying memory file.
  MemfdMemory(FileDescriptor fd, size_t size);

  // Constructs a new MemfdMemory object over a newly allocated, anonymous
  // shared memory file of at least `size` bytes.
  explicit MemfdMemory(size_t size);

  MemfdMemory(MemfdMemory&&);
  MemfdMemory& operator=(MemfdMemory&&);
  MemfdMemory(const MemfdMemory&) = delete;
  MemfdMemory& operator=(const MemfdMemory&) = delete;
  ~MemfdMemory();

  size_t size() const { return size_; }
  bool is_valid() const { return fd_.is_valid(); }
  const FileDescriptor& descriptor() const { return fd_; }

  // Invalidates this object and returns a FileDescrtiptor which can be used
  // later to reconstruct an equivalent MemfdMemory object with the same size().
  FileDescriptor TakeDescriptor() { return std::move(fd_); }

  // Resets this object, closing its underlying memory file descriptor.
  void reset() { fd_.reset(); }

  // Returns a new MemfdMemory object with its own handle to the same underlying
  // memory file as `this`. Must only be called on a valid MemfdMemory object
  // (i.e. is_valid() must be true.)
  MemfdMemory Clone();

  // Maps the entire file owned by this object and returns a Mapping for it.
  // Must only be called on a valid MemfdMemory object (i.e. is_valid() must be
  // true.)
  Mapping Map();

 private:
  FileDescriptor fd_;
  size_t size_ = 0;
};

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_MEMORY_H_
