// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_NATIVE_UNWINDER_ANDROID_H_
#define BASE_PROFILER_NATIVE_UNWINDER_ANDROID_H_

#include <memory>
#include <vector>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/profiler/unwinder.h"
#include "third_party/libunwindstack/src/libunwindstack/include/unwindstack/DexFiles.h"
#include "third_party/libunwindstack/src/libunwindstack/include/unwindstack/Memory.h"

namespace base {

class NativeUnwinderAndroidMapDelegate;
class NativeUnwinderAndroidMemoryRegionsMap;
class NativeUnwinderAndroidMemoryRegionsMapImpl;

// Implementation of unwindstack::Memory that restricts memory access to a stack
// buffer, used by NativeUnwinderAndroid. While unwinding, only memory accesses
// within the stack should be performed to restore registers.
class BASE_EXPORT UnwindStackMemoryAndroid : public unwindstack::Memory {
 public:
  UnwindStackMemoryAndroid(uintptr_t stack_ptr, uintptr_t stack_top);
  ~UnwindStackMemoryAndroid() override;

  size_t Read(uint64_t addr, void* dst, size_t size) override;

 private:
  const uintptr_t stack_ptr_;
  const uintptr_t stack_top_;
};

// Native unwinder implementation for Android, using libunwindstack.
class BASE_EXPORT NativeUnwinderAndroid
    : public Unwinder,
      public ModuleCache::AuxiliaryModuleProvider {
 public:
  // Creates maps object from /proc/self/maps for use by NativeUnwinderAndroid.
  // Since this is an expensive call, the maps object should be re-used across
  // all profiles in a process.
  // Set |use_updatable_maps| to true to use unwindstack::LocalUpdatableMaps,
  // instead of unwindstack::LocalMaps. LocalUpdatableMaps might be preferable
  // when the frames come from dynamically added ELFs like JITed ELFs, or
  // dynamically loaded libraries. With LocalMaps the frames corresponding to
  // newly loaded ELFs don't get unwound since the existing maps structure
  // fails to find a map for the given pc while LocalUpdatableMaps reparses
  // /proc/self/maps when it fails to find a map for the given pc and then can
  // successfully unwind through newly loaded ELFs as well.
  static std::unique_ptr<NativeUnwinderAndroidMemoryRegionsMap>
  CreateMemoryRegionsMap(bool use_updatable_maps = false);

  // |exclude_module_with_base_address| is used to exclude a specific module and
  // let another unwinder take control. TryUnwind() will exit with
  // UNRECOGNIZED_FRAME and CanUnwindFrom() will return false when a frame is
  // encountered in that module.
  // |map_delegate| is used to manage memory used by libunwindstack. It must
  // outlives this object.
  NativeUnwinderAndroid(uintptr_t exclude_module_with_base_address,
                        NativeUnwinderAndroidMapDelegate* map_delegate);
  ~NativeUnwinderAndroid() override;

  NativeUnwinderAndroid(const NativeUnwinderAndroid&) = delete;
  NativeUnwinderAndroid& operator=(const NativeUnwinderAndroid&) = delete;

  // Unwinder
  void InitializeModules() override;
  bool CanUnwindFrom(const Frame& current_frame) const override;
  UnwindResult TryUnwind(UnwinderStateCapture* capture_state,
                         RegisterContext* thread_context,
                         uintptr_t stack_top,
                         std::vector<Frame>* stack) override;

  // ModuleCache::AuxiliaryModuleProvider
  std::unique_ptr<const ModuleCache::Module> TryCreateModuleForAddress(
      uintptr_t address) override;

 private:
  unwindstack::DexFiles* GetOrCreateDexFiles(unwindstack::ArchEnum arch);

  void EmitDexFrame(uintptr_t dex_pc,
                    unwindstack::ArchEnum,
                    std::vector<Frame>* stack);

  std::unique_ptr<unwindstack::DexFiles> dex_files_;

  const uintptr_t exclude_module_with_base_address_;
  raw_ptr<NativeUnwinderAndroidMapDelegate> map_delegate_;
  const raw_ptr<NativeUnwinderAndroidMemoryRegionsMapImpl> memory_regions_map_;
  // This is a vector (rather than an array) because it gets used in functions
  // from libunwindstack.
  const std::vector<std::string> search_libs_ = {"libart.so", "libartd.so"};
};

}  // namespace base

#endif  // BASE_PROFILER_NATIVE_UNWINDER_ANDROID_H_
