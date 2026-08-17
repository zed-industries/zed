// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// IWYU pragma: private, include "base/memory/raw_ptr.h"

#ifndef PARTITION_ALLOC_POINTERS_RAW_PTR_H_
#define PARTITION_ALLOC_POINTERS_RAW_PTR_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <iterator>
#include <memory>
#include <type_traits>
#include <utility>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/flags.h"
#include "partition_alloc/partition_alloc_base/augmentations/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/cxx20_is_constant_evaluated.h"
#include "partition_alloc/partition_alloc_base/types/same_as_any.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/pointers/instance_tracer.h"

#if PA_HAVE_SPACESHIP_OPERATOR
#include <compare>
#endif

#if PA_BUILDFLAG(IS_WIN)
#include "partition_alloc/partition_alloc_base/win/win_handle_types.h"
#endif

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
#include "partition_alloc/partition_alloc_base/check.h"
// Live implementation of MiraclePtr being built.
#if PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) || \
    PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)
#define PA_RAW_PTR_CHECK(condition) PA_BASE_CHECK(condition)
#else
// No-op implementation of MiraclePtr being built.
// Note that `PA_BASE_DCHECK()` evaporates from non-DCHECK builds,
// minimizing impact of generated code.
#define PA_RAW_PTR_CHECK(condition) PA_BASE_DCHECK(condition)
#endif  // PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||
        // PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)
#else   // PA_BUILDFLAG(USE_PARTITION_ALLOC)
// Without PartitionAlloc, there's no `PA_BASE_D?CHECK()` implementation
// available.
#define PA_RAW_PTR_CHECK(condition)
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC)

#if PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL)
#include "partition_alloc/pointers/raw_ptr_backup_ref_impl.h"
#elif PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL)
#include "partition_alloc/pointers/raw_ptr_asan_unowned_impl.h"
#elif PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL)
#include "partition_alloc/pointers/raw_ptr_hookable_impl.h"
#else
#include "partition_alloc/pointers/raw_ptr_noop_impl.h"
#endif

namespace cc {
class ImageDecodeCache;
class Scheduler;
class TextureLayerImpl;
}  // namespace cc
namespace base::internal {
class DelayTimerBase;
class JobTaskSource;
}  // namespace base::internal
namespace base::test {
struct RawPtrCountingImplForTest;
}
namespace content::responsiveness {
class Calculator;
}
namespace v8 {
class JobTask;
}
namespace blink::scheduler {
class MainThreadTaskQueue;
class NonMainThreadTaskQueue;
}  // namespace blink::scheduler
namespace base::sequence_manager::internal {
class TaskQueueImpl;
}
namespace mojo {
class Connector;
}

namespace partition_alloc::internal {

// NOTE: All methods should be `PA_ALWAYS_INLINE`. raw_ptr is meant to be a
// lightweight replacement of a raw pointer, hence performance is critical.

// This is a bitfield representing the different flags that can be applied to a
// raw_ptr.
//
// Internal use only: Developers shouldn't use those values directly.
//
// Housekeeping rules: Try not to change trait values, so that numeric trait
// values stay constant across builds (could be useful e.g. when analyzing stack
// traces). A reasonable exception to this rule are `*ForTest` traits. As a
// matter of fact, we propose that new non-test traits are added before the
// `*ForTest` traits.
enum class RawPtrTraits : unsigned {
  kEmpty = 0,

  // Disables dangling pointer detection, but keeps other raw_ptr protections.
  //
  // Don't use directly, use DisableDanglingPtrDetection or DanglingUntriaged
  // instead.
  kMayDangle = (1 << 0),

  // Disables any hooks, when building with
  // PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL).
  //
  // Internal use only.
  kDisableHooks = (1 << 2),

  // Pointer arithmetic is discouraged and disabled by default.
  //
  // Don't use directly, use AllowPtrArithmetic instead.
  kAllowPtrArithmetic = (1 << 3),

  // This pointer has BRP disabled for experimental rewrites of containers.
  //
  // Don't use directly.
  kDisableBRP = (1 << 4),

  // Uninitialized pointers are discouraged and disabled by default.
  //
  // Don't use directly, use AllowUninitialized instead.
  kAllowUninitialized = (1 << 5),

  // *** ForTest traits below ***

  // Adds accounting, on top of the NoOp implementation, for test purposes.
  // raw_ptr/raw_ref with this trait perform extra bookkeeping, e.g. to track
  // the number of times the raw_ptr is wrapped, unwrapped, etc.
  //
  // Test only. Include raw_ptr_counting_impl_for_test.h in your test
  // files when using this trait.
  kUseCountingImplForTest = (1 << 10),

  // Helper trait that can be used to test raw_ptr's behaviour or conversions.
  //
  // Test only.
  kDummyForTest = (1 << 11),

  kAllMask = kMayDangle | kDisableHooks | kAllowPtrArithmetic | kDisableBRP |
             kAllowUninitialized | kUseCountingImplForTest | kDummyForTest,
};
// Template specialization to use |PA_DEFINE_OPERATORS_FOR_FLAGS| without
// |kMaxValue| declaration.
template <>
constexpr inline RawPtrTraits kAllFlags<RawPtrTraits> = RawPtrTraits::kAllMask;

PA_DEFINE_OPERATORS_FOR_FLAGS(RawPtrTraits);

}  // namespace partition_alloc::internal

namespace base {
using partition_alloc::internal::RawPtrTraits;

namespace raw_ptr_traits {

// IsSupportedType<T>::value answers whether raw_ptr<T>:
//   1) compiles
//   2) is safe at runtime
//
// Templates that may end up using raw_ptr should use IsSupportedType to ensure
// that raw_ptr is not used with unsupported types. As an example, see how
// base::internal::Unretained(Ref)Wrapper uses IsSupportedType to decide whether
// it should use `raw_ptr<T>` or `T*`.
template <typename T>
struct IsSupportedType {
  static constexpr bool value =
      // raw_ptr<T> is not compatible with function pointer types. Also, they
      // don't even need the raw_ptr protection, because they don't point on
      // heap.
      !std::is_function_v<T> &&

#if __OBJC__
      // raw_ptr<T> is not compatible with pointers to Objective-C classes for a
      // multitude of reasons. They may fail to compile in many cases, and
      // wouldn't work well with tagged pointers. Anyway, Objective-C objects
      // have their own way of tracking lifespan, hence don't need the raw_ptr
      // protection as much.
      //
      // Such pointers are detected by checking if they're convertible to |id|
      // type.
      !std::is_convertible_v<T*, id> &&
#endif  // __OBJC__

