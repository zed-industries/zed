/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_TRACING_IPC_SHARED_MEMORY_WINDOWS_H_
#define SRC_TRACING_IPC_SHARED_MEMORY_WINDOWS_H_

#include "perfetto/base/build_config.h"

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)

#include <string>

#include "perfetto/base/build_config.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/tracing/core/shared_memory.h"

namespace perfetto {

// Implements the SharedMemory and its factory for the Windows IPC transport.
// This used only for standalone builds and NOT in chromium, which instead uses
// a custom Mojo wrapper (MojoSharedMemory in chromium's //services/tracing/).
class SharedMemoryWindows : public SharedMemory {
 public:
  class Factory : public SharedMemory::Factory {
   public:
    ~Factory() override;
    std::unique_ptr<SharedMemory> CreateSharedMemory(size_t) override;
  };

  // Create a brand new SHM region.
  enum Flags { kNone = 0, kInheritableHandles };
  static std::unique_ptr<SharedMemoryWindows> Create(
      size_t size,
      Flags flags = Flags::kNone);
  static std::unique_ptr<SharedMemoryWindows> Attach(const std::string& key);
  static std::unique_ptr<SharedMemoryWindows> AttachToHandleWithKey(
      base::ScopedPlatformHandle fd,
      const std::string& key);
  ~SharedMemoryWindows() override;
  const std::string& key() const { return key_; }
  const base::ScopedPlatformHandle& handle() const { return handle_; }

  // SharedMemory implementation.
  using SharedMemory::start;  // Equal priority to const and non-const versions
  const void* start() const override { return start_; }
  size_t size() const override { return size_; }

 private:
  SharedMemoryWindows(void* start,
                      size_t size,
                      std::string,
                      base::ScopedPlatformHandle);
  SharedMemoryWindows(const SharedMemoryWindows&) = delete;
  SharedMemoryWindows& operator=(const SharedMemoryWindows&) = delete;

  void* const start_;
  const size_t size_;
  std::string key_;
  base::ScopedPlatformHandle handle_;
};

}  // namespace perfetto

#endif  // OS_WIN

#endif  // SRC_TRACING_IPC_SHARED_MEMORY_WINDOWS_H_
