// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_CPU_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_CPU_H_

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base {

// Query information about the processor.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) CPU final {
 public:
  CPU();
  CPU(CPU&&);
  CPU(const CPU&) = delete;

  // Get a preallocated instance of CPU.
  // This can be used in very early application startup. The instance of CPU is
  // created without branding, see CPU(bool requires_branding) for details and
  // implications.
  static const CPU& GetInstanceNoAllocation();

  enum IntelMicroArchitecture {
    PENTIUM = 0,
    SSE = 1,
    SSE2 = 2,
    SSE3 = 3,
    SSSE3 = 4,
    SSE41 = 5,
    SSE42 = 6,
    AVX = 7,
    AVX2 = 8,
    FMA3 = 9,
    MAX_INTEL_MICRO_ARCHITECTURE = 10
  };

  // Accessors for CPU information.
  int signature() const { return signature_; }
  int stepping() const { return stepping_; }
  int type() const { return type_; }
  bool has_mmx() const { return has_mmx_; }
  bool has_sse() const { return has_sse_; }
  bool has_sse2() const { return has_sse2_; }
  bool has_sse3() const { return has_sse3_; }
  bool has_ssse3() const { return has_ssse3_; }
  bool has_sse41() const { return has_sse41_; }
  bool has_sse42() const { return has_sse42_; }
  bool has_popcnt() const { return has_popcnt_; }
  bool has_avx() const { return has_avx_; }
  bool has_fma3() const { return has_fma3_; }
  bool has_avx2() const { return has_avx2_; }
  bool has_aesni() const { return has_aesni_; }
  bool has_non_stop_time_stamp_counter() const {
    return has_non_stop_time_stamp_counter_;
  }
  bool is_running_in_vm() const { return is_running_in_vm_; }

  // Armv8.5-A extensions for control flow and memory safety.
#if PA_BUILDFLAG(PA_ARCH_CPU_ARM_FAMILY)
  bool has_mte() const { return has_mte_; }
  bool has_bti() const { return has_bti_; }
#else
  constexpr bool has_mte() const { return false; }
  constexpr bool has_bti() const { return false; }
#endif

#if PA_BUILDFLAG(PA_ARCH_CPU_X86_FAMILY)
  // Memory protection key support for user-mode pages
  bool has_pku() const { return has_pku_; }
#else
  constexpr bool has_pku() const { return false; }
#endif

 private:
  // Query the processor for CPUID information.
  void Initialize();

  int signature_ = 0;  // raw form of type, family, model, and stepping
  int type_ = 0;       // process type
  int stepping_ = 0;   // processor revision number
  bool has_mmx_ = false;
  bool has_sse_ = false;
  bool has_sse2_ = false;
  bool has_sse3_ = false;
  bool has_ssse3_ = false;
  bool has_sse41_ = false;
  bool has_sse42_ = false;
  bool has_popcnt_ = false;
  bool has_avx_ = false;
  bool has_fma3_ = false;
  bool has_avx2_ = false;
  bool has_aesni_ = false;
#if PA_BUILDFLAG(PA_ARCH_CPU_ARM_FAMILY)
  bool has_mte_ = false;  // Armv8.5-A MTE (Memory Taggging Extension)
  bool has_bti_ = false;  // Armv8.5-A BTI (Branch Target Identification)
#endif
#if PA_BUILDFLAG(PA_ARCH_CPU_X86_FAMILY)
  bool has_pku_ = false;
#endif
  bool has_non_stop_time_stamp_counter_ = false;
  bool is_running_in_vm_ = false;
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_CPU_H_
