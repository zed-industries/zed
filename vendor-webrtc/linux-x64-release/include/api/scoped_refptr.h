/*
 *  Copyright 2011 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Originally these classes are from Chromium.
// http://src.chromium.org/viewvc/chrome/trunk/src/base/memory/ref_counted.h?view=markup

//
// A smart pointer class for reference counted objects.  Use this class instead
// of calling AddRef and Release manually on a reference counted object to
// avoid common memory leaks caused by forgetting to Release an object
// reference.  Sample usage:
//
//   class MyFoo : public RefCounted<MyFoo> {
//    ...
//   };
//
//   void some_function() {
//     scoped_refptr<MyFoo> foo = make_ref_counted<MyFoo>();
//     foo->Method(param);
//     // `foo` is released when this function returns
//   }
//
//   void some_other_function() {
//     scoped_refptr<MyFoo> foo = make_ref_counted<MyFoo>();
//     ...
//     foo = nullptr;  // explicitly releases `foo`
//     ...
//     if (foo)
//       foo->Method(param);
//   }
//
// The above examples show how scoped_refptr<T> acts like a pointer to T.
// Given two scoped_refptr<T> classes, it is also possible to exchange
// references between the two objects, like so:
//
//   {
//     scoped_refptr<MyFoo> a = make_ref_counted<MyFoo>();
//     scoped_refptr<MyFoo> b;
//
//     b.swap(a);
//     // now, `b` references the MyFoo object, and `a` references null.
//   }
//
// To make both `a` and `b` in the above example reference the same MyFoo
// object, simply use the assignment operator:
//
//   {
//     scoped_refptr<MyFoo> a = make_ref_counted<MyFoo>();
//     scoped_refptr<MyFoo> b;
//
//     b = a;
//     // now, `a` and `b` each own a reference to the same MyFoo object.
//   }
//

#ifndef API_SCOPED_REFPTR_H_
#define API_SCOPED_REFPTR_H_

#include <cstddef>
#include <utility>

#include "absl/base/nullability.h"

namespace webrtc {

template <class T>
class ABSL_NULLABILITY_COMPATIBLE scoped_refptr {
 public:
  using element_type = T;

  scoped_refptr() : ptr_(nullptr) {}
  scoped_refptr(std::nullptr_t) : ptr_(nullptr) {}  // NOLINT(runtime/explicit)

  explicit scoped_refptr(T* absl_nullable p) : ptr_(p) {
    if (ptr_)
      ptr_->AddRef();
  }

  scoped_refptr(const scoped_refptr<T>& r) : ptr_(r.ptr_) {
    if (ptr_)
      ptr_->AddRef();
  }

  template <typename U>
  scoped_refptr(const scoped_refptr<U>& r) : ptr_(r.get()) {
    if (ptr_)
      ptr_->AddRef();
  }

  // Move constructors.
  scoped_refptr(scoped_refptr<T>&& r) noexcept : ptr_(r.release()) {}

  template <typename U>
  scoped_refptr(scoped_refptr<U>&& r) noexcept : ptr_(r.release()) {}

  ~scoped_refptr() {
    if (ptr_)
      ptr_->Release();
  }

  T* get() const { return ptr_; }
  explicit operator bool() const { return ptr_ != nullptr; }
  T& operator*() const { return *ptr_; }
  T* operator->() const { return ptr_; }

  // Returns the (possibly null) raw pointer, and makes the scoped_refptr hold a
  // null pointer, all without touching the reference count of the underlying
  // pointed-to object. The object is still reference counted, and the caller of
  // release() is now the proud owner of one reference, so it is responsible for
  // calling Release() once on the object when no longer using it.
  T* release() {
    T* retVal = ptr_;
    ptr_ = nullptr;
    return retVal;
  }

  scoped_refptr<T>& operator=(T* absl_nullable p) {
    // AddRef first so that self assignment should work
    if (p)
      p->AddRef();
    if (ptr_)
      ptr_->Release();
    ptr_ = p;
    return *this;
  }

  scoped_refptr<T>& operator=(const scoped_refptr<T>& r) {
    return *this = r.ptr_;
  }

  template <typename U>
  scoped_refptr<T>& operator=(const scoped_refptr<U>& r) {
    return *this = r.get();
  }

  scoped_refptr<T>& operator=(scoped_refptr<T>&& r) noexcept {
    scoped_refptr<T>(std::move(r)).swap(*this);
    return *this;
  }

  template <typename U>
  scoped_refptr<T>& operator=(scoped_refptr<U>&& r) noexcept {
    scoped_refptr<T>(std::move(r)).swap(*this);
    return *this;
  }

  void swap(T** absl_nonnull pp) noexcept {
    T* p = ptr_;
    ptr_ = *pp;
    *pp = p;
  }

  void swap(scoped_refptr<T>& r) noexcept { swap(&r.ptr_); }

 protected:
  T* ptr_;
};

template <typename T, typename U>
bool operator==(const scoped_refptr<T>& a, const scoped_refptr<U>& b) {
  return a.get() == b.get();
}
template <typename T, typename U>
bool operator!=(const scoped_refptr<T>& a, const scoped_refptr<U>& b) {
  return !(a == b);
}

template <typename T>
bool operator==(const scoped_refptr<T>& a, std::nullptr_t) {
  return a.get() == nullptr;
}

template <typename T>
bool operator!=(const scoped_refptr<T>& a, std::nullptr_t) {
  return !(a == nullptr);
}

template <typename T>
bool operator==(std::nullptr_t, const scoped_refptr<T>& a) {
  return a.get() == nullptr;
}

template <typename T>
bool operator!=(std::nullptr_t, const scoped_refptr<T>& a) {
  return !(a == nullptr);
}

// Comparison with raw pointer.
template <typename T, typename U>
bool operator==(const scoped_refptr<T>& a, const U* b) {
  return a.get() == b;
}
template <typename T, typename U>
bool operator!=(const scoped_refptr<T>& a, const U* b) {
  return !(a == b);
}

template <typename T, typename U>
bool operator==(const T* a, const scoped_refptr<U>& b) {
  return a == b.get();
}
template <typename T, typename U>
bool operator!=(const T* a, const scoped_refptr<U>& b) {
  return !(a == b);
}

// Ordered comparison, needed for use as a std::map key.
template <typename T, typename U>
bool operator<(const scoped_refptr<T>& a, const scoped_refptr<U>& b) {
  return a.get() < b.get();
}

}  // namespace webrtc

#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
// Backwards compatible alias.
// TODO: bugs.webrtc.org/42225969 - Deprecate and remove.
using ::webrtc::scoped_refptr;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // API_SCOPED_REFPTR_H_
