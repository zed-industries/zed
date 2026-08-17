/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_REF_COUNTED_BASE_H_
#define API_REF_COUNTED_BASE_H_

#include <type_traits>

#include "api/ref_count.h"
#include "rtc_base/ref_counter.h"

namespace webrtc {

class RefCountedBase {
 public:
  RefCountedBase() = default;

  RefCountedBase(const RefCountedBase&) = delete;
  RefCountedBase& operator=(const RefCountedBase&) = delete;

  void AddRef() const { ref_count_.IncRef(); }
  RefCountReleaseStatus Release() const {
    const auto status = ref_count_.DecRef();
    if (status == RefCountReleaseStatus::kDroppedLastRef) {
      delete this;
    }
    return status;
  }

 protected:
  // Provided for internal webrtc subclasses for corner cases where it's
  // necessary to know whether or not a reference is exclusively held.
  bool HasOneRef() const { return ref_count_.HasOneRef(); }

  virtual ~RefCountedBase() = default;

 private:
  mutable webrtc::webrtc_impl::RefCounter ref_count_{0};
};

// Template based version of `RefCountedBase` for simple implementations that do
// not need (or want) destruction via virtual destructor or the overhead of a
// vtable.
//
// To use:
//   struct MyInt : public webrtc::RefCountedNonVirtual<MyInt>  {
//     int foo_ = 0;
//   };
//
//   webrtc::scoped_refptr<MyInt> my_int(new MyInt());
//
// sizeof(MyInt) on a 32 bit system would then be 8, int + refcount and no
// vtable generated.
template <typename T>
class RefCountedNonVirtual {
 public:
  RefCountedNonVirtual() = default;

  RefCountedNonVirtual(const RefCountedNonVirtual&) = delete;
  RefCountedNonVirtual& operator=(const RefCountedNonVirtual&) = delete;

  void AddRef() const { ref_count_.IncRef(); }
  RefCountReleaseStatus Release() const {
    // If you run into this assert, T has virtual methods. There are two
    // options:
    // 1) The class doesn't actually need virtual methods, the type is complete
    //    so the virtual attribute(s) can be removed.
    // 2) The virtual methods are a part of the design of the class. In this
    //    case you can consider using `RefCountedBase` instead or alternatively
    //    use `webrtc::RefCountedObject`.
    static_assert(!std::is_polymorphic<T>::value,
                  "T has virtual methods. RefCountedBase is a better fit.");
    const auto status = ref_count_.DecRef();
    if (status == RefCountReleaseStatus::kDroppedLastRef) {
      delete static_cast<const T*>(this);
    }
    return status;
  }

 protected:
  // Provided for internal webrtc subclasses for corner cases where it's
  // necessary to know whether or not a reference is exclusively held.
  bool HasOneRef() const { return ref_count_.HasOneRef(); }

  ~RefCountedNonVirtual() = default;

 private:
  mutable webrtc::webrtc_impl::RefCounter ref_count_{0};
};

}  // namespace webrtc

// Backwards compatibe aliases.
// TODO: https://issues.webrtc.org/42225969 - deprecate and remove.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using RefCountedBase = webrtc::RefCountedBase;
template <typename T>
using RefCountedNonVirtual = webrtc::RefCountedNonVirtual<T>;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // API_REF_COUNTED_BASE_H_
