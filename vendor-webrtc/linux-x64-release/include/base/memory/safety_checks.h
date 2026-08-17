// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SAFETY_CHECKS_H_
#define BASE_MEMORY_SAFETY_CHECKS_H_

#include <new>
#include <type_traits>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/memory/stack_allocated.h"
#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
#include "base/allocator/partition_alloc_support.h"
#include "partition_alloc/partition_alloc_constants.h"  // nogncheck
#endif

#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
#include "partition_alloc/shim/allocator_shim_default_dispatch_to_partition_alloc.h"
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)

// This header defines `ADVANCED_MEMORY_SAFETY_CHECKS()` macro.
// They can be used to specify a class/struct that is targeted to perform
// additional CHECKS across variety of memory safety mechanisms such as
// PartitionAllocator.
//   ```
//   class Foo {
//     ADVANCED_MEMORY_SAFETY_CHECKS();
//   }
//   ```
// Checks here are disabled by default because of their performance cost.
// Currently, the macro is managed by the memory safety team internally and
// you should not add / remove it manually.
//
// Additional checks here are categorized into either one of enum
// `MemorySafetyCheck`. Some of them are too costly and disabled even for
// `ADVANCED_MEMORY_SAFETY_CHECKS()` annotated types. These checks can be
// enabled by passing optional arguments to the macro.
//   ```
//   class Foo {
//     ADVANCED_MEMORY_SAFETY_CHECKS(
//       /*enable=*/ kFoo | kBar);
//   }
//   ```
// It is also possible to disable default checks for annotated types.
//   ```
//   class Foo {
//     ADVANCED_MEMORY_SAFETY_CHECKS(
//       /*enable=*/  kFoo,
//       /*disable=*/ kBaz);
//   }
//   ```

namespace base {

// We cannot hide things behind anonymous namespace because they are referenced
// via macro, which can be defined anywhere.
// To avoid tainting ::base namespace, define things inside this namespace.
namespace internal {

enum class MemorySafetyCheck : uint32_t {
  kNone = 0,
  kForcePartitionAlloc = (1u << 0),
  // Enables |FreeFlags::kSchedulerLoopQuarantine|.
  // Requires PA-E.
  kSchedulerLoopQuarantine = (1u << 1),
};

constexpr MemorySafetyCheck operator|(MemorySafetyCheck a,
                                      MemorySafetyCheck b) {
  return static_cast<MemorySafetyCheck>(static_cast<uint32_t>(a) |
                                        static_cast<uint32_t>(b));
}

constexpr MemorySafetyCheck operator&(MemorySafetyCheck a,
                                      MemorySafetyCheck b) {
  return static_cast<MemorySafetyCheck>(static_cast<uint32_t>(a) &
                                        static_cast<uint32_t>(b));
}

constexpr MemorySafetyCheck operator~(MemorySafetyCheck a) {
  return static_cast<MemorySafetyCheck>(~static_cast<uint32_t>(a));
}

// Set of checks for ADVANCED_MEMORY_SAFETY_CHECKS() annotated objects.
constexpr auto kAdvancedMemorySafetyChecks =
    MemorySafetyCheck::kForcePartitionAlloc |
    MemorySafetyCheck::kSchedulerLoopQuarantine;

// Define type traits to determine type |T|'s memory safety check status.
namespace {

// Allocator type traits.
constexpr bool ShouldUsePartitionAlloc(MemorySafetyCheck checks) {
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  return static_cast<bool>(checks &
                           (MemorySafetyCheck::kForcePartitionAlloc |
                            MemorySafetyCheck::kSchedulerLoopQuarantine));
#else
  return false;
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
}

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)

// Returns |partition_alloc::AllocFlags| corresponding to |checks|.
constexpr partition_alloc::AllocFlags GetAllocFlags(MemorySafetyCheck checks) {
  return partition_alloc::AllocFlags::kReturnNull |
         partition_alloc::AllocFlags::kNoHooks;
}

// Returns |partition_alloc::FreeFlags| corresponding to |checks|.
constexpr partition_alloc::FreeFlags GetFreeFlags(MemorySafetyCheck checks) {
  auto flags = partition_alloc::FreeFlags::kNone;
  if (static_cast<bool>(checks & MemorySafetyCheck::kSchedulerLoopQuarantine)) {
    flags |= partition_alloc::FreeFlags::kSchedulerLoopQuarantine;
  }
  return flags;
}

#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC)

}  // namespace

// Public utility type traits.
template <typename T>
inline constexpr MemorySafetyCheck get_memory_safety_checks = [] {
  if constexpr (requires { T::kMemorySafetyChecks; }) {
    return T::kMemorySafetyChecks;
  } else {
    return static_cast<MemorySafetyCheck>(0);
  }
}();

template <typename T, MemorySafetyCheck c>
inline constexpr bool is_memory_safety_checked =
    (get_memory_safety_checks<T> & c) == c;

// Allocator functions.
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
ALWAYS_INLINE partition_alloc::PartitionRoot*
GetPartitionRootForMemorySafetyCheckedAllocation() {
  return allocator_shim::internal::PartitionAllocMalloc::Allocator();
}
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)