      // Specific disallowed types.
      !partition_alloc::internal::base::kSameAsAny<
          T,
#if PA_BUILDFLAG(IS_WIN)
// raw_ptr<HWND__> is unsafe at runtime - if the handle happens to also
// represent a valid pointer into a PartitionAlloc-managed region then it can
// lead to manipulating random memory when treating it as BackupRefPtr
// ref-count.  See also https://crbug.com/1262017.
//
// TODO(crbug.com/40799223): Cover other handle types like HANDLE,
// HLOCAL, HINTERNET, or HDEVINFO.  Maybe we should avoid using raw_ptr<T> when
// T=void (as is the case in these handle types).  OTOH, explicit,
// non-template-based raw_ptr<void> should be allowed.  Maybe this can be solved
// by having 2 traits: IsPointeeAlwaysSafe (to be used in templates) and
// IsPointeeUsuallySafe (to be used in the static_assert in raw_ptr).  The
// upside of this approach is that it will safely handle base::Bind closing over
// HANDLE.  The downside of this approach is that base::Bind closing over a
// void* pointer will not get UaF protection.
#define PA_WINDOWS_HANDLE_TYPE(name) name##__,
#include "partition_alloc/partition_alloc_base/win/win_handle_types_list.inc"
#undef PA_WINDOWS_HANDLE_TYPE
#endif
          // Performance-sensitive types identified via sampling profiler data;
          // see crbug.com/1287151
          base::internal::DelayTimerBase,
          cc::Scheduler,
          content::responsiveness::Calculator,

          // Performance-sensitive types identified via speedometer3; see
          // crbug.com/335556942
          base::internal::JobTaskSource,
          base::sequence_manager::internal::TaskQueueImpl,
          blink::scheduler::MainThreadTaskQueue,
          blink::scheduler::NonMainThreadTaskQueue,
          mojo::Connector,
          v8::JobTask,

          // Performance-sensitive types identified via MotionMark; see
          // crbug.com/335556942
          cc::ImageDecodeCache,
          cc::TextureLayerImpl>;
};

#if PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL)
template <RawPtrTraits Traits>
using UnderlyingImplForTraits = internal::RawPtrBackupRefImpl<
    /*AllowDangling=*/partition_alloc::internal::ContainsFlags(
        Traits,
        RawPtrTraits::kMayDangle),
    /*DisableBRP=*/partition_alloc::internal::ContainsFlags(
        Traits,
        RawPtrTraits::kDisableBRP)>;

#elif PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL)
template <RawPtrTraits Traits>
using UnderlyingImplForTraits = internal::RawPtrAsanUnownedImpl<
    partition_alloc::internal::ContainsFlags(Traits,
                                             RawPtrTraits::kAllowPtrArithmetic),
    partition_alloc::internal::ContainsFlags(Traits, RawPtrTraits::kMayDangle)>;

#elif PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL)
template <RawPtrTraits Traits>
using UnderlyingImplForTraits = internal::RawPtrHookableImpl<
    /*EnableHooks=*/!partition_alloc::internal::ContainsFlags(
        Traits,
        RawPtrTraits::kDisableHooks)>;

#else
template <RawPtrTraits Traits>
using UnderlyingImplForTraits = internal::RawPtrNoOpImpl;
#endif

constexpr bool IsPtrArithmeticAllowed([[maybe_unused]] RawPtrTraits Traits) {
#if PA_BUILDFLAG(ENABLE_POINTER_ARITHMETIC_TRAIT_CHECK)
  return partition_alloc::internal::ContainsFlags(
      Traits, RawPtrTraits::kAllowPtrArithmetic);
#else
  return true;
#endif
}

// ImplForTraits is the struct that implements raw_ptr functions. Think of
// raw_ptr as a thin wrapper, that directs calls to ImplForTraits. ImplForTraits
// may be different from UnderlyingImplForTraits, because it may select a
// test impl instead.
template <RawPtrTraits Traits>
using ImplForTraits =
    std::conditional_t<partition_alloc::internal::ContainsFlags(
                           Traits,
                           RawPtrTraits::kUseCountingImplForTest),
                       test::RawPtrCountingImplForTest,
                       UnderlyingImplForTraits<Traits>>;

// `kTypeTraits` is a customization interface to accosiate `T` with some
// `RawPtrTraits`. Users may create specialization of this variable
// to enable some traits by default.
// Note that specialization must be declared before the first use that would
// cause implicit instantiation of `raw_ptr` or `raw_ref`, in every translation
// unit where such use occurs.
template <typename T, typename SFINAE = void>
constexpr inline auto kTypeTraits = RawPtrTraits::kEmpty;

}  // namespace raw_ptr_traits

// `raw_ptr<T>` is a non-owning smart pointer that has improved memory-safety
// over raw pointers. See the documentation for details:
// https://source.chromium.org/chromium/chromium/src/+/main:base/memory/raw_ptr.md
//
// raw_ptr<T> is marked as [[gsl::Pointer]] which allows the compiler to catch
// some bugs where the raw_ptr holds a dangling pointer to a temporary object.
// However the [[gsl::Pointer]] analysis expects that such types do not have a
// non-default move constructor/assignment. Thus, it's possible to get an error
// where the pointer is not actually dangling, and have to work around the
// compiler. We have not managed to construct such an example in Chromium yet.
template <typename T, RawPtrTraits PointerTraits = RawPtrTraits::kEmpty>
class PA_TRIVIAL_ABI PA_GSL_POINTER raw_ptr {
 public:
  // Users may specify `RawPtrTraits` via raw_ptr's second template parameter
  // `PointerTraits`, or specialization of `raw_ptr_traits::kTypeTraits<T>`.
  constexpr static auto Traits = PointerTraits | raw_ptr_traits::kTypeTraits<T>;
  using Impl = typename raw_ptr_traits::ImplForTraits<Traits>;
  // Needed to make gtest Pointee matcher work with raw_ptr.
  using element_type = T;
  using DanglingType = raw_ptr<T, Traits | RawPtrTraits::kMayDangle>;

#if !PA_BUILDFLAG(USE_PARTITION_ALLOC)
  // See comment at top about `PA_RAW_PTR_CHECK()`.
  static_assert(std::is_same_v<Impl, internal::RawPtrNoOpImpl>);
#endif  // !PA_BUILDFLAG(USE_PARTITION_ALLOC)

  static_assert(partition_alloc::internal::AreValidFlags(Traits),
                "Unknown raw_ptr trait(s)");
  static_assert(raw_ptr_traits::IsSupportedType<T>::value,
                "raw_ptr<T> doesn't work with this kind of pointee type T");

  static constexpr bool kZeroOnConstruct =
      Impl::kMustZeroOnConstruct || (PA_BUILDFLAG(RAW_PTR_ZERO_ON_CONSTRUCT) &&
                                     !partition_alloc::internal::ContainsFlags(
                                         Traits,
                                         RawPtrTraits::kAllowUninitialized));
  static constexpr bool kZeroOnMove =
      Impl::kMustZeroOnMove || PA_BUILDFLAG(RAW_PTR_ZERO_ON_MOVE);
  static constexpr bool kZeroOnDestruct =
      Impl::kMustZeroOnDestruct || PA_BUILDFLAG(RAW_PTR_ZERO_ON_DESTRUCT);

// A non-trivial default ctor is required for complex implementations (e.g.
// BackupRefPtr), or even for NoOpImpl when zeroing is requested.
#if PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||   \
    PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL) || \
    PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL) ||     \
    PA_BUILDFLAG(RAW_PTR_ZERO_ON_CONSTRUCT)
  PA_ALWAYS_INLINE constexpr raw_ptr() noexcept {
    if constexpr (kZeroOnConstruct) {
      wrapped_ptr_ = nullptr;
    }
  }
