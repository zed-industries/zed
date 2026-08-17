// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_PARTITION_ALLOCATOR_INSPECT_UTILS_H_
#define TOOLS_MEMORY_PARTITION_ALLOCATOR_INSPECT_UTILS_H_

// This file includes utilities used in PartitionAlloc tools, also
// found in this directory.

#include <fcntl.h>
#include <signal.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#include <optional>

#include "base/files/file.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_MAC)
#include <mach/mach.h>
#endif

namespace partition_alloc::tools {

// SIGSTOPs a process.
class ScopedSigStopper {
 public:
  explicit ScopedSigStopper(pid_t pid) : pid_(pid) { kill(pid_, SIGSTOP); }
  ~ScopedSigStopper() { kill(pid_, SIGCONT); }

 private:
  const pid_t pid_;
};

base::ScopedFD OpenProcMem(pid_t pid);
base::ScopedFD OpenPagemap(pid_t pid);

char* CreateMappingAtAddress(uintptr_t address, size_t size);

// Reads the memory of a remote process.
class RemoteProcessMemoryReader {
 public:
  explicit RemoteProcessMemoryReader(pid_t pid);
  ~RemoteProcessMemoryReader();
  // Whether access to the remote process memory has been granted.
  bool IsValid() const;

  // Returns true for success.
  bool ReadMemory(uintptr_t remote_address, size_t size, char* buffer);
  // Reads remote process memory at the *same* address in the current
  // process. Local memory is mapped with mmap(). Returns nullptr in case of
  // failure.
  char* ReadAtSameAddressInLocalMemory(uintptr_t address, size_t size);
  pid_t pid() const { return pid_; }

 private:
  const pid_t pid_;
  bool is_valid_;

#if BUILDFLAG(IS_LINUX)
  base::ScopedFD mem_fd_;
#elif BUILDFLAG(IS_MAC)
  task_t task_;
#endif
};

uintptr_t IndexThreadCacheNeedleArray(RemoteProcessMemoryReader& reader,
                                      size_t index);

// Allows to access an object copied from remote memory "as if" it were
// local. Of course, dereferencing any pointer from within it will at best
// fault.
template <typename T>
class RawBuffer {
 public:
  RawBuffer() = default;
  const T* get() const { return reinterpret_cast<const T*>(buffer_); }
  char* get_buffer() { return buffer_; }

  static std::optional<RawBuffer<T>> ReadFromProcessMemory(
      RemoteProcessMemoryReader& reader,
      uintptr_t address) {
    RawBuffer<T> buf;
    bool ok = reader.ReadMemory(reinterpret_cast<unsigned long>(address),
                                sizeof(T), buf.get_buffer());
    if (!ok)
      return std::nullopt;

    return {buf};
  }

  static RawBuffer<T> FromData(const void* data) {
    RawBuffer<T> ret;
    memcpy(ret.buffer_, data, sizeof(T));
    return ret;
  }

 private:
  alignas(T) char buffer_[sizeof(T)];
};

}  // namespace partition_alloc::tools

#endif  // TOOLS_MEMORY_PARTITION_ALLOCATOR_INSPECT_UTILS_H_
