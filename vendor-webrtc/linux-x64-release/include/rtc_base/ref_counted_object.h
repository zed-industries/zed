/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_BASE_REF_COUNTED_OBJECT_H_
#define RTC_BASE_REF_COUNTED_OBJECT_H_

#include "api/scoped_refptr.h"
#include "rtc_base/ref_count.h"
#include "rtc_base/ref_counter.h"

namespace webrtc {

template <class T>
class RefCountedObject : public T {
 public:
  RefCountedObject() {}

  RefCountedObject(const RefCountedObject&) = delete;
  RefCountedObject& operator=(const RefCountedObject&) = delete;

  template <class P0>
  explicit RefCountedObject(P0&& p0) : T(std::forward<P0>(p0)) {}

  template <class P0, class P1, class... Args>
  RefCountedObject(P0&& p0, P1&& p1, Args&&... args)
      : T(std::forward<P0>(p0),
          std::forward<P1>(p1),
          std::forward<Args>(args)...) {}

  void AddRef() const override { ref_count_.IncRef(); }

  RefCountReleaseStatus Release() const override {
    const auto status = ref_count_.DecRef();
    if (status == RefCountReleaseStatus::kDroppedLastRef) {
      delete this;
    }
    return status;
  }

  // Return whether the reference count is one. If the reference count is used
  // in the conventional way, a reference count of 1 implies that the current
  // thread owns the reference and no other thread shares it. This call
  // performs the test for a reference count of one, and performs the memory
  // barrier needed for the owning thread to act on the object, knowing that it
  // has exclusive access to the object.
  virtual bool HasOneRef() const { return ref_count_.HasOneRef(); }

 protected:
  ~RefCountedObject() override {}

  mutable webrtc::webrtc_impl::RefCounter ref_count_{0};
};

template <class T>
class FinalRefCountedObject final : public T {
 public:
  using T::T;
  // Above using declaration propagates a default move constructor
  // FinalRefCountedObject(FinalRefCountedObject&& other), but we also need
  // move construction from T.
  explicit FinalRefCountedObject(T&& other) : T(std::move(other)) {}
  FinalRefCountedObject(const FinalRefCountedObject&) = delete;
  FinalRefCountedObject& operator=(const FinalRefCountedObject&) = delete;

  void AddRef() const { ref_count_.IncRef(); }
  RefCountReleaseStatus Release() const {
    const auto status = ref_count_.DecRef();
    if (status == RefCountReleaseStatus::kDroppedLastRef) {
      delete this;
    }
    return status;
  }
  bool HasOneRef() const { return ref_count_.HasOneRef(); }

 private:
  ~FinalRefCountedObject() = default;

  mutable webrtc::webrtc_impl::RefCounter ref_count_{0};
};

}  // namespace webrtc

// Backwards compatibe aliases.
// TODO: https://issues.webrtc.org/42225969 - deprecate and remove.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::FinalRefCountedObject;
using ::webrtc::RefCountedObject;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_REF_COUNTED_OBJECT_H_
