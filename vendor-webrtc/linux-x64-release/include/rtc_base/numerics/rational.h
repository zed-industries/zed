/*
 *  Copyright 2024 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_RATIONAL_H_
#define RTC_BASE_NUMERICS_RATIONAL_H_

namespace webrtc {

// This is the worst implementation of a rational...
struct Rational {
  int numerator;
  int denominator;

  bool operator==(const Rational& other) const {
    return numerator == other.numerator && denominator == other.denominator;
  }
};

}  // namespace webrtc

#endif  // RTC_BASE_NUMERICS_RATIONAL_H_
