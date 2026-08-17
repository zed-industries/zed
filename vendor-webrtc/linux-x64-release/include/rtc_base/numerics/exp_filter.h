/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_EXP_FILTER_H_
#define RTC_BASE_NUMERICS_EXP_FILTER_H_

namespace webrtc {

// This class can be used, for example, for smoothing the result of bandwidth
// estimation and packet loss estimation.

class ExpFilter {
 public:
  static const float kValueUndefined;

  explicit ExpFilter(float alpha, float max = kValueUndefined) : max_(max) {
    Reset(alpha);
  }

  // Resets the filter to its initial state, and resets filter factor base to
  // the given value `alpha`.
  void Reset(float alpha);

  // Applies the filter with a given exponent on the provided sample:
  // y(k) = min(alpha_^ exp * y(k-1) + (1 - alpha_^ exp) * sample, max_).
  float Apply(float exp, float sample);

  // Returns current filtered value.
  float filtered() const { return filtered_; }

  // Changes the filter factor base to the given value `alpha`.
  void UpdateBase(float alpha);

 private:
  float alpha_;     // Filter factor base.
  float filtered_;  // Current filter output.
  const float max_;
};
}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::ExpFilter;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_NUMERICS_EXP_FILTER_H_
