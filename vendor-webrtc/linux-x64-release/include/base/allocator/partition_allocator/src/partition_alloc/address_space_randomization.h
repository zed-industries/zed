// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_ADDRESS_SPACE_RANDOMIZATION_H_
#define PARTITION_ALLOC_ADDRESS_SPACE_RANDOMIZATION_H_

#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/page_allocator_constants.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc {

// Calculates a random preferred mapping address. In calculating an address, we
// balance good ASLR against not fragmenting the address space too badly.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) uintptr_t GetRandomPageBase();

namespace internal {

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
AslrAddress(uintptr_t mask) {
  return mask & PageAllocationGranularityBaseMask();
}
PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
AslrMask(uintptr_t bits) {
  return AslrAddress((1ULL << bits) - 1ULL);
}

// Turn off formatting, because the thicket of nested ifdefs below is
// incomprehensible without indentation. It is also incomprehensible with
// indentation, but the only other option is a combinatorial explosion of
// *_{win,linux,mac,foo}_{32,64}.h files.
//
// clang-format off

#if PA_BUILDFLAG(PA_ARCH_CPU_64_BITS)

  #if defined(MEMORY_TOOL_REPLACES_ALLOCATOR)

    // We shouldn't allocate system pages at all for sanitizer builds. However,
    // we do, and if random hint addresses interfere with address ranges
    // hard-coded in those tools, bad things happen. This address range is
    // copied from TSAN source but works with all tools. See
    // https://crbug.com/539863.
    PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
    ASLRMask() {
      return AslrAddress(0x007fffffffffULL);
    }
    PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
    ASLROffset() {
      return AslrAddress(0x7e8000000000ULL);
    }

  #elif PA_BUILDFLAG(IS_WIN)

