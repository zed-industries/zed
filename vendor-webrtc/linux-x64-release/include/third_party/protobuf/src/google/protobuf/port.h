// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// A common header that is included across all protobuf headers.  We do our best
// to avoid #defining any macros here; instead we generally put macros in
// port_def.inc and port_undef.inc so they are not visible from outside of
// protobuf.

#ifndef GOOGLE_PROTOBUF_PORT_H__
#define GOOGLE_PROTOBUF_PORT_H__

#include <atomic>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <new>
#include <string>
#include <type_traits>
#include <typeinfo>


#include "absl/base/config.h"
#include "absl/base/prefetch.h"
#include "absl/meta/type_traits.h"
#include "absl/strings/string_view.h"
#include "absl/types/optional.h"

#if defined(ABSL_HAVE_ADDRESS_SANITIZER)
#include <sanitizer/asan_interface.h>
#endif

// must be last
#include "google/protobuf/port_def.inc"


namespace google {
namespace protobuf {

class MessageLite;

namespace internal {

struct MessageTraitsImpl;

template <typename T>
PROTOBUF_ALWAYS_INLINE void StrongPointer(T* var) {
#if defined(__GNUC__)
  asm("" : : "r"(var));
#else
  auto volatile unused = var;
  (void)&unused;  // Use address to avoid an extra load of "unused".
#endif
}

#if defined(__x86_64__) && defined(__linux__) && !defined(__APPLE__) && \
    !defined(__ANDROID__) && defined(__clang__) && __clang_major__ >= 19
// Optimized implementation for clang where we can generate a relocation without
// adding runtime instructions.
template <typename T, T ptr>
PROTOBUF_ALWAYS_INLINE void StrongPointer() {
  // This injects a relocation in the code path without having to run code, but
  // we can only do it with a newer clang.
  asm(".reloc ., BFD_RELOC_NONE, %p0" ::"Ws"(ptr));
}

template <typename T, typename TraitsImpl = MessageTraitsImpl>
PROTOBUF_ALWAYS_INLINE void StrongReferenceToType() {
  static constexpr auto ptr =
      decltype(TraitsImpl::template value<T>)::StrongPointer();
  // This is identical to the implementation of StrongPointer() above, but it
  // has to be explicitly inlined here or else Clang 19 will raise an error in
  // some configurations.
  asm(".reloc ., BFD_RELOC_NONE, %p0" ::"Ws"(ptr));
}
#else   // .reloc
// Portable fallback. It usually generates a single LEA instruction or
// equivalent.
template <typename T, T ptr>
PROTOBUF_ALWAYS_INLINE void StrongPointer() {
  StrongPointer(ptr);
}

template <typename T, typename TraitsImpl = MessageTraitsImpl>
PROTOBUF_ALWAYS_INLINE void StrongReferenceToType() {
  return StrongPointer(
      decltype(TraitsImpl::template value<T>)::StrongPointer());
}
#endif  // .reloc


// See comments on `AllocateAtLeast` for information on size returning new.
struct SizedPtr {
  void* p;
  size_t n;
};

// Debug hook allowing setting up test scenarios for AllocateAtLeast usage.
using AllocateAtLeastHookFn = SizedPtr (*)(size_t, void*);

// `AllocAtLeastHook` API
constexpr bool HaveAllocateAtLeastHook();
void SetAllocateAtLeastHook(AllocateAtLeastHookFn fn, void* context = nullptr);

#if !defined(NDEBUG) && defined(ABSL_HAVE_THREAD_LOCAL) && \
    defined(__cpp_inline_variables)

// Hook data for current thread. These vars must not be accessed directly, use
// the 'HaveAllocateAtLeastHook()` and `SetAllocateAtLeastHook()` API instead.
inline thread_local AllocateAtLeastHookFn allocate_at_least_hook = nullptr;
inline thread_local void* allocate_at_least_hook_context = nullptr;

constexpr bool HaveAllocateAtLeastHook() { return true; }
inline void SetAllocateAtLeastHook(AllocateAtLeastHookFn fn, void* context) {
  allocate_at_least_hook = fn;
  allocate_at_least_hook_context = context;
}

#else  // !NDEBUG && ABSL_HAVE_THREAD_LOCAL && __cpp_inline_variables

constexpr bool HaveAllocateAtLeastHook() { return false; }
inline void SetAllocateAtLeastHook(AllocateAtLeastHookFn fn, void* context) {}

#endif  // !NDEBUG && ABSL_HAVE_THREAD_LOCAL && __cpp_inline_variables

// Allocates at least `size` bytes. This function follows the c++ language
// proposal from D0901R10 (http://wg21.link/D0901R10) and will be implemented
// in terms of the new operator new semantics when available. The allocated
// memory should be released by a call to `SizedDelete` or `::operator delete`.
inline SizedPtr AllocateAtLeast(size_t size) {
#if !defined(NDEBUG) && defined(ABSL_HAVE_THREAD_LOCAL) && \
    defined(__cpp_inline_variables)
  if (allocate_at_least_hook != nullptr) {
    return allocate_at_least_hook(size, allocate_at_least_hook_context);
  }
#endif  // !NDEBUG && ABSL_HAVE_THREAD_LOCAL && __cpp_inline_variables
  return {::operator new(size), size};
}

inline void SizedDelete(void* p, size_t size) {
#if defined(__cpp_sized_deallocation)
  ::operator delete(p, size);
#else
  // Avoid -Wunused-parameter
  (void)size;
  ::operator delete(p);
#endif
}
inline void SizedArrayDelete(void* p, size_t size) {
#if defined(__cpp_sized_deallocation)
  ::operator delete[](p, size);
#else
  // Avoid -Wunused-parameter
  (void)size;
  ::operator delete[](p);
#endif
}

// Tag type used to invoke the constinit constructor overload of classes
// such as ArenaStringPtr and MapFieldBase. Such constructors are internal
// implementation details of the library.
struct ConstantInitialized {
  explicit ConstantInitialized() = default;
};

// Tag type used to invoke the arena constructor overload of classes such
// as ExtensionSet and MapFieldLite in aggregate initialization. These
// classes typically don't have move/copy constructors, which rules out
// explicit initialization in pre-C++17.
struct ArenaInitialized {
  explicit ArenaInitialized() = default;
};

template <typename To, typename From>
void AssertDownCast(From* from) {
  static_assert(std::is_base_of<From, To>::value, "illegal DownCast");

  // Check that this function is not used to downcast message types.
  // For those we should use {Down,Dynamic}CastTo{Message,Generated}.
  static_assert(!std::is_base_of_v<MessageLite, To>);

#if PROTOBUF_RTTI
  // RTTI: debug mode only!
  assert(from == nullptr || dynamic_cast<To*>(from) != nullptr);
#endif
}

template <typename To, typename From>
inline To DownCast(From* f) {
  AssertDownCast<std::remove_pointer_t<To>>(f);
  return static_cast<To>(f);
}

template <typename ToRef, typename From>
inline ToRef DownCast(From& f) {
  AssertDownCast<std::remove_reference_t<ToRef>>(&f);
  return static_cast<ToRef>(f);
}

// Looks up the name of `T` via RTTI, if RTTI is available.
template <typename T>
inline absl::optional<absl::string_view> RttiTypeName() {
#if PROTOBUF_RTTI
  return typeid(T).name();
#else
  return absl::nullopt;
#endif
}

// Helpers for identifying our supported types.
template <typename T>
struct is_supported_integral_type
    : absl::disjunction<std::is_same<T, int32_t>, std::is_same<T, uint32_t>,
                        std::is_same<T, int64_t>, std::is_same<T, uint64_t>,
                        std::is_same<T, bool>> {};

template <typename T>
struct is_supported_floating_point_type
    : absl::disjunction<std::is_same<T, float>, std::is_same<T, double>> {};

template <typename T>
struct is_supported_string_type
    : absl::disjunction<std::is_same<T, std::string>> {};

template <typename T>
struct is_supported_scalar_type
    : absl::disjunction<is_supported_integral_type<T>,
                        is_supported_floating_point_type<T>,
                        is_supported_string_type<T>> {};

template <typename T>
struct is_supported_message_type
    : absl::disjunction<std::is_base_of<MessageLite, T>> {
  static constexpr auto force_complete_type = sizeof(T);
};

// To prevent sharing cache lines between threads
#ifdef __cpp_aligned_new
enum { kCacheAlignment = 64 };
#else
enum { kCacheAlignment = alignof(max_align_t) };  // do the best we can
#endif

// The maximum byte alignment we support.
enum { kMaxMessageAlignment = 8 };

// Returns true if debug hardening for clearing oneof message on arenas is
// enabled.
inline constexpr bool DebugHardenClearOneofMessageOnArena() {
#ifdef NDEBUG
  return false;
#else
  return true;
#endif
}

constexpr bool HasAnySanitizer() {
#if defined(ABSL_HAVE_ADDRESS_SANITIZER) || \
    defined(ABSL_HAVE_MEMORY_SANITIZER) || defined(ABSL_HAVE_THREAD_SANITIZER)
  return true;
#else
  return false;
#endif
}

constexpr bool PerformDebugChecks() {
  if (HasAnySanitizer()) return true;
#if defined(NDEBUG)
  return false;
#else
  return true;
#endif
}

// Force copy the default string to a string field so that non-optimized builds
// have harder-to-rely-on address stability.
constexpr bool DebugHardenForceCopyDefaultString() {
  return false;
}

constexpr bool DebugHardenForceCopyInRelease() {
  return false;
}

constexpr bool DebugHardenForceCopyInSwap() {
  return false;
}

constexpr bool DebugHardenForceCopyInMove() {
  return false;
}

constexpr bool DebugHardenForceAllocationOnConstruction() {
  return false;
}

constexpr bool DebugHardenFuzzMessageSpaceUsedLong() {
  return false;
}

// Reads n bytes from p, if PerformDebugChecks() is true. This allows ASAN to
// detect if a range of memory is not valid when we expect it to be. The
// volatile keyword is necessary here to prevent the compiler from optimizing
// away the memory reads below.
inline void AssertBytesAreReadable(const volatile char* p, int n) {
  if (PerformDebugChecks()) {
    for (int i = 0; i < n; ++i) {
      p[i];
    }
  }
}

// Returns true if pointers are 8B aligned, leaving least significant 3 bits
// available.
inline constexpr bool PtrIsAtLeast8BAligned() { return alignof(void*) >= 8; }

inline constexpr bool IsLazyParsingSupported() {
  // We need 3 bits for pointer tagging in lazy parsing.
  return PtrIsAtLeast8BAligned();
}

// Prefetch 5 64-byte cache line starting from 7 cache-lines ahead.
// Constants are somewhat arbitrary and pretty aggressive, but were
// chosen to give a better benchmark results. E.g. this is ~20%
// faster, single cache line prefetch is ~12% faster, increasing
// decreasing distance makes results 2-4% worse. Important note,
// prefetch doesn't require a valid address, so it is ok to prefetch
// past the end of message/valid memory, however we are doing this
// inside inline asm block, since computing the invalid pointer
// is a potential UB. Only insert prefetch once per function,
PROTOBUF_ALWAYS_INLINE void Prefetch5LinesFrom7Lines(const void* ptr) {
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 448);
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 512);
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 576);
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 640);
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 704);
}

