// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_RAW_PTR_ASAN_SERVICE_H_
#define BASE_MEMORY_RAW_PTR_ASAN_SERVICE_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)
#include <cstddef>
#include <cstdint>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/types/strong_alias.h"

namespace base {

using EnableDereferenceCheck =
    base::StrongAlias<class EnableDereferenceCheckTag, bool>;
using EnableExtractionCheck =
    base::StrongAlias<class EnableExtractionCheckTag, bool>;
using EnableInstantiationCheck =
    base::StrongAlias<class EnableInstantiationCheckTag, bool>;

class BASE_EXPORT RawPtrAsanService {
 public:
  enum class ReportType {
    kDereference,
    kExtraction,
    kInstantiation,
  };

  struct PendingReport {
    ReportType type = ReportType::kDereference;
    uintptr_t allocation_base = 0;
    size_t allocation_size = 0;
  };

  void Configure(EnableDereferenceCheck,
                 EnableExtractionCheck,
                 EnableInstantiationCheck);

  bool IsSupportedAllocation(void*) const;

  bool IsEnabled() const { return mode_ == Mode::kEnabled; }

  NO_SANITIZE("address")
  ALWAYS_INLINE bool is_dereference_check_enabled() const {
    return is_dereference_check_enabled_;
  }

  NO_SANITIZE("address")
  ALWAYS_INLINE bool is_extraction_check_enabled() const {
    return is_extraction_check_enabled_;
  }

  NO_SANITIZE("address")
  ALWAYS_INLINE bool is_instantiation_check_enabled() const {
    return is_instantiation_check_enabled_;
  }

  NO_SANITIZE("address") ALWAYS_INLINE static RawPtrAsanService& GetInstance() {
    return instance_;
  }

  void WarnOnDanglingExtraction(const volatile void* ptr) const;
  void CrashOnDanglingInstantiation(const volatile void* ptr) const;

  static void SetPendingReport(ReportType type, const volatile void* ptr);

 private:
  enum class Mode {
    kUninitialized,
    kDisabled,
    kEnabled,
  };

  uint8_t* GetShadow(void* ptr) const;

  static void MallocHook(const volatile void*, size_t);
  static void FreeHook(const volatile void*) {}
  static void ErrorReportCallback(const char* report,
                                  bool* should_exit_cleanly);

  Mode mode_ = Mode::kUninitialized;
  bool is_dereference_check_enabled_ = false;
  bool is_extraction_check_enabled_ = false;
  bool is_instantiation_check_enabled_ = false;

  size_t shadow_offset_ = 0;

  static RawPtrAsanService instance_;  // Not a static local variable because
                                       // `GetInstance()` is used in hot paths.
};

}  // namespace base

#endif  // PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)
#endif  // BASE_MEMORY_RAW_PTR_ASAN_SERVICE_H_
