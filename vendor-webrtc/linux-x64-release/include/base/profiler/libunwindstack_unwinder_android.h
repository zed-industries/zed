// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_LIBUNWINDSTACK_UNWINDER_ANDROID_H_
#define BASE_PROFILER_LIBUNWINDSTACK_UNWINDER_ANDROID_H_

#include <memory>
#include <vector>

#include "base/base_export.h"
#include "base/profiler/native_unwinder_android_memory_regions_map_impl.h"
#include "base/profiler/unwinder.h"
#include "third_party/libunwindstack/src/libunwindstack/include/unwindstack/DexFiles.h"
#include "third_party/libunwindstack/src/libunwindstack/include/unwindstack/JitDebug.h"
#include "third_party/libunwindstack/src/libunwindstack/include/unwindstack/Memory.h"

namespace base {

// This unwinder uses the libunwindstack::Unwinder internally to provide the
// base::Unwinder implementation. This is in contrast to
// base::NativeUnwinderAndroid, which uses functions from libunwindstack
// selectively to provide a subset of libunwindstack::Unwinder features. This
// causes some divergences from other base::Unwinder (this unwinder either fully
// succeeds or fully fails). A good source for a compariative unwinder would be
// traced_perf or heapprofd on android which uses the same API.
class BASE_EXPORT LibunwindstackUnwinderAndroid : public Unwinder {
 public:
  LibunwindstackUnwinderAndroid();
  ~LibunwindstackUnwinderAndroid() override;

  LibunwindstackUnwinderAndroid(const LibunwindstackUnwinderAndroid&) = delete;
  LibunwindstackUnwinderAndroid& operator=(
      const LibunwindstackUnwinderAndroid&) = delete;

  // Unwinder
  void InitializeModules() override;
  bool CanUnwindFrom(const Frame& current_frame) const override;
  UnwindResult TryUnwind(UnwinderStateCapture* capture_state,
                         RegisterContext* thread_context,
                         uintptr_t stack_top,
                         std::vector<Frame>* stack) override;

 private:
  unwindstack::JitDebug* GetOrCreateJitDebug(unwindstack::ArchEnum arch);
  unwindstack::DexFiles* GetOrCreateDexFiles(unwindstack::ArchEnum arch);

  std::unique_ptr<NativeUnwinderAndroidMemoryRegionsMapImpl>
      memory_regions_map_;

  std::unique_ptr<unwindstack::JitDebug> jit_debug_;
  std::unique_ptr<unwindstack::DexFiles> dex_files_;
  // Libraries where to search for dex and jit descriptors.
  const std::vector<std::string> search_libs_ = {"libart.so", "libartd.so"};
};
}  // namespace base

#endif  // BASE_PROFILER_LIBUNWINDSTACK_UNWINDER_ANDROID_H_
