/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_UTIL_THRESHOLD_CURVE_H_
#define MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_UTIL_THRESHOLD_CURVE_H_

#include "rtc_base/checks.h"

namespace webrtc {

class ThresholdCurve {
 public:
  struct Point {
    constexpr Point(float x, float y) : x(x), y(y) {}
    float x;
    float y;
  };

  // ThresholdCurve defines a curve. The curve is characterized by the two
  // conjunction points: A and B. The curve segments the metric space into
  // three domains - above the curve, on it and below it.
  //
  // y-axis ^  |
  //        | A|
  //        |   \    A: (a.x, a.y)
  //        |    \   B: (b.x, b.y)
  //        |    B\________
  //        |---------------> bandwidth
  //
  // If either a.x == b.x or a.y == b.y, the curve can be defined
  // by a single point. (We merge the two points into one - either the lower or
  // the leftmost one - for easier treatment.)
  //
  // y-axis ^  |
  //        |  |
  //        |  |
  //        |  |
  //        | P|__________
  //        |---------------> bandwidth
  ThresholdCurve(const Point& left, const Point& right)
      : a(GetPoint(left, right, true)),
        b(GetPoint(left, right, false)),
        slope(b.x - a.x == 0.0f ? 0.0f : (b.y - a.y) / (b.x - a.x)),
        offset(a.y - slope * a.x) {
    // TODO(eladalon): We might want to introduce some numerical validations.
  }

  ThresholdCurve(float a_x, float a_y, float b_x, float b_y)
      : ThresholdCurve(Point{a_x, a_y}, Point{b_x, b_y}) {}

  // Checks if a point is strictly below the curve.
  bool IsBelowCurve(const Point& p) const {
    if (p.x < a.x) {
      return true;
    } else if (p.x == a.x) {
      // In principle, we could merge this into the next else, but to avoid
      // numerical errors, we treat it separately.
      return p.y < a.y;
    } else if (a.x < p.x && p.x < b.x) {
      return p.y < offset + slope * p.x;
    } else {  // if (b.x <= p.x)
      return p.y < b.y;
    }
  }

  // Checks if a point is strictly above the curve.
  bool IsAboveCurve(const Point& p) const {
    if (p.x <= a.x) {
      return false;
    } else if (a.x < p.x && p.x < b.x) {
      return p.y > offset + slope * p.x;
    } else {  // if (b.x <= p.x)
      return p.y > b.y;
    }
  }

  bool operator<=(const ThresholdCurve& rhs) const {
    // This curve is <= the rhs curve if no point from this curve is
    // above a corresponding point from the rhs curve.
    return !IsBelowCurve(rhs.a) && !IsBelowCurve(rhs.b) &&
           !rhs.IsAboveCurve(a) && !rhs.IsAboveCurve(b);
  }

 private:
  static const Point& GetPoint(const Point& left,
                               const Point& right,
                               bool is_for_left) {
    RTC_DCHECK_LE(left.x, right.x);
    RTC_DCHECK_GE(left.y, right.y);

    // Same X-value or Y-value triggers merging both points to the
    // lower and/or left of the two points, respectively.
    if (left.x == right.x) {
      return right;
    } else if (left.y == right.y) {
      return left;
    }

    // If unmerged, boolean flag determines which of the points is desired.
    return is_for_left ? left : right;
  }

  const Point a;
  const Point b;
  const float slope;
  const float offset;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_UTIL_THRESHOLD_CURVE_H_
