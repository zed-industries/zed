// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_NATIVE_UNWINDER_ANDROID_MEMORY_REGIONS_MAP_IMPL_H_
#define BASE_PROFILER_NATIVE_UNWINDER_ANDROID_MEMORY_REGIONS_MAP_IMPL_H_

#include "base/profiler/native_unwinder_android_memory_regions_map.h"
#include "third_party/libunwindstack/src/libunwindstack/include/unwindstack/Maps.h"
#include "third_party/libunwindstack/src/libunwindstack/include/unwindstack/Memory.h"

namespace base {

class NativeUnwinderAndroidMemoryRegionsMapImpl
    : public NativeUnwinderAndroidMemoryRegionsMap {
 public:
  NativeUnwinderAndroidMemoryRegionsMapImpl(
      std::unique_ptr<unwindstack::Maps> maps,
      std::unique_ptr<unwindstack::Memory> memory);

  ~NativeUnwinderAndroidMemoryRegionsMapImpl() override;

  unwindstack::Maps* maps() { return maps_.get(); }
  // We use a non-const reference here because some functions in libunwindstack
  // expect that.
  std::shared_ptr<unwindstack::Memory>& memory() { return memory_; }

  void SetMapsForTesting(std::unique_ptr<unwindstack::Maps> maps) {
    maps_ = std::move(maps);
  }

 private:
  std::unique_ptr<unwindstack::Maps> maps_;
  std::shared_ptr<unwindstack::Memory> memory_;
};

}  // namespace base

#endif  // BASE_PROFILER_NATIVE_UNWINDER_ANDROID_MEMORY_REGIONS_MAP_IMPL_H_
