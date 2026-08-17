// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SHARED_MEMORY_HOOKS_H_
#define BASE_MEMORY_SHARED_MEMORY_HOOKS_H_

#include "base/memory/read_only_shared_memory_region.h"
#include "base/memory/unsafe_shared_memory_region.h"
#include "base/memory/writable_shared_memory_region.h"

namespace mojo {
class SharedMemoryUtils;
namespace core::ipcz_driver {
class BaseSharedMemoryService;
}
}  // namespace mojo

namespace base {

class SharedMemoryHooks {
 public:
  SharedMemoryHooks() = delete;

 private:
  friend class SharedMemoryHooksTest;
  friend mojo::SharedMemoryUtils;
  friend class mojo::core::ipcz_driver::BaseSharedMemoryService;

  // Allows shared memory region creation to be hooked. Useful for sandboxed
  // processes that are restricted from invoking the platform APIs directly.
  // Intentionally private so callers need to be explicitly friended.
  static void SetCreateHooks(
      ReadOnlySharedMemoryRegion::CreateFunction* read_only_hook,
      UnsafeSharedMemoryRegion::CreateFunction* unsafe_hook,
      WritableSharedMemoryRegion::CreateFunction* writable_hook) {
    ReadOnlySharedMemoryRegion::set_create_hook(read_only_hook);
    UnsafeSharedMemoryRegion::set_create_hook(unsafe_hook);
    WritableSharedMemoryRegion::set_create_hook(writable_hook);
  }
};

}  // namespace base

#endif  // BASE_MEMORY_SHARED_MEMORY_HOOKS_H_
