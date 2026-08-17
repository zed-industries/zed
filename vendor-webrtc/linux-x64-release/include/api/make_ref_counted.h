/*
 *  Copyright 2022 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_MAKE_REF_COUNTED_H_
#define API_MAKE_REF_COUNTED_H_

#include <type_traits>
#include <utility>

#include "absl/base/nullability.h"
#include "api/ref_count.h"
#include "api/scoped_refptr.h"
#include "rtc_base/ref_counted_object.h"

namespace webrtc {

namespace webrtc_make_ref_counted_internal {
// Determines if the given class has AddRef and Release methods.
template <typename T>
class HasAddRefAndRelease {
 private:
  template <typename C,
            decltype(std::declval<C>().AddRef())* = nullptr,
            decltype(std::declval<C>().Release())* = nullptr>
  static int Test(int);
  template <typename>
  static char Test(...);

 public:
  static constexpr bool value = std::is_same_v<decltype(Test<T>(0)), int>;
};
}  // namespace webrtc_make_ref_counted_internal

// General utilities for constructing a reference counted class and the
// appropriate reference count implementation for that class.
//
// These utilities select either the `RefCountedObject` implementation or
// `FinalRefCountedObject` depending on whether the to-be-shared class is
// derived from the RefCountInterface interface or not (respectively).

// `make_ref_counted`:
//
// Use this when you want to construct a reference counted object of type T and
// get a `scoped_refptr<>` back. Example:
//
//   auto p = make_ref_counted<Foo>("bar", 123);
//
// For a class that inherits from RefCountInterface, this is equivalent to:
//
//   auto p = scoped_refptr<Foo>(new RefCountedObject<Foo>("bar", 123));
//
// If the class does not inherit from RefCountInterface, but does have
// AddRef/Release methods (so a T* is convertible to webrtc::scoped_refptr),
// this is equivalent to just
//
//   auto p = scoped_refptr<Foo>(new Foo("bar", 123));
//
// Otherwise, the example is equivalent to:
//
//   auto p = scoped_refptr<FinalRefCountedObject<Foo>>(
//       new FinalRefCountedObject<Foo>("bar", 123));
//
// In these cases, `make_ref_counted` reduces the amount of boilerplate code but
// also helps with the most commonly intended usage of RefCountedObject whereby
// methods for reference counting, are virtual and designed to satisfy the need
// of an interface. When such a need does not exist, it is more efficient to use
// the `FinalRefCountedObject` template, which does not add the vtable overhead.
//
// Note that in some cases, using RefCountedObject directly may still be what's
// needed.

// `make_ref_counted` for abstract classes that are convertible to
// RefCountInterface. The is_abstract requirement rejects classes that inherit
// both RefCountInterface and RefCounted object, which is a a discouraged
// pattern, and would result in double inheritance of RefCountedObject if this
// template was applied.
template <
    typename T,
    typename... Args,
    typename std::enable_if<std::is_convertible_v<T*, RefCountInterface*> &&
                                std::is_abstract_v<T>,
                            T>::type* = nullptr>
absl_nonnull scoped_refptr<T> make_ref_counted(Args&&... args) {
  return scoped_refptr<T>(new RefCountedObject<T>(std::forward<Args>(args)...));
}

// `make_ref_counted` for complete classes that are not convertible to
// RefCountInterface and already carry a ref count.
template <
    typename T,
    typename... Args,
    typename std::enable_if<
        !std::is_convertible_v<T*, RefCountInterface*> &&
            webrtc_make_ref_counted_internal::HasAddRefAndRelease<T>::value,
        T>::type* = nullptr>
absl_nonnull scoped_refptr<T> make_ref_counted(Args&&... args) {
  return scoped_refptr<T>(new T(std::forward<Args>(args)...));
}

// `make_ref_counted` for complete classes that are not convertible to
// RefCountInterface and have no ref count of their own.
template <
    typename T,
    typename... Args,
    typename std::enable_if<
        !std::is_convertible_v<T*, RefCountInterface*> &&
            !webrtc_make_ref_counted_internal::HasAddRefAndRelease<T>::value,

        T>::type* = nullptr>
absl_nonnull scoped_refptr<FinalRefCountedObject<T>> make_ref_counted(
    Args&&... args) {
  return scoped_refptr<FinalRefCountedObject<T>>(
      new FinalRefCountedObject<T>(std::forward<Args>(args)...));
}

}  // namespace webrtc

#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
// Backwards compatibe alias.
// TODO: bugs.webrtc.org/42225969 - deprecate and remove.
using ::webrtc::make_ref_counted;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // API_MAKE_REF_COUNTED_H_