// Prefetch 5 64-byte cache lines starting from 1 cache-line ahead.
PROTOBUF_ALWAYS_INLINE void Prefetch5LinesFrom1Line(const void* ptr) {
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 64);
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 128);
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 192);
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 256);
  PROTOBUF_PREFETCH_WITH_OFFSET(ptr, 320);
}

#if defined(NDEBUG) && ABSL_HAVE_BUILTIN(__builtin_unreachable)
[[noreturn]] ABSL_ATTRIBUTE_COLD PROTOBUF_ALWAYS_INLINE void Unreachable() {
  __builtin_unreachable();
}
#elif ABSL_HAVE_BUILTIN(__builtin_FILE) && ABSL_HAVE_BUILTIN(__builtin_LINE)
[[noreturn]] ABSL_ATTRIBUTE_COLD inline void Unreachable(
    const char* file = __builtin_FILE(), int line = __builtin_LINE()) {
  protobuf_assumption_failed("Unreachable", file, line);
}
#else
[[noreturn]] ABSL_ATTRIBUTE_COLD inline void Unreachable() {
  protobuf_assumption_failed("Unreachable", "", 0);
}
#endif

constexpr bool HasMemoryPoisoning() {
#if defined(ABSL_HAVE_ADDRESS_SANITIZER)
  return true;
#else
  return false;
#endif
}

