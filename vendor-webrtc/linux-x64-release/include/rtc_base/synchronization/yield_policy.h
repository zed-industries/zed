/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_BASE_SYNCHRONIZATION_YIELD_POLICY_H_
#define RTC_BASE_SYNCHRONIZATION_YIELD_POLICY_H_

namespace webrtc {
class YieldInterface {
 public:
  virtual ~YieldInterface() = default;
  virtual void YieldExecution() = 0;
};

// Sets the current thread-local yield policy while it's in scope and reverts
// to the previous policy when it leaves the scope.
class ScopedYieldPolicy final {
 public:
  explicit ScopedYieldPolicy(YieldInterface* policy);
  ScopedYieldPolicy(const ScopedYieldPolicy&) = delete;
  ScopedYieldPolicy& operator=(const ScopedYieldPolicy&) = delete;
  ~ScopedYieldPolicy();
  // Will yield as specified by the currently active thread-local yield policy
  // (which by default is a no-op).
  static void YieldExecution();

 private:
  YieldInterface* const previous_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::ScopedYieldPolicy;
using ::webrtc::YieldInterface;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_SYNCHRONIZATION_YIELD_POLICY_H_