template <MemorySafetyCheck checks>
NOINLINE void* HandleMemorySafetyCheckedOperatorNew(std::size_t count) {
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  if constexpr (ShouldUsePartitionAlloc(checks)) {
    return GetPartitionRootForMemorySafetyCheckedAllocation()
        ->AllocInline<GetAllocFlags(checks)>(count);
  }
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  return ::operator new(count);
}

template <MemorySafetyCheck checks>
NOINLINE void* HandleMemorySafetyCheckedOperatorNew(
    std::size_t count,
    std::align_val_t alignment) {
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  if constexpr (ShouldUsePartitionAlloc(checks)) {
    return GetPartitionRootForMemorySafetyCheckedAllocation()
        ->AlignedAlloc<GetAllocFlags(checks)>(static_cast<size_t>(alignment),
                                              count);
  }
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  return ::operator new(count, alignment);
}

template <MemorySafetyCheck checks>
NOINLINE void HandleMemorySafetyCheckedOperatorDelete(void* ptr) {
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  if constexpr (ShouldUsePartitionAlloc(checks)) {
    GetPartitionRootForMemorySafetyCheckedAllocation()
        ->Free<GetFreeFlags(checks)>(ptr);
    return;
  }
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  ::operator delete(ptr);
}

template <MemorySafetyCheck checks>
NOINLINE void HandleMemorySafetyCheckedOperatorDelete(
    void* ptr,
    std::align_val_t alignment) {
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  if constexpr (ShouldUsePartitionAlloc(checks)) {
    GetPartitionRootForMemorySafetyCheckedAllocation()
        ->Free<GetFreeFlags(checks)>(ptr);
    return;
  }
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  ::operator delete(ptr, alignment);
}

}  // namespace internal

// Macros to annotate class/struct's default memory safety check.
// ADVANCED_MEMORY_SAFETY_CHECKS(): Enable Check |kAdvancedChecks| for this
// object.
//
// Note that if you use this macro at the top of struct declaration, the
// declaration context would be left as |private|. Please switch it back to
// |public| manually if needed.
//
//   struct ObjectWithAdvancedChecks {
//     ADVANCED_MEMORY_SAFETY_CHECKS();
//   public:
//     int public_field;
//   };
#define MEMORY_SAFETY_CHECKS_INTERNAL(SPECIFIER, DEFAULT_CHECKS,               \
                                      ENABLED_CHECKS, DISABLED_CHECKS, ...)    \
 public:                                                                       \
  static constexpr auto kMemorySafetyChecks = [] {                             \
    using enum base::internal::MemorySafetyCheck;                              \
    return (DEFAULT_CHECKS | ENABLED_CHECKS) & ~(DISABLED_CHECKS);             \
  }();                                                                         \
  SPECIFIER static void* operator new(std::size_t count) {                     \
    return base::internal::HandleMemorySafetyCheckedOperatorNew<               \
        kMemorySafetyChecks>(count);                                           \
  }                                                                            \
  SPECIFIER static void* operator new(std::size_t count,                       \
                                      std::align_val_t alignment) {            \
    return base::internal::HandleMemorySafetyCheckedOperatorNew<               \
        kMemorySafetyChecks>(count, alignment);                                \
  }                                                                            \
  /* Though we do not hook placement new, we need to define this */            \
  /* explicitly to allow it. */                                                \
  ALWAYS_INLINE static void* operator new(std::size_t, void* ptr) {            \
    return ptr;                                                                \
  }                                                                            \
  SPECIFIER static void operator delete(void* ptr) noexcept {                  \
    base::internal::HandleMemorySafetyCheckedOperatorDelete<                   \
        kMemorySafetyChecks>(ptr);                                             \
  }                                                                            \
  SPECIFIER static void operator delete(void* ptr,                             \
                                        std::align_val_t alignment) noexcept { \
    base::internal::HandleMemorySafetyCheckedOperatorDelete<                   \
        kMemorySafetyChecks>(ptr, alignment);                                  \
  }                                                                            \
                                                                               \
 private:                                                                      \
  static_assert(true) /* semicolon here */

