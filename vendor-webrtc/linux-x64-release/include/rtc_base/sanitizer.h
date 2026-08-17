/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SANITIZER_H_
#define RTC_BASE_SANITIZER_H_

#include <stddef.h>  // For size_t.

#ifdef __cplusplus
#include <type_traits>
#endif

#if defined(__has_feature)
#if __has_feature(address_sanitizer)
#define RTC_HAS_ASAN 1
#endif
#if __has_feature(memory_sanitizer)
#define RTC_HAS_MSAN 1
#endif
#endif
#ifndef RTC_HAS_ASAN
#define RTC_HAS_ASAN 0
#endif
#ifndef RTC_HAS_MSAN
#define RTC_HAS_MSAN 0
#endif

#if RTC_HAS_ASAN
#include <sanitizer/asan_interface.h>
#endif
#if RTC_HAS_MSAN
#include <sanitizer/msan_interface.h>
#endif

#ifdef __has_attribute
#if __has_attribute(no_sanitize)
#define RTC_NO_SANITIZE(what) __attribute__((no_sanitize(what)))
#endif
#endif
#ifndef RTC_NO_SANITIZE
#define RTC_NO_SANITIZE(what)
#endif

// Ask ASan to mark the memory range [ptr, ptr + element_size * num_elements)
// as being unaddressable, so that reads and writes are not allowed. ASan may
// narrow the range to the nearest alignment boundaries.
static inline void rtc_AsanPoison(const volatile void* ptr,
                                  size_t element_size,
                                  size_t num_elements) {
#if RTC_HAS_ASAN
  ASAN_POISON_MEMORY_REGION(ptr, element_size * num_elements);
#else
  // This is to prevent from the compiler raising a warning/error over unused
  // variables. We cannot use clang's annotation (`[[maybe_unused]]`) because
  // this file is also included from c files which doesn't support the
  // annotation till we switch to C23
  (void)ptr;
  (void)element_size;
  (void)num_elements;
#endif
}

// Ask ASan to mark the memory range [ptr, ptr + element_size * num_elements)
// as being addressable, so that reads and writes are allowed. ASan may widen
// the range to the nearest alignment boundaries.
static inline void rtc_AsanUnpoison(const volatile void* ptr,
                                    size_t element_size,
                                    size_t num_elements) {
#if RTC_HAS_ASAN
  ASAN_UNPOISON_MEMORY_REGION(ptr, element_size * num_elements);
#else
  (void)ptr;
  (void)element_size;
  (void)num_elements;
#endif
}

// Ask MSan to mark the memory range [ptr, ptr + element_size * num_elements)
// as being uninitialized.
static inline void rtc_MsanMarkUninitialized(const volatile void* ptr,
                                             size_t element_size,
                                             size_t num_elements) {
#if RTC_HAS_MSAN
  __msan_poison(ptr, element_size * num_elements);
#else
  (void)ptr;
  (void)element_size;
  (void)num_elements;
#endif
}

// Force an MSan check (if any bits in the memory range [ptr, ptr +
// element_size * num_elements) are uninitialized the call will crash with an
// MSan report).
static inline void rtc_MsanCheckInitialized(const volatile void* ptr,
                                            size_t element_size,
                                            size_t num_elements) {
#if RTC_HAS_MSAN
  __msan_check_mem_is_initialized(ptr, element_size * num_elements);
#else
  (void)ptr;
  (void)element_size;
  (void)num_elements;
#endif
}

#ifdef __cplusplus

namespace webrtc {
namespace sanitizer_impl {

template <typename T>
constexpr bool IsTriviallyCopyable() {
  return static_cast<bool>(std::is_trivially_copy_constructible<T>::value &&
                           (std::is_trivially_copy_assignable<T>::value ||
                            !std::is_copy_assignable<T>::value) &&
                           std::is_trivially_destructible<T>::value);
}

}  // namespace sanitizer_impl

template <typename T>
inline void AsanPoison(const T& mem) {
  rtc_AsanPoison(mem.data(), sizeof(mem.data()[0]), mem.size());
}

template <typename T>
inline void AsanUnpoison(const T& mem) {
  rtc_AsanUnpoison(mem.data(), sizeof(mem.data()[0]), mem.size());
}

template <typename T>
inline void MsanMarkUninitialized(const T& mem) {
  rtc_MsanMarkUninitialized(mem.data(), sizeof(mem.data()[0]), mem.size());
}

template <typename T>
inline T MsanUninitialized(T t) {
#if RTC_HAS_MSAN
  // TODO(bugs.webrtc.org/8762): Switch to std::is_trivially_copyable when it
  // becomes available in downstream projects.
  static_assert(sanitizer_impl::IsTriviallyCopyable<T>(), "");
#endif
  rtc_MsanMarkUninitialized(&t, sizeof(T), 1);
  return t;
}

template <typename T>
inline void MsanCheckInitialized(const T& mem) {
  rtc_MsanCheckInitialized(mem.data(), sizeof(mem.data()[0]), mem.size());
}

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::AsanPoison;
using ::webrtc::AsanUnpoison;
using ::webrtc::MsanCheckInitialized;
using ::webrtc::MsanMarkUninitialized;
using ::webrtc::MsanUninitialized;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // __cplusplus

#endif  // RTC_BASE_SANITIZER_H_