// Poison memory region when supported by sanitizer config.
inline void PoisonMemoryRegion(const void* p, size_t n) {
#if defined(ABSL_HAVE_ADDRESS_SANITIZER)
  ASAN_POISON_MEMORY_REGION(p, n);
#else
  // Nothing
#endif
}

inline void UnpoisonMemoryRegion(const void* p, size_t n) {
#if defined(ABSL_HAVE_ADDRESS_SANITIZER)
  ASAN_UNPOISON_MEMORY_REGION(p, n);
#else
  // Nothing
#endif
}

inline bool IsMemoryPoisoned(const void* p) {
#if defined(ABSL_HAVE_ADDRESS_SANITIZER)
  return __asan_address_is_poisoned(p);
#else
  return false;
#endif
}

#if defined(ABSL_HAVE_THREAD_SANITIZER)
// TODO: it would be preferable to use __tsan_external_read/
// __tsan_external_write, but they can cause dlopen issues.
template <typename T>
PROTOBUF_ALWAYS_INLINE void TSanRead(const T* impl) {
  char protobuf_tsan_dummy = impl->_tsan_detect_race;
  asm volatile("" : "+r"(protobuf_tsan_dummy));
}

// We currently use a dedicated member for TSan checking so the value of this
// member is not important. We can unconditionally write to it without affecting
// correctness of the rest of the class.
template <typename T>
PROTOBUF_ALWAYS_INLINE void TSanWrite(T* impl) {
  impl->_tsan_detect_race = 0;
}
#else
PROTOBUF_ALWAYS_INLINE void TSanRead(const void*) {}
PROTOBUF_ALWAYS_INLINE void TSanWrite(const void*) {}
#endif