#else
  // raw_ptr can be trivially default constructed (leaving |wrapped_ptr_|
  // uninitialized).
  PA_ALWAYS_INLINE constexpr raw_ptr() noexcept = default;
  static_assert(!kZeroOnConstruct);
#endif  // PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||
        // PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL) ||
        // PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL) ||
        // PA_BUILDFLAG(RAW_PTR_ZERO_ON_CONSTRUCT)

// A non-trivial copy ctor and assignment operator are required for complex
// implementations (e.g. BackupRefPtr). Unlike the blocks around, we don't need
// these for NoOpImpl even when zeroing is requested; better to keep them
// trivial.
#if PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||   \
    PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL) || \
    PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL)
  PA_ALWAYS_INLINE constexpr raw_ptr(const raw_ptr& p) noexcept
      : wrapped_ptr_(Impl::Duplicate(p.wrapped_ptr_)) {
    Impl::Trace(tracer_.owner_id(), p.wrapped_ptr_);
  }

  PA_ALWAYS_INLINE constexpr raw_ptr& operator=(const raw_ptr& p) noexcept {
    // Increment the ref-count first before releasing, in case the pointer is
    // assigned to itself. (This is different from the concern in the assign-T*
    // version of this operator, where a different pointer to the same allocator
    // slot could cause trouble, which isn't a concern here at all.)
    //
    // Unlike the move version of this operator, don't add |this != &p| branch,
    // for performance reasons. Self-assignment is rare, so unconditionally
    // calling `Duplicate()` is almost certainly cheaper than adding an
    // additional branch, even if always correctly predicted.
    T* new_ptr = Impl::Duplicate(p.wrapped_ptr_);
    Impl::ReleaseWrappedPtr(wrapped_ptr_);
    Impl::Untrace(tracer_.owner_id());
    wrapped_ptr_ = new_ptr;
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
    return *this;
  }
#else
  PA_ALWAYS_INLINE raw_ptr(const raw_ptr&) noexcept = default;
  PA_ALWAYS_INLINE raw_ptr& operator=(const raw_ptr&) noexcept = default;
#endif  // PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||
        // PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL) ||
        // PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL)

// A non-trivial move ctor and assignment operator are required for complex
// implementations (e.g. BackupRefPtr), or even for NoOpImpl when zeroing is
// requested.
#if PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||   \
    PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL) || \
    PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL) ||     \
    PA_BUILDFLAG(RAW_PTR_ZERO_ON_MOVE)
  PA_ALWAYS_INLINE constexpr raw_ptr(raw_ptr&& p) noexcept {
    wrapped_ptr_ = p.wrapped_ptr_;
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
    if constexpr (kZeroOnMove) {
      p.wrapped_ptr_ = nullptr;
      Impl::Untrace(p.tracer_.owner_id());
    }
  }
  PA_ALWAYS_INLINE constexpr raw_ptr& operator=(raw_ptr&& p) noexcept {
    // Unlike the the copy version of this operator, this branch is necessary
    // for correctness.
    if (this != &p) [[likely]] {
      Impl::ReleaseWrappedPtr(wrapped_ptr_);
      Impl::Untrace(tracer_.owner_id());
      wrapped_ptr_ = p.wrapped_ptr_;
      Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
      if constexpr (kZeroOnMove) {
        p.wrapped_ptr_ = nullptr;
        Impl::Untrace(p.tracer_.owner_id());
      }
    }
    return *this;
  }
#else
  PA_ALWAYS_INLINE raw_ptr(raw_ptr&&) noexcept = default;
  PA_ALWAYS_INLINE raw_ptr& operator=(raw_ptr&&) noexcept = default;
  static_assert(!kZeroOnMove);
#endif  // PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||
        // PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL) ||
        // PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL) ||
        // PA_BUILDFLAG(RAW_PTR_ZERO_ON_MOVE)

// A non-trivial default dtor is required for complex implementations (e.g.
// BackupRefPtr), or even for NoOpImpl when zeroing is requested.
#if PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||   \
    PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL) || \
    PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL) ||     \
    PA_BUILDFLAG(RAW_PTR_ZERO_ON_DESTRUCT)
  PA_ALWAYS_INLINE PA_CONSTEXPR_DTOR ~raw_ptr() noexcept {
    Impl::ReleaseWrappedPtr(wrapped_ptr_);
    Impl::Untrace(tracer_.owner_id());
    // Work around external issues where raw_ptr is used after destruction.
    if constexpr (kZeroOnDestruct) {
      wrapped_ptr_ = nullptr;
    }
  }
#else
  PA_ALWAYS_INLINE ~raw_ptr() noexcept = default;
  static_assert(!kZeroOnDestruct);