    // Windows 8.10 and newer support the full 48 bit address range. Since
    // ASLROffset() is non-zero and may cause a carry, use 47 bit masks. See
    // http://www.alex-ionescu.com/?p=246
    PA_ALWAYS_INLINE constexpr uintptr_t ASLRMask() {
      return AslrMask(47);
    }
    // Try not to map pages into the range where Windows loads DLLs by default.
    PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
      return 0x80000000ULL;
    }

  #elif PA_BUILDFLAG(IS_APPLE)

    // macOS as of 10.12.5 does not clean up entries in page map levels 3/4
    // [PDP/PML4] created from mmap or mach_vm_allocate, even after the region
    // is destroyed. Using a virtual address space that is too large causes a
    // leak of about 1 wired [can never be paged out] page per call to mmap. The
    // page is only reclaimed when the process is killed. Confine the hint to a
    // 39-bit section of the virtual address space.
    //
    // This implementation adapted from
    // https://chromium-review.googlesource.com/c/v8/v8/+/557958. The difference
    // is that here we clamp to 39 bits, not 32.
    //
    // TODO(crbug.com/40528509): Remove this limitation if/when the macOS
    // behavior changes.
    PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
    ASLRMask() {
      return AslrMask(38);
    }
    PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
    ASLROffset() {
      // Be careful, there is a zone where macOS will not map memory, at least
      // on ARM64. From an ARM64 machine running 12.3, the range seems to be
      // [0x1000000000, 0x7000000000). Make sure that the range we use is
      // outside these bounds. In 12.3, there is a reserved area between
      // MACH_VM_MIN_GPU_CARVEOUT_ADDRESS and MACH_VM_MAX_GPU_CARVEOUT_ADDRESS,
      // which is reserved on ARM64. See these constants in XNU's source code
      // for details (xnu-8019.80.24/osfmk/mach/arm/vm_param.h).
      return AslrAddress(0x10000000000ULL);
    }

  #elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)

    #if PA_BUILDFLAG(PA_ARCH_CPU_X86_64)

      // Linux (and macOS) support the full 47-bit user space of x64 processors.
      // Use only 46 to allow the kernel a chance to fulfill the request.
      PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
      ASLRMask() {
        return AslrMask(46);
      }
      PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
      ASLROffset() {
        return AslrAddress(0);
      }

    #elif PA_BUILDFLAG(IS_ANDROID) && (PA_BUILDFLAG(PA_ARCH_CPU_ARM64) || PA_BUILDFLAG(PA_ARCH_CPU_RISCV64))
      // Restrict the address range on Android to avoid a large performance
      // regression in single-process WebViews. See https://crbug.com/837640.
      PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
      ASLRMask() {
        return AslrMask(30);
      }
      PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
      ASLROffset() {
        return AslrAddress(0x20000000ULL);
      }
    #elif PA_BUILDFLAG(PA_ARCH_CPU_ARM64)
      #if PA_BUILDFLAG(IS_LINUX)

      // Linux on arm64 can use 39, 42, 48, or 52-bit user space, depending on
      // page size and number of levels of translation pages used. We use
      // 39-bit as base as all setups should support this, lowered to 38-bit
      // as ASLROffset() could cause a carry.
      PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
      ASLRMask() {
        return AslrMask(38);
      }
      PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
      ASLROffset() {
        return AslrAddress(0x1000000000ULL);
      }

      #else

      // ARM64 on Linux has 39-bit user space. Use 38 bits since ASLROffset()
      // could cause a carry.
      PA_ALWAYS_INLINE constexpr uintptr_t ASLRMask() {
        return AslrMask(38);
      }
      PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
        return AslrAddress(0x1000000000ULL);
      }

      #endif

    #elif PA_BUILDFLAG(PA_ARCH_CPU_PPC64)

      #if PA_BUILDFLAG(IS_AIX)

        // AIX has 64 bits of virtual addressing, but we limit the address range
        // to (a) minimize segment lookaside buffer (SLB) misses; and (b) use
        // extra address space to isolate the mmap regions.
        PA_ALWAYS_INLINE constexpr uintptr_t ASLRMask() {
          return AslrMask(30);
        }
        PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
          return AslrAddress(0x400000000000ULL);
        }

      #elif PA_BUILDFLAG(PA_ARCH_CPU_BIG_ENDIAN)

        // Big-endian Linux PPC has 44 bits of virtual addressing. Use 42.
        PA_ALWAYS_INLINE constexpr uintptr_t ASLRMask() {
          return AslrMask(42);
        }
        PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
          return AslrAddress(0);
        }

      #else  // !PA_BUILDFLAG(IS_AIX) && !PA_BUILDFLAG(PA_ARCH_CPU_BIG_ENDIAN)
        #if PA_BUILDFLAG(IS_LINUX)

        // Little-endian Linux PPC has 48 bits of virtual addressing. Use 46.
        PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t ASLRMask() {
          return AslrMask(46);
        }
        PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t ASLROffset() {
          return AslrAddress(0);
        }

        #else

        PA_ALWAYS_INLINE constexpr uintptr_t ASLRMask() {
          return AslrMask(46);
        }
        PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
          return AslrAddress(0);
        }

        #endif

      #endif  // !PA_BUILDFLAG(IS_AIX) && !PA_BUILDFLAG(PA_ARCH_CPU_BIG_ENDIAN)

    #elif PA_BUILDFLAG(PA_ARCH_CPU_S390X)

      // Linux on Z uses bits 22 - 32 for Region Indexing, which translates to
      // 42 bits of virtual addressing. Truncate to 40 bits to allow kernel a
      // chance to fulfill the request.
      PA_ALWAYS_INLINE constexpr uintptr_t ASLRMask() {
        return AslrMask(40);
      }
      PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
        return AslrAddress(0);
      }

    #elif PA_BUILDFLAG(PA_ARCH_CPU_S390)

      // 31 bits of virtual addressing. Truncate to 29 bits to allow the kernel
      // a chance to fulfill the request.
      PA_ALWAYS_INLINE constexpr uintptr_t ASLRMask() {
        return AslrMask(29);
      }
      PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
        return AslrAddress(0);
      }

    #else  // !PA_BUILDFLAG(PA_ARCH_CPU_X86_64) && !PA_BUILDFLAG(PA_ARCH_CPU_PPC64) &&
           // !PA_BUILDFLAG(PA_ARCH_CPU_S390X) && !PA_BUILDFLAG(PA_ARCH_CPU_S390)

      // For all other POSIX variants, use 30 bits.
      PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
      ASLRMask() {
        return AslrMask(30);
      }

      #if PA_BUILDFLAG(IS_SOLARIS)

        // For our Solaris/illumos mmap hint, we pick a random address in the
        // bottom half of the top half of the address space (that is, the third
        // quarter). Because we do not MAP_FIXED, this will be treated only as a
        // hint -- the system will not fail to mmap because something else
        // happens to already be mapped at our random address. We deliberately
        // set the hint high enough to get well above the system's break (that
        // is, the heap); Solaris and illumos will try the hint and if that
        // fails allocate as if there were no hint at all. The high hint
        // prevents the break from getting hemmed in at low values, ceding half
        // of the address space to the system heap.
        PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
          return AslrAddress(0x80000000ULL);
        }

      #elif PA_BUILDFLAG(IS_AIX)

        // The range 0x30000000 - 0xD0000000 is available on AIX; choose the
        // upper range.
        PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
          return AslrAddress(0x90000000ULL);
        }

      #else  // !PA_BUILDFLAG(IS_SOLARIS) && !PA_BUILDFLAG(IS_AIX)

        // The range 0x20000000 - 0x60000000 is relatively unpopulated across a
        // variety of ASLR modes (PAE kernel, NX compat mode, etc) and on macOS
        // 10.6 and 10.7.
        PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
        ASLROffset() {
          return AslrAddress(0x20000000ULL);
        }

      #endif  // !PA_BUILDFLAG(IS_SOLARIS) && !PA_BUILDFLAG(IS_AIX)

    #endif  // !PA_BUILDFLAG(PA_ARCH_CPU_X86_64) && !PA_BUILDFLAG(PA_ARCH_CPU_PPC64) &&
            // !PA_BUILDFLAG(PA_ARCH_CPU_S390X) && !PA_BUILDFLAG(PA_ARCH_CPU_S390)

  #endif  // PA_BUILDFLAG(IS_POSIX)

#elif PA_BUILDFLAG(PA_ARCH_CPU_32_BITS)

  // This is a good range on 32-bit Windows and Android (the only platforms on
  // which we support 32-bitness). Allocates in the 0.5 - 1.5 GiB region. There
  // is no issue with carries here.
  PA_ALWAYS_INLINE constexpr uintptr_t ASLRMask() {
    return AslrMask(30);
  }
  PA_ALWAYS_INLINE constexpr uintptr_t ASLROffset() {
    return AslrAddress(0x20000000ULL);
  }

#else

  #error Please tell us about your exotic hardware! Sounds interesting.

#endif  // PA_BUILDFLAG(PA_ARCH_CPU_32_BITS)

// clang-format on

}  // namespace internal

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_ADDRESS_SPACE_RANDOMIZATION_H_