#if DCHECK_IS_ON()
// Specify NOINLINE to display the operator on a stack trace.
// When 2 args provided, these two are passed to `ENABLED_CHECKS` and
// `DISABLED_CHECKS`. A couple of `MemorySafetyCheck::kNone` is ignored.
// When 1 arg provided, the one is passed to `ENABLED_CHECKS` and the first
// `MemorySafetyCheck::kNone` serves a default value for `DISABLED_CHECKS`.
// When 0 arg provided, both of `MemorySafetyCheck::kNone`s serve as default
// values for `ENABLED_CHECKS` and `DISABLED_CHECKS` accordingly.
#define ADVANCED_MEMORY_SAFETY_CHECKS(...)                                    \
  MEMORY_SAFETY_CHECKS_INTERNAL(                                              \
      NOINLINE NOT_TAIL_CALLED,                                               \
      base::internal::kAdvancedMemorySafetyChecks __VA_OPT__(, ) __VA_ARGS__, \
      kNone, kNone)
#else
#define ADVANCED_MEMORY_SAFETY_CHECKS(...)                                    \
  MEMORY_SAFETY_CHECKS_INTERNAL(                                              \
      ALWAYS_INLINE,                                                          \
      base::internal::kAdvancedMemorySafetyChecks __VA_OPT__(, ) __VA_ARGS__, \
      kNone, kNone)
#endif  // DCHECK_IS_ON()

// When a struct/class with `ADVANCED_MEMORY_SAFETY_CHECKS()` is inherited, a
// derived struct/class operator will use customized `operator new()` and
// `operator delete()` too. If a class has multiple base classes with the macro,
// a compiler may complain ambiguity between multiple `operator new()`s. On the
// other hand, if a class uses private inheritance, a compiler may report
// private `operator new()` that is making impossible to `new` that class. We
// have two utility macros to resolve these issues:
// - `INHERIT_MEMORY_SAFETY_CHECKS(BaseClass)`
//       Explicitly exports operators from given `BaseClass` to re-apply
//       checks specified in the parent class. This is the recommended option as
//       a derived class is likely to have the same characteristics to its baes
//       class. This macro accepts additional arguments to overwrite
//       `BaseClass`'s opted-in checks.
//         ```
//         INHERIT_MEMORY_SAFETY_CHECKS(BaseClass,
//           /*enable=*/  kFoo | kBar,
//           /*disable=*/ kBaz);
//         ```
// - `DEFAULT_MEMORY_SAFETY_CHECKS()`
//       Re-define default `operator new()` and `operator delete()` using
//       global operators that comes with default checks. This macro accepts
//       additional arguments to enable some checks manually.
//         ```
//         DEFAULT_MEMORY_SAFETY_CHECKS(BaseClass,
//           /*enable=*/ kFoo | kBar);
//         ```
//
// Note that if you use these macros at the top of struct declaration, the
// declaration context would be left as |private|. Please switch it back to
// |public| manually if needed.
#define INHERIT_MEMORY_SAFETY_CHECKS(BASE_CLASS, ...)                          \
  MEMORY_SAFETY_CHECKS_INTERNAL(ALWAYS_INLINE,                                 \
                                BASE_CLASS::kMemorySafetyChecks __VA_OPT__(, ) \
                                    __VA_ARGS__,                               \
                                kNone, kNone)

#define DEFAULT_MEMORY_SAFETY_CHECKS(...) \
  MEMORY_SAFETY_CHECKS_INTERNAL(          \
      ALWAYS_INLINE, kNone __VA_OPT__(, ) __VA_ARGS__, kNone, kNone)

// Utility function to detect Double-Free or Out-of-Bounds writes.
// This function can be called to memory assumed to be valid.
// If not, this may crash (not guaranteed).
// This is useful if you want to investigate crashes at `free()`,
// to know which point at execution it goes wrong.
BASE_EXPORT void CheckHeapIntegrity(const void* ptr);

// Utility class to exclude deallocation from optional safety checks when an
// instance is on the stack. Can be applied to performance critical functions.
class BASE_EXPORT ScopedSafetyChecksExclusion {
  STACK_ALLOCATED();

 public:
  // Make this non-trivially-destructible to suppress unused variable warning.
  ~ScopedSafetyChecksExclusion() {}  // NOLINT(modernize-use-equals-default)

 private:
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  base::allocator::ScopedSchedulerLoopQuarantineExclusion
      opt_out_scheduler_loop_quarantine_;
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
};

}  // namespace base

#endif  // BASE_MEMORY_SAFETY_CHECKS_H_