#endif  // PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL) ||
        // PA_BUILDFLAG(USE_RAW_PTR_ASAN_UNOWNED_IMPL) ||
        // PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL) ||
        // PA_BUILDFLAG(RAW_PTR_ZERO_ON_DESTRUCT)

  // Cross-kind copy constructor.
  // Move is not supported as different traits may use different ref-counts, so
  // let move operations degrade to copy, which handles it well.
  template <RawPtrTraits PassedTraits,
            typename = std::enable_if_t<Traits != PassedTraits>>
  PA_ALWAYS_INLINE constexpr explicit raw_ptr(
      const raw_ptr<T, PassedTraits>& p) noexcept
      : wrapped_ptr_(Impl::WrapRawPtrForDuplication(
            raw_ptr_traits::ImplForTraits<raw_ptr<T, PassedTraits>::Traits>::
                UnsafelyUnwrapPtrForDuplication(p.wrapped_ptr_))) {
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
    // Limit cross-kind conversions only to cases where `kMayDangle` gets added,
    // because that's needed for ExtractAsDangling() and Unretained(Ref)Wrapper.
    // Use a static_assert, instead of disabling via SFINAE, so that the
    // compiler catches other conversions. Otherwise the implicits
    // `raw_ptr<T> -> T* -> raw_ptr<>` route will be taken.
    static_assert(Traits == (raw_ptr<T, PassedTraits>::Traits |
                             RawPtrTraits::kMayDangle));
  }

  // Cross-kind assignment.
  // Move is not supported as different traits may use different ref-counts, so
  // let move operations degrade to copy, which handles it well.
  template <RawPtrTraits PassedTraits,
            typename = std::enable_if_t<Traits != PassedTraits>>
  PA_ALWAYS_INLINE constexpr raw_ptr& operator=(
      const raw_ptr<T, PassedTraits>& p) noexcept {
    // Limit cross-kind assignments only to cases where `kMayDangle` gets added,
    // because that's needed for ExtractAsDangling() and Unretained(Ref)Wrapper.
    // Use a static_assert, instead of disabling via SFINAE, so that the
    // compiler catches other conversions. Otherwise the implicit
    // `raw_ptr<T> -> T* -> raw_ptr<>` route will be taken.
    static_assert(Traits == (raw_ptr<T, PassedTraits>::Traits |
                             RawPtrTraits::kMayDangle));
    // If it was the same type, another overload would've been used.
    static_assert(!std::is_same_v<raw_ptr, std::decay_t<decltype(p)>>);

    // Unlike the regular varsion of operator=, we don't have an issue of
    // `*this` and `ptr` being the same object (because it isn't even the same
    // type, as asserted above), so no need to increment the ref-count first.
    Impl::ReleaseWrappedPtr(wrapped_ptr_);
    Impl::Untrace(tracer_.owner_id());
    wrapped_ptr_ = Impl::WrapRawPtrForDuplication(
        raw_ptr_traits::ImplForTraits<raw_ptr<T, PassedTraits>::Traits>::
            UnsafelyUnwrapPtrForDuplication(p.wrapped_ptr_));
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
    return *this;
  }

  // Deliberately implicit, because raw_ptr is supposed to resemble raw ptr.
  // Ignore kZeroOnConstruct, because here the caller explicitly wishes to
  // initialize with nullptr.
  // NOLINTNEXTLINE(google-explicit-constructor)
  PA_ALWAYS_INLINE constexpr raw_ptr(std::nullptr_t) noexcept
      : wrapped_ptr_(nullptr) {}

  // Deliberately implicit, because raw_ptr is supposed to resemble raw ptr.
  // NOLINTNEXTLINE(google-explicit-constructor)
  PA_ALWAYS_INLINE constexpr raw_ptr(T* p) noexcept
      : wrapped_ptr_(Impl::WrapRawPtr(p)) {
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
  }

  // Deliberately implicit in order to support implicit upcast.
  template <typename U,
            typename = std::enable_if_t<
                std::is_convertible_v<U*, T*> &&
                !std::is_void_v<typename std::remove_cv<T>::type>>>
  // NOLINTNEXTLINE(google-explicit-constructor)
  PA_ALWAYS_INLINE constexpr raw_ptr(const raw_ptr<U, Traits>& ptr) noexcept
      : wrapped_ptr_(
            Impl::Duplicate(Impl::template Upcast<T, U>(ptr.wrapped_ptr_))) {
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
  }
  // Deliberately implicit in order to support implicit upcast.
  template <typename U,
            typename = std::enable_if_t<
                std::is_convertible_v<U*, T*> &&
                !std::is_void_v<typename std::remove_cv<T>::type>>>
  // NOLINTNEXTLINE(google-explicit-constructor)
  PA_ALWAYS_INLINE constexpr raw_ptr(raw_ptr<U, Traits>&& ptr) noexcept
      : wrapped_ptr_(Impl::template Upcast<T, U>(ptr.wrapped_ptr_)) {
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
    if constexpr (kZeroOnMove) {
      ptr.wrapped_ptr_ = nullptr;
      Impl::Untrace(ptr.tracer_.owner_id());
    }
  }

  PA_ALWAYS_INLINE constexpr raw_ptr& operator=(std::nullptr_t) noexcept {
    Impl::ReleaseWrappedPtr(wrapped_ptr_);
    Impl::Untrace(tracer_.owner_id());
    wrapped_ptr_ = nullptr;
    return *this;
  }
  PA_ALWAYS_INLINE constexpr raw_ptr& operator=(T* p) noexcept {
    // Duplicate before releasing, in case the pointers point to the same
    // allocator slot. Releasing the pointer first could lead to dropping the
    // ref-count to 0 for the slot, immediately unqurantining and releasing it,
    // just to immediately reacquire the the ref-count on that slot, leading to
    // correctness issues.
    T* new_ptr = Impl::WrapRawPtr(p);
    Impl::ReleaseWrappedPtr(wrapped_ptr_);
    Impl::Untrace(tracer_.owner_id());
    wrapped_ptr_ = new_ptr;
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
    return *this;
  }

  // Upcast assignment
  template <typename U,
            typename = std::enable_if_t<
                std::is_convertible_v<U*, T*> &&
                !std::is_void_v<typename std::remove_cv<T>::type>>>
  PA_ALWAYS_INLINE constexpr raw_ptr& operator=(
      const raw_ptr<U, Traits>& ptr) noexcept {
    // If it was the same type, another overload would've been used.
    static_assert(!std::is_same_v<raw_ptr, std::decay_t<decltype(ptr)>>);

    // Unlike the regular varsion of operator=, we don't have an issue of
    // `*this` and `ptr` being the same object (because it isn't even the same
    // type, as asserted above), so no need to increment the ref-count first.
    Impl::ReleaseWrappedPtr(wrapped_ptr_);
    Impl::Untrace(tracer_.owner_id());
    wrapped_ptr_ =
        Impl::Duplicate(Impl::template Upcast<T, U>(ptr.wrapped_ptr_));
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
    return *this;
  }
  template <typename U,
            typename = std::enable_if_t<
                std::is_convertible_v<U*, T*> &&
                !std::is_void_v<typename std::remove_cv<T>::type>>>
  PA_ALWAYS_INLINE constexpr raw_ptr& operator=(
      raw_ptr<U, Traits>&& ptr) noexcept {
    // If it was the same type, another overload would've been used.
    static_assert(!std::is_same_v<raw_ptr, std::decay_t<decltype(ptr)>>);

    // Unlike the regular varsion of operator=, we don't have an issue of
    // `*this` and `ptr` being the same object (because it isn't even the same
    // type, as asserted above), so no need to increment the ref-count first.
    Impl::ReleaseWrappedPtr(wrapped_ptr_);
    Impl::Untrace(tracer_.owner_id());
    wrapped_ptr_ = Impl::template Upcast<T, U>(ptr.wrapped_ptr_);
    Impl::Trace(tracer_.owner_id(), wrapped_ptr_);
    if constexpr (kZeroOnMove) {
      ptr.wrapped_ptr_ = nullptr;
      Impl::Untrace(ptr.tracer_.owner_id());
    }
    return *this;
  }

  // Avoid using. The goal of raw_ptr is to be as close to raw pointer as
  // possible, so use it only if absolutely necessary (e.g. for const_cast).
  PA_ALWAYS_INLINE constexpr T* get() const { return GetForExtraction(); }

  // You may use |raw_ptr<T>::AsEphemeralRawAddr()| to obtain |T**| or |T*&|
  // from |raw_ptr<T>|, as long as you follow these requirements:
  // - DO NOT carry T**/T*& obtained via AsEphemeralRawAddr() out of
  //   expression.
  // - DO NOT use raw_ptr or T**/T*& multiple times within an expression.
  //
  // https://chromium.googlesource.com/chromium/src/+/main/base/memory/raw_ptr.md#in_out-arguments-need-to-be-refactored
  class EphemeralRawAddr {
   public:
    EphemeralRawAddr(const EphemeralRawAddr&) = delete;
    EphemeralRawAddr& operator=(const EphemeralRawAddr&) = delete;
    void* operator new(size_t) = delete;
    void* operator new(size_t, void*) = delete;
    PA_ALWAYS_INLINE PA_CONSTEXPR_DTOR ~EphemeralRawAddr() { original = copy; }

    PA_ALWAYS_INLINE constexpr T** operator&() && PA_LIFETIME_BOUND {
      return &copy;
    }
    // NOLINTNEXTLINE(google-explicit-constructor)
    PA_ALWAYS_INLINE constexpr operator T*&() && PA_LIFETIME_BOUND {
      return copy;
    }

   private:
    friend class raw_ptr;
    PA_ALWAYS_INLINE constexpr explicit EphemeralRawAddr(raw_ptr& ptr)
        : copy(ptr.get()), original(ptr) {}
    T* copy;
    raw_ptr& original;  // Original pointer.
  };
  PA_ALWAYS_INLINE PA_CONSTEXPR_DTOR EphemeralRawAddr AsEphemeralRawAddr() & {
    return EphemeralRawAddr(*this);
  }

  PA_ALWAYS_INLINE constexpr explicit operator bool() const {
    return !!wrapped_ptr_;
  }

  template <typename U = T,
            typename = std::enable_if_t<
                !std::is_void_v<typename std::remove_cv<U>::type>>>
  PA_ALWAYS_INLINE constexpr U& operator*() const {
    return *GetForDereference();
  }
  PA_ALWAYS_INLINE constexpr T* operator->() const {
    return GetForDereference();
  }

  // Deliberately implicit, because raw_ptr is supposed to resemble raw ptr.
  // NOLINTNEXTLINE(google-explicit-constructor)
  PA_ALWAYS_INLINE constexpr operator T*() const { return GetForExtraction(); }
  template <typename U>
  PA_ALWAYS_INLINE constexpr explicit operator U*() const {
    // This operator may be invoked from static_cast, meaning the types may not
    // be implicitly convertible, hence the need for static_cast here.
    return static_cast<U*>(GetForExtraction());
  }

  PA_ALWAYS_INLINE constexpr raw_ptr& operator++() {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot increment raw_ptr unless AllowPtrArithmetic trait is present.");
    wrapped_ptr_ = Impl::Advance(wrapped_ptr_, 1, true);
    return *this;
  }
  PA_ALWAYS_INLINE constexpr raw_ptr& operator--() {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot decrement raw_ptr unless AllowPtrArithmetic trait is present.");
    wrapped_ptr_ = Impl::Retreat(wrapped_ptr_, 1, true);
    return *this;
  }
  PA_ALWAYS_INLINE constexpr raw_ptr operator++(int /* post_increment */) {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot increment raw_ptr unless AllowPtrArithmetic trait is present.");
    raw_ptr result = *this;
    ++(*this);
    return result;
  }
  PA_ALWAYS_INLINE constexpr raw_ptr operator--(int /* post_decrement */) {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot decrement raw_ptr unless AllowPtrArithmetic trait is present.");
    raw_ptr result = *this;
    --(*this);
    return result;
  }
  template <
      typename Z,
      typename = std::enable_if_t<partition_alloc::internal::is_offset_type<Z>>>
  PA_ALWAYS_INLINE constexpr raw_ptr& operator+=(Z delta_elems) {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot increment raw_ptr unless AllowPtrArithmetic trait is present.");
    wrapped_ptr_ = Impl::Advance(wrapped_ptr_, delta_elems, true);
    return *this;
  }
  template <
      typename Z,
      typename = std::enable_if_t<partition_alloc::internal::is_offset_type<Z>>>
  PA_ALWAYS_INLINE constexpr raw_ptr& operator-=(Z delta_elems) {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot decrement raw_ptr unless AllowPtrArithmetic trait is present.");
    wrapped_ptr_ = Impl::Retreat(wrapped_ptr_, delta_elems, true);
    return *this;
  }

  template <typename Z,
            typename U = T,
            typename = std::enable_if_t<
                !std::is_void_v<typename std::remove_cv<U>::type> &&
                partition_alloc::internal::is_offset_type<Z>>>
  PA_ALWAYS_INLINE constexpr U& operator[](Z delta_elems) const {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot index raw_ptr unless AllowPtrArithmetic trait is present.");
    // Call SafelyUnwrapPtrForDereference() to simulate what GetForDereference()
    // does, but without creating a temporary.
    return *Impl::SafelyUnwrapPtrForDereference(
        Impl::Advance(wrapped_ptr_, delta_elems, false));
  }

  // Do not disable operator+() and operator-().
  // They provide OOB checks, which prevent from assigning an arbitrary value to
  // raw_ptr, leading BRP to modifying arbitrary memory thinking it's ref-count.
  // Keep them enabled, which may be blocked later when attempting to apply the
  // += or -= operation, when disabled. In the absence of operators +/-, the
  // compiler is free to implicitly convert to the underlying T* representation
  // and perform ordinary pointer arithmetic, thus invalidating the purpose
  // behind disabling them.
  //
  // For example, disabling these when `!is_offset_type<Z>` would remove the
  // operators for Z=uint64_t on 32-bit systems. The compiler instead would
  // generate code that converts `raw_ptr<T>` to `T*` and adds uint64_t to that,
  // bypassing the OOB protection entirely.
  template <typename Z>
  PA_ALWAYS_INLINE friend constexpr raw_ptr operator+(const raw_ptr& p,
                                                      Z delta_elems) {
    // Don't check `is_offset_type<Z>` here, as existence of `Advance` is
    // already gated on that, and we'd get double errors.
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot add to raw_ptr unless AllowPtrArithmetic trait is present.");
    raw_ptr result = Impl::Advance(p.wrapped_ptr_, delta_elems, false);
    return result;
  }
  template <typename Z>
  PA_ALWAYS_INLINE friend constexpr raw_ptr operator+(Z delta_elems,
                                                      const raw_ptr& p) {
    return p + delta_elems;
  }
  template <typename Z>
  PA_ALWAYS_INLINE friend constexpr raw_ptr operator-(const raw_ptr& p,
                                                      Z delta_elems) {
    // Don't check `is_offset_type<Z>` here, as existence of `Retreat` is
    // already gated on that, and we'd get double errors.
    static_assert(raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
                  "cannot subtract from raw_ptr unless AllowPtrArithmetic "
                  "trait is present.");
    raw_ptr result = Impl::Retreat(p.wrapped_ptr_, delta_elems, false);
    return result;
  }

  // The "Do not disable operator+() and operator-()" comment above doesn't
  // apply to the delta operator-() below.
  PA_ALWAYS_INLINE friend constexpr ptrdiff_t operator-(const raw_ptr& p1,
                                                        const raw_ptr& p2) {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot subtract raw_ptrs unless AllowPtrArithmetic trait is present.");
    return Impl::GetDeltaElems(p1.wrapped_ptr_, p2.wrapped_ptr_);
  }
  PA_ALWAYS_INLINE friend constexpr ptrdiff_t operator-(T* p1,
                                                        const raw_ptr& p2) {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot subtract raw_ptrs unless AllowPtrArithmetic trait is present.");
    return Impl::GetDeltaElems(p1, p2.wrapped_ptr_);
  }
  PA_ALWAYS_INLINE friend constexpr ptrdiff_t operator-(const raw_ptr& p1,
                                                        T* p2) {
    static_assert(
        raw_ptr_traits::IsPtrArithmeticAllowed(Traits),
        "cannot subtract raw_ptrs unless AllowPtrArithmetic trait is present.");
    return Impl::GetDeltaElems(p1.wrapped_ptr_, p2);
  }

  // Stop referencing the underlying pointer and free its memory. Compared to
  // raw delete calls, this avoids the raw_ptr to be temporarily dangling
  // during the free operation, which will lead to taking the slower path that
  // involves quarantine.
  PA_ALWAYS_INLINE constexpr void ClearAndDelete() noexcept {
    delete GetForExtractionAndReset();
  }
  PA_ALWAYS_INLINE constexpr void ClearAndDeleteArray() noexcept {
    delete[] GetForExtractionAndReset();
  }

  // Clear the underlying pointer and return another raw_ptr instance
  // that is allowed to dangle.
  // This can be useful in cases such as:
  // ```
  //  ptr.ExtractAsDangling()->SelfDestroy();
  // ```
  // ```
  //  c_style_api_do_something_and_destroy(ptr.ExtractAsDangling());
  // ```
  // NOTE, avoid using this method as it indicates an error-prone memory
  // ownership pattern. If possible, use smart pointers like std::unique_ptr<>
  // instead of raw_ptr<>.
  // If you have to use it, avoid saving the return value in a long-lived
  // variable (or worse, a field)! It's meant to be used as a temporary, to be
  // passed into a cleanup & freeing function, and destructed at the end of the
  // statement.
  PA_ALWAYS_INLINE constexpr DanglingType ExtractAsDangling() noexcept {
    DanglingType res(std::move(*this));
    // Not all implementation clear the source pointer on move. Furthermore,
    // even for implemtantions that do, cross-kind conversions (that add
    // kMayDangle) fall back to a copy, instead of move. So do it here just in
    // case. Should be cheap.
    operator=(nullptr);
    return res;
  }

  // Comparison operators between raw_ptr and raw_ptr<U>/U*/std::nullptr_t.
  // Strictly speaking, it is not necessary to provide these: the compiler can
  // use the conversion operator implicitly to allow comparisons to fall back to
  // comparisons between raw pointers. However, `operator T*`/`operator U*` may
  // perform safety checks with a higher runtime cost, so to avoid this, provide
  // explicit comparison operators for all combinations of parameters.

  // Comparisons between `raw_ptr`s. Typically, these would be defined inline as
  // comparisons between `raw_ptr` and `raw_ptr<U>`. Unfortunately, the friend
  // declaration grants access to `raw_ptr::GetForComparison()`, but not
  // `raw_ptr<U>::GetForComparison()`, since that's an unrelated type; both
  // instantiations must declare the same signature as a friend for it to access
  // both private methods. Switching to `raw_ptr<U>, raw_ptr<V>` achieves this,
  // but then if the implementation is inline, the compile will generate it for
  // both instantiations, and not know which (identical) instance to resolve to,
  // causing a compile error. Thus the definitions must also be out-of-lined
  // below, so they are only instantiated once.
  template <typename U, typename V, RawPtrTraits R1, RawPtrTraits R2>
  friend constexpr bool operator==(const raw_ptr<U, R1>& lhs,
                                   const raw_ptr<V, R2>& rhs);
  template <typename U, typename V, RawPtrTraits R1, RawPtrTraits R2>
  friend constexpr bool operator!=(const raw_ptr<U, R1>& lhs,
                                   const raw_ptr<V, R2>& rhs);
