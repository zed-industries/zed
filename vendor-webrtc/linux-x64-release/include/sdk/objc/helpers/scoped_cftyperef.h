/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 */

#ifndef SDK_OBJC_HELPERS_SCOPED_CFTYPEREF_H_
#define SDK_OBJC_HELPERS_SCOPED_CFTYPEREF_H_

#include <CoreFoundation/CoreFoundation.h>

namespace webrtc {

// RETAIN: ScopedTypeRef should retain the object when it takes
// ownership.
// ASSUME: Assume the object already has already been retained.
// ScopedTypeRef takes over ownership.
enum class RetainPolicy { RETAIN, ASSUME };

namespace internal {
template <typename T>
struct CFTypeRefTraits {
  static T InvalidValue() { return nullptr; }
  static void Release(T ref) { CFRelease(ref); }
  static T Retain(T ref) {
    CFRetain(ref);
    return ref;
  }
};

template <typename T, typename Traits>
class ScopedTypeRef {
 public:
  ScopedTypeRef() : ptr_(Traits::InvalidValue()) {}
  explicit ScopedTypeRef(T ptr) : ptr_(ptr) {}
  ScopedTypeRef(T ptr, RetainPolicy policy) : ScopedTypeRef(ptr) {
    if (ptr_ && policy == RetainPolicy::RETAIN)
      Traits::Retain(ptr_);
  }

  ScopedTypeRef(const ScopedTypeRef<T, Traits>& rhs) : ptr_(rhs.ptr_) {
    if (ptr_)
      ptr_ = Traits::Retain(ptr_);
  }

  ~ScopedTypeRef() {
    if (ptr_) {
      Traits::Release(ptr_);
    }
  }

  T get() const { return ptr_; }
  T operator->() const { return ptr_; }
  explicit operator bool() const { return ptr_; }

  bool operator!() const { return !ptr_; }

  ScopedTypeRef& operator=(const T& rhs) {
    if (ptr_)
      Traits::Release(ptr_);
    ptr_ = rhs;
    return *this;
  }

  ScopedTypeRef& operator=(const ScopedTypeRef<T, Traits>& rhs) {
    reset(rhs.get(), RetainPolicy::RETAIN);
    return *this;
  }

  // This is intended to take ownership of objects that are
  // created by pass-by-pointer initializers.
  T* InitializeInto() {
    RTC_DCHECK(!ptr_);
    return &ptr_;
  }

  void reset(T ptr, RetainPolicy policy = RetainPolicy::ASSUME) {
    if (ptr && policy == RetainPolicy::RETAIN)
      Traits::Retain(ptr);
    if (ptr_)
      Traits::Release(ptr_);
    ptr_ = ptr;
  }

  T release() {
    T temp = ptr_;
    ptr_ = Traits::InvalidValue();
    return temp;
  }

 private:
  T ptr_;
};
}  // namespace internal

template <typename T>
using ScopedCFTypeRef =
    internal::ScopedTypeRef<T, internal::CFTypeRefTraits<T>>;

template <typename T>
static ScopedCFTypeRef<T> AdoptCF(T cftype) {
  return ScopedCFTypeRef<T>(cftype, RetainPolicy::RETAIN);
}

template <typename T>
static ScopedCFTypeRef<T> ScopedCF(T cftype) {
  return ScopedCFTypeRef<T>(cftype);
}

}  // namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::AdoptCF;
using ::webrtc::RetainPolicy;
using ::webrtc::ScopedCF;
using ::webrtc::ScopedCFTypeRef;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // SDK_OBJC_HELPERS_SCOPED_CFTYPEREF_H_
