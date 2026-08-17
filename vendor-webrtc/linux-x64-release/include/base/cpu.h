// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CPU_H_
#define BASE_CPU_H_

#include <cstdint>
#include <string>

#include "base/base_export.h"
#include "build/build_config.h"

namespace base {

#if defined(ARCH_CPU_X86_FAMILY)
namespace internal {

struct X86ModelInfo {
  int family;
  int model;
  int ext_family;
  int ext_model;
};

// Compute the CPU family and model based on the vendor and CPUID signature.
BASE_EXPORT X86ModelInfo ComputeX86FamilyAndModel(const std::string& vendor,
                                                  int signature);

}  // namespace internal
#endif  // defined(ARCH_CPU_X86_FAMILY)

// Query information about the processor.
class BASE_EXPORT CPU final {
 public:
  CPU();
  CPU(CPU&&);
  CPU(const CPU&) = delete;

  // Get a preallocated instance of CPU.
  // It can be used in very early application startup or in memory allocation
  // handlers. The instance of CPU is created without branding, see `CPU(bool
  // requires_branding)` for details and implications.
  //
  // Support for various security features such as Arm's BTI and MTE uses this
  // instance to detect CPU support. To prevent any attempt
  // to disable a feature by attacking this data, base::ProtectedMemory is used
  // to protected it from write accesses.
  // Note that base::ProtectedMemory falls back to unprotected memory if the
  // target OS is not supported.
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
    AVX_VNNI = 10,
    AVX512F = 11,
    AVX512BW = 12,
    AVX512_VNNI = 13,
    MAX_INTEL_MICRO_ARCHITECTURE = 14
  };

  // Accessors for CPU information.
  // TODO(crbug.com/335001230): Most if not all of these should be x86-only.
  std::string vendor_name() const { return cpu_vendor_; }
  int signature() const { return signature_; }
  int stepping() const { return stepping_; }
  int model() const { return model_; }
  int family() const { return family_; }
  int type() const { return type_; }
  int extended_model() const { return ext_model_; }
  int extended_family() const { return ext_family_; }
#if defined(ARCH_CPU_X86_FAMILY)
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
  bool has_avx_vnni() const { return has_avx_vnni_; }
  bool has_avx512_f() const { return has_avx512_f_; }
  bool has_avx512_bw() const { return has_avx512_bw_; }
  bool has_avx512_vnni() const { return has_avx512_vnni_; }
#endif
  bool has_aesni() const { return has_aesni_; }
  bool has_non_stop_time_stamp_counter() const {
    return has_non_stop_time_stamp_counter_;
  }
  bool is_running_in_vm() const { return is_running_in_vm_; }

  // Armv8.5-A extensions for control flow and memory safety.
#if defined(ARCH_CPU_ARM_FAMILY)
  bool has_mte() const { return has_mte_; }
  bool has_bti() const { return has_bti_; }
#else
  constexpr bool has_mte() const { return false; }
  constexpr bool has_bti() const { return false; }
#endif

#if defined(ARCH_CPU_X86_FAMILY)
  // Memory protection key support for user-mode pages
  bool has_pku() const { return has_pku_; }
#else
  constexpr bool has_pku() const { return false; }
#endif

#if defined(ARCH_CPU_X86_FAMILY)
  IntelMicroArchitecture GetIntelMicroArchitecture() const;
#endif
  std::string cpu_brand() const { return cpu_brand_; }

 private:
  // Query the processor for CPUID information.
  void Initialize();

  int signature_ = 0;  // raw form of type, family, model, and stepping
  int type_ = 0;       // process type
  int family_ = 0;     // family of the processor
  int model_ = 0;      // model of processor
  int stepping_ = 0;   // processor revision number
  int ext_model_ = 0;
  int ext_family_ = 0;
#if defined(ARCH_CPU_X86_FAMILY)
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
  bool has_avx_vnni_ = false;
  bool has_avx512_f_ = false;
  bool has_avx512_bw_ = false;
  bool has_avx512_vnni_ = false;
#endif
  bool has_aesni_ = false;
#if defined(ARCH_CPU_ARM_FAMILY)
  bool has_mte_ = false;  // Armv8.5-A MTE (Memory Taggging Extension)
  bool has_bti_ = false;  // Armv8.5-A BTI (Branch Target Identification)
#endif
#if defined(ARCH_CPU_X86_FAMILY)
  bool has_pku_ = false;
#endif
  bool has_non_stop_time_stamp_counter_ = false;
  bool is_running_in_vm_ = false;

  // The CPUID instruction of the X86 instruction set returns the vendor name in
  // 3 32bit registers, which make 12 characters. See "Intel® 64 and IA-32
  // Architectures Software Developer’s Manual - Volume 2".
  static constexpr size_t kVendorNameSize = 12;
  char cpu_vendor_[kVendorNameSize + 1] = "unknown";
  // The CPUID instruction of the X86 instruction set returns the brand name in
  // 3*4 32bit registers, which make 48 characters. See "Intel® 64 and IA-32
  // Architectures Software Developer’s Manual - Volume 2".
  static constexpr size_t kBrandNameSize = 48;
  char cpu_brand_[kBrandNameSize + 1] = "\0";
};

}  // namespace base

#endif  // BASE_CPU_H_