#if PA_HAVE_SPACESHIP_OPERATOR
  template <typename U, typename V, RawPtrTraits R1, RawPtrTraits R2>
  friend constexpr auto operator<=>(const raw_ptr<U, R1>& lhs,
                                    const raw_ptr<V, R2>& rhs);
#else
  template <typename U, typename V, RawPtrTraits R1, RawPtrTraits R2>
  friend constexpr bool operator<(const raw_ptr<U, R1>& lhs,
                                  const raw_ptr<V, R2>& rhs);
  template <typename U, typename V, RawPtrTraits R1, RawPtrTraits R2>
  friend constexpr bool operator>(const raw_ptr<U, R1>& lhs,
                                  const raw_ptr<V, R2>& rhs);
  template <typename U, typename V, RawPtrTraits R1, RawPtrTraits R2>
  friend constexpr bool operator<=(const raw_ptr<U, R1>& lhs,
                                   const raw_ptr<V, R2>& rhs);
  template <typename U, typename V, RawPtrTraits R1, RawPtrTraits R2>
  friend constexpr bool operator>=(const raw_ptr<U, R1>& lhs,
                                   const raw_ptr<V, R2>& rhs);
#endif

  // Comparisons with U*. These operators also handle the case where the RHS is
  // T*. Because these only call `raw_ptr::GetForComparison()`, they can be
  // written inline in the typical way.
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator==(const raw_ptr& lhs,
                                                    U* rhs) {
    return lhs.GetForComparison() == rhs;
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator!=(const raw_ptr& lhs,
                                                    U* rhs) {
    return !(lhs == rhs);
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator==(U* lhs,
                                                    const raw_ptr& rhs) {
    return rhs == lhs;  // Reverse order to call the operator above.
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator!=(U* lhs,
                                                    const raw_ptr& rhs) {
    return rhs != lhs;  // Reverse order to call the operator above.
  }
#if PA_HAVE_SPACESHIP_OPERATOR
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr auto operator<=>(const raw_ptr& lhs,
                                                     U* rhs) {
    return lhs.GetForComparison() <=> rhs;
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr auto operator<=>(U* lhs,
                                                     const raw_ptr& rhs) {
    return lhs <=> rhs.GetForComparison();
  }
#else
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator<(const raw_ptr& lhs, U* rhs) {
    return lhs.GetForComparison() < rhs;
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator<=(const raw_ptr& lhs,
                                                    U* rhs) {
    return lhs.GetForComparison() <= rhs;
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator>(const raw_ptr& lhs, U* rhs) {
    return lhs.GetForComparison() > rhs;
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator>=(const raw_ptr& lhs,
                                                    U* rhs) {
    return lhs.GetForComparison() >= rhs;
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator<(U* lhs, const raw_ptr& rhs) {
    return lhs < rhs.GetForComparison();
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator<=(U* lhs,
                                                    const raw_ptr& rhs) {
    return lhs <= rhs.GetForComparison();
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator>(U* lhs, const raw_ptr& rhs) {
    return lhs > rhs.GetForComparison();
  }
  template <typename U>
  PA_ALWAYS_INLINE friend constexpr bool operator>=(U* lhs,
                                                    const raw_ptr& rhs) {
    return lhs >= rhs.GetForComparison();
  }
#endif

  // Comparisons with `std::nullptr_t`.
  PA_ALWAYS_INLINE friend constexpr bool operator==(const raw_ptr& lhs,
                                                    std::nullptr_t) {
    return !lhs;
  }
  PA_ALWAYS_INLINE friend constexpr bool operator!=(const raw_ptr& lhs,
                                                    std::nullptr_t) {
    return !!lhs;  // Use !! otherwise the costly implicit cast will be used.
  }
  PA_ALWAYS_INLINE friend constexpr bool operator==(std::nullptr_t,
                                                    const raw_ptr& rhs) {
    return !rhs;
  }
  PA_ALWAYS_INLINE friend constexpr bool operator!=(std::nullptr_t,
                                                    const raw_ptr& rhs) {
    return !!rhs;  // Use !! otherwise the costly implicit cast will be used.
  }

  PA_ALWAYS_INLINE friend constexpr void swap(raw_ptr& lhs,
                                              raw_ptr& rhs) noexcept {
    Impl::IncrementSwapCountForTest();
    std::swap(lhs.wrapped_ptr_, rhs.wrapped_ptr_);
  }

  PA_ALWAYS_INLINE void ReportIfDangling() const noexcept {
#if PA_BUILDFLAG(USE_RAW_PTR_BACKUP_REF_IMPL)
    Impl::ReportIfDangling(wrapped_ptr_);
#endif
  }

 private:
  // This getter is meant for situations where the pointer is meant to be
  // dereferenced. It is allowed to crash on nullptr (it may or may not),
  // because it knows that the caller will crash on nullptr.
  PA_ALWAYS_INLINE constexpr T* GetForDereference() const {
    return Impl::SafelyUnwrapPtrForDereference(wrapped_ptr_);
  }
  // This getter is meant for situations where the raw pointer is meant to be
  // extracted outside of this class, but not necessarily with an intention to
  // dereference. It mustn't crash on nullptr.
  PA_ALWAYS_INLINE constexpr T* GetForExtraction() const {
    return Impl::SafelyUnwrapPtrForExtraction(wrapped_ptr_);
  }
  // This getter is meant *only* for situations where the pointer is meant to be
  // compared (guaranteeing no dereference or extraction outside of this class).
  // Any verifications can and should be skipped for performance reasons.
  PA_ALWAYS_INLINE constexpr T* GetForComparison() const {
    return Impl::UnsafelyUnwrapPtrForComparison(wrapped_ptr_);
  }

  PA_ALWAYS_INLINE constexpr T* GetForExtractionAndReset() {
    T* ptr = GetForExtraction();
    operator=(nullptr);
    return ptr;
  }

  T* wrapped_ptr_;
  PA_NO_UNIQUE_ADDRESS internal::InstanceTracer tracer_;

  template <typename U, base::RawPtrTraits R>
  friend class raw_ptr;
};

template <typename U, typename V, RawPtrTraits Traits1, RawPtrTraits Traits2>
PA_ALWAYS_INLINE constexpr bool operator==(const raw_ptr<U, Traits1>& lhs,
                                           const raw_ptr<V, Traits2>& rhs) {
  return lhs.GetForComparison() == rhs.GetForComparison();
}

template <typename U, typename V, RawPtrTraits Traits1, RawPtrTraits Traits2>
PA_ALWAYS_INLINE constexpr bool operator!=(const raw_ptr<U, Traits1>& lhs,
                                           const raw_ptr<V, Traits2>& rhs) {
  return !(lhs == rhs);
}

#if PA_HAVE_SPACESHIP_OPERATOR
template <typename U, typename V, RawPtrTraits Traits1, RawPtrTraits Traits2>
PA_ALWAYS_INLINE constexpr auto operator<=>(const raw_ptr<U, Traits1>& lhs,
                                            const raw_ptr<V, Traits2>& rhs) {
  return lhs.GetForComparison() <=> rhs.GetForComparison();
}
#else
template <typename U, typename V, RawPtrTraits Traits1, RawPtrTraits Traits2>
PA_ALWAYS_INLINE constexpr bool operator<(const raw_ptr<U, Traits1>& lhs,
                                          const raw_ptr<V, Traits2>& rhs) {
  return lhs.GetForComparison() < rhs.GetForComparison();
}

template <typename U, typename V, RawPtrTraits Traits1, RawPtrTraits Traits2>
PA_ALWAYS_INLINE constexpr bool operator>(const raw_ptr<U, Traits1>& lhs,
                                          const raw_ptr<V, Traits2>& rhs) {
  return lhs.GetForComparison() > rhs.GetForComparison();
}

template <typename U, typename V, RawPtrTraits Traits1, RawPtrTraits Traits2>
PA_ALWAYS_INLINE constexpr bool operator<=(const raw_ptr<U, Traits1>& lhs,
                                           const raw_ptr<V, Traits2>& rhs) {
  return lhs.GetForComparison() <= rhs.GetForComparison();
}

template <typename U, typename V, RawPtrTraits Traits1, RawPtrTraits Traits2>
PA_ALWAYS_INLINE constexpr bool operator>=(const raw_ptr<U, Traits1>& lhs,
                                           const raw_ptr<V, Traits2>& rhs) {
  return lhs.GetForComparison() >= rhs.GetForComparison();
}
#endif

template <typename T>
inline constexpr bool IsRawPtr = false;
template <typename T, RawPtrTraits Traits>
inline constexpr bool IsRawPtr<raw_ptr<T, Traits>> = true;

template <typename T>
inline constexpr bool IsRawPtrMayDangle = false;
template <typename T, RawPtrTraits Traits>
inline constexpr bool IsRawPtrMayDangle<raw_ptr<T, Traits>> =
    partition_alloc::internal::ContainsFlags(Traits, RawPtrTraits::kMayDangle);

template <typename T>
inline constexpr bool IsPointerOrRawPtr = std::is_pointer_v<T>;
template <typename T, RawPtrTraits Traits>
inline constexpr bool IsPointerOrRawPtr<raw_ptr<T, Traits>> = true;

// Like `std::remove_pointer_t<>`, but also converts `raw_ptr<T>` => `T`.
template <typename T>
struct RemovePointer {
  using type = std::remove_pointer_t<T>;
};
template <typename T, RawPtrTraits Traits>
struct RemovePointer<raw_ptr<T, Traits>> {
  using type = T;
};
template <typename T>
using RemovePointerT = typename RemovePointer<T>::type;

}  // namespace base

using base::raw_ptr;

// DisableDanglingPtrDetection option for raw_ptr annotates
// "intentional-and-safe" dangling pointers. It is meant to be used at the
// margin, only if there is no better way to re-architecture the code.
//
// Usage:
// raw_ptr<T, DisableDanglingPtrDetection> dangling_ptr;
//
// When using it, please provide a justification about what guarantees that it
// will never be dereferenced after becoming dangling.
constexpr inline auto DisableDanglingPtrDetection =
    base::RawPtrTraits::kMayDangle;

// See `docs/dangling_ptr.md`
// Annotates known dangling raw_ptr. Those haven't been triaged yet. All the
// occurrences are meant to be removed. See https://crbug.com/1291138.
constexpr inline auto DanglingUntriaged = base::RawPtrTraits::kMayDangle;

// Unlike DanglingUntriaged, this annotates raw_ptrs that are known to
// dangle only occasionally on the CQ.
//
// These were found from CQ runs and analysed in this dashboard:
// https://docs.google.com/spreadsheets/d/1k12PQOG4y1-UEV9xDfP1F8FSk4cVFywafEYHmzFubJ8/
//
// This is not meant to be added manually. You can ignore this flag.
constexpr inline auto FlakyDanglingUntriaged = base::RawPtrTraits::kMayDangle;

// Dangling raw_ptr that is more likely to cause UAF: its memory was freed in
// one task, and the raw_ptr was released in a different one.
//
// This is not meant to be added manually. You can ignore this flag.
constexpr inline auto AcrossTasksDanglingUntriaged =
    base::RawPtrTraits::kMayDangle;

// The use of pointer arithmetic with raw_ptr is strongly discouraged and
// disabled by default. Usually a container like span<> should be used
// instead of the raw_ptr.
constexpr inline auto AllowPtrArithmetic =
    base::RawPtrTraits::kAllowPtrArithmetic;

// The use of uninitialized pointers is strongly discouraged. raw_ptrs will
// be initialized to nullptr by default in all cases when building against
// Chromium. However, third-party projects built in a standalone manner may
// wish to opt out where possible. One way to do this is via buildflags,
// thus affecting all raw_ptrs, but a finer-grained mechanism is the use
// of the kAllowUninitialized trait.
//
// Note that opting out may not always be effective, given that algorithms
// like BackupRefPtr require nullptr initializaion for correctness and thus
// silently enforce it.
constexpr inline auto AllowUninitialized =
    base::RawPtrTraits::kAllowUninitialized;

// This flag is used to tag a subset of dangling pointers. Similarly to
// DanglingUntriaged, those pointers are known to be dangling. However, we also
// detected that those raw_ptr's were never released (either by calling
// raw_ptr's destructor or by resetting its value), which can ultimately put
// pressure on the BRP quarantine.
//
// This is not meant to be added manually. You can ignore this flag.
constexpr inline auto LeakedDanglingUntriaged = base::RawPtrTraits::kMayDangle;

// Temporary introduced alias in the context of rewriting std::vector<T*> into
// std::vector<raw_ptr<T>> and in order to temporarily bypass the dangling ptr
// checks on the CQ. This alias will be removed gradually after the cl lands and
// will be replaced by DanglingUntriaged where necessary.
constexpr inline auto VectorExperimental = base::RawPtrTraits::kMayDangle;

// Temporary alias introduced in the context of rewriting std::set<T*> into
// std::set<raw_ptr<T>> and in order to temporarily bypass the dangling ptr
// checks on the CQ. This alias will be removed gradually after the rewrite cl
// lands and will be replaced by DanglingUntriaged where necessary.
constexpr inline auto SetExperimental = base::RawPtrTraits::kMayDangle;

// Temporary alias introduced in the context of rewriting more containers and in
// order to temporarily bypass the dangling ptr checks on the CQ. This alias
// will be removed gradually after the rewrite cl lands and will be replaced by
// DanglingUntriaged where necessary.
constexpr inline auto CtnExperimental = base::RawPtrTraits::kMayDangle;

// Public verson used in callbacks arguments when it is known that they might
// receive dangling pointers. In any other cases, please
// use one of:
// - raw_ptr<T, DanglingUntriaged>
// - raw_ptr<T, DisableDanglingPtrDetection>
template <typename T, base::RawPtrTraits Traits = base::RawPtrTraits::kEmpty>
using MayBeDangling = base::raw_ptr<T, Traits | base::RawPtrTraits::kMayDangle>;

namespace std {

// Override so set/map lookups do not create extra raw_ptr. This also allows
// dangling pointers to be used for lookup.
template <typename T, base::RawPtrTraits Traits>
struct less<raw_ptr<T, Traits>> {
  using Impl = typename raw_ptr<T, Traits>::Impl;
  using is_transparent = void;

  bool operator()(const raw_ptr<T, Traits>& lhs,
                  const raw_ptr<T, Traits>& rhs) const {
    Impl::IncrementLessCountForTest();
    return lhs < rhs;
  }

  bool operator()(T* lhs, const raw_ptr<T, Traits>& rhs) const {
    Impl::IncrementLessCountForTest();
    return lhs < rhs;
  }

  bool operator()(const raw_ptr<T, Traits>& lhs, T* rhs) const {
    Impl::IncrementLessCountForTest();
    return lhs < rhs;
  }
};

template <typename T, base::RawPtrTraits Traits>
struct hash<raw_ptr<T, Traits>> {
  typedef raw_ptr<T, Traits> argument_type;
  typedef std::size_t result_type;
  result_type operator()(argument_type const& ptr) const {
    return hash<T*>()(ptr.get());
  }
};

// Define for cases where raw_ptr<T> holds a pointer to an array of type T.
// This is consistent with definition of std::iterator_traits<T*>.
// Algorithms like std::binary_search need that.
template <typename T, base::RawPtrTraits Traits>
struct iterator_traits<raw_ptr<T, Traits>> {
  using difference_type = ptrdiff_t;
  using value_type = std::remove_cv_t<T>;
  using pointer = T*;
  using reference = T&;
  using iterator_category = std::random_access_iterator_tag;
};

// Specialize std::pointer_traits. The latter is required to obtain the
// underlying raw pointer in the std::to_address(pointer) overload.
// Implementing the pointer_traits is the standard blessed way to customize
// `std::to_address(pointer)` in C++20 [3].
//
// [1] https://wg21.link/pointer.traits.optmem
template <typename T, ::base::RawPtrTraits Traits>
struct pointer_traits<::raw_ptr<T, Traits>> {
  using pointer = ::raw_ptr<T, Traits>;
  using element_type = T;
  using difference_type = ptrdiff_t;

  template <typename U>
  using rebind = ::raw_ptr<U, Traits>;

  static constexpr pointer pointer_to(element_type& r) noexcept {
    return pointer(&r);
  }

  static constexpr element_type* to_address(pointer p) noexcept {
    return p.get();
  }
};

#if PA_BUILDFLAG(ASSERT_CPP_20)
// Mark `raw_ptr<T>` and `T*` as having a common reference type (the type to
// which both can be converted or bound) of `T*`. This makes them satisfy
// `std::equality_comparable`, which allows usage like:
// ```
//   std::vector<raw_ptr<T>> v;
//   T* e;
//   auto it = std::ranges::find(v, e);
// ```
// Without this, the `find()` call above would fail to compile with a cryptic
// error about being unable to invoke `std::ranges::equal_to()`.
template <typename T,
          base::RawPtrTraits Traits,
          template <typename> typename TQ,
          template <typename> typename UQ>
struct basic_common_reference<raw_ptr<T, Traits>, T*, TQ, UQ> {
  using type = T*;
};

template <typename T,
          base::RawPtrTraits Traits,
          template <typename> typename TQ,
          template <typename> typename UQ>
struct basic_common_reference<T*, raw_ptr<T, Traits>, TQ, UQ> {
  using type = T*;
};
#endif  // PA_BUILDFLAG(ASSERT_CPP_20)

}  // namespace std

#endif  // PARTITION_ALLOC_POINTERS_RAW_PTR_H_