// This trampoline allows calling from codegen without needing a #include to
// absl. It simplifies IWYU and deps.
inline void PrefetchToLocalCache(const void* ptr) {
  absl::PrefetchToLocalCache(ptr);
}

template <typename T>
constexpr T* Launder(T* p) {
#if defined(__cpp_lib_launder) && __cpp_lib_launder >= 201606L
  return std::launder(p);
#elif ABSL_HAVE_BUILTIN(__builtin_launder)
  return __builtin_launder(p);
#else
  return p;
#endif
}

#if defined(PROTOBUF_CUSTOM_VTABLE)
constexpr bool EnableCustomNew() { return true; }
template <typename T>
constexpr bool EnableCustomNewFor() {
  return true;
}
#elif ABSL_HAVE_BUILTIN(__is_bitwise_cloneable)
constexpr bool EnableCustomNew() { return true; }
template <typename T>
constexpr bool EnableCustomNewFor() {
  return __is_bitwise_cloneable(T);
}
#else
constexpr bool EnableCustomNew() { return false; }
template <typename T>
constexpr bool EnableCustomNewFor() {
  return false;
}
#endif

constexpr bool IsOss() { return true; }

// Counter library for debugging internal protobuf logic.
// It allows instrumenting code that has different options (eg fast vs slow
// path) to get visibility into how much we are hitting each path.
// When compiled with -DPROTOBUF_INTERNAL_ENABLE_DEBUG_COUNTERS, the counters
// register an atexit handler to dump the table. Otherwise, they are a noop and
// have not runtime cost.
//
// Usage:
//
// if (do_fast) {
//   PROTOBUF_DEBUG_COUNTER("Foo.Fast").Inc();
//   ...
// } else {
//   PROTOBUF_DEBUG_COUNTER("Foo.Slow").Inc();
//   ...
// }
class PROTOBUF_EXPORT RealDebugCounter {
 public:
  explicit RealDebugCounter(absl::string_view name) { Register(name); }
  // Lossy increment.
  void Inc() { counter_.store(value() + 1, std::memory_order_relaxed); }
  size_t value() const { return counter_.load(std::memory_order_relaxed); }

 private:
  void Register(absl::string_view name);
  std::atomic<size_t> counter_{};
};

// When the feature is not enabled, the type is a noop.
class NoopDebugCounter {
 public:
  explicit constexpr NoopDebugCounter() = default;
  constexpr void Inc() {}
};

// Default empty string object. Don't use this directly. Instead, call
// GetEmptyString() to get the reference. This empty string is aligned with a
// minimum alignment of 8 bytes to match the requirement of ArenaStringPtr.

// Take advantage of C++20 constexpr support in std::string.
class alignas(8) GlobalEmptyStringConstexpr {
 public:
  const std::string& get() const { return value_; }
  // Nothing to init, or destroy.
  std::string* Init() const { return nullptr; }

  template <typename T = std::string, bool = (T(), true)>
  static constexpr std::true_type HasConstexprDefaultConstructor(int) {
    return {};
  }
  static constexpr std::false_type HasConstexprDefaultConstructor(char) {
    return {};
  }

 private:
  std::string value_;
};

class alignas(8) GlobalEmptyStringDynamicInit {
 public:
  const std::string& get() const {
    return *reinterpret_cast<const std::string*>(internal::Launder(buffer_));
  }
  std::string* Init() {
    return ::new (static_cast<void*>(buffer_)) std::string();
  }

 private:
  alignas(std::string) char buffer_[sizeof(std::string)];
};

using GlobalEmptyString = std::conditional_t<
    GlobalEmptyStringConstexpr::HasConstexprDefaultConstructor(0),
    const GlobalEmptyStringConstexpr, GlobalEmptyStringDynamicInit>;

PROTOBUF_EXPORT extern GlobalEmptyString fixed_address_empty_string;

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_PORT_H__
