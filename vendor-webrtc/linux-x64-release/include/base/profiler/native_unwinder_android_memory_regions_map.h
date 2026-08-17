// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_NATIVE_UNWINDER_ANDROID_MEMORY_REGIONS_MAP_H_
#define BASE_PROFILER_NATIVE_UNWINDER_ANDROID_MEMORY_REGIONS_MAP_H_

namespace base {

// NativeUnwinderAndroidMemoryRegionsMap is an opaque interface that hides
// concrete libunwindstack types, i.e. `unwindstack::Maps` and
// `unwindstack::Memory`. By introducing the interface, chrome code can
// pass the underlying instances around without referencing libunwindstack.
// NativeUnwinderAndroidMemoryRegionsMap's implementation must live in the
// stack_unwinder dynamic feature module.
//
// Code within the dynamic feature module is expected to downcast to the
// derived type to access the unwindstack types.
class NativeUnwinderAndroidMemoryRegionsMap {
 public:
  NativeUnwinderAndroidMemoryRegionsMap() = default;
  virtual ~NativeUnwinderAndroidMemoryRegionsMap() = default;

  NativeUnwinderAndroidMemoryRegionsMap(
      const NativeUnwinderAndroidMemoryRegionsMap&) = delete;
  NativeUnwinderAndroidMemoryRegionsMap& operator=(
      const NativeUnwinderAndroidMemoryRegionsMap&) = delete;
};

}  // namespace base

#endif  // BASE_PROFILER_NATIVE_UNWINDER_ANDROID_MEMORY_REGIONS_MAP_H_
