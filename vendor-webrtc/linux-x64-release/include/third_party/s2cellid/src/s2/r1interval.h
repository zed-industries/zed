// Copyright 2005 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS-IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// Author: ericv@google.com (Eric Veach)

#ifndef S2_R1INTERVAL_H_
#define S2_R1INTERVAL_H_

#include <algorithm>
#include <cmath>
#include <iosfwd>
#include <iostream>

#include "base/check.h"
#include "s2/_fpcontractoff.h"
#include "s2/util/math/vector.h"  // IWYU pragma: export

// An R1Interval represents a closed, bounded interval on the real line.
// It is capable of representing the empty interval (containing no points)
// and zero-length intervals (containing a single point).
//
// This class is intended to be copied by value as desired.  It uses
// the default copy constructor and assignment operator.
class R1Interval {
 public:
  // Constructor.  If lo > hi, the interval is empty.
  R1Interval(double lo, double hi) : bounds_(lo, hi) {}

  // The default constructor creates an empty interval.  (Any interval where
  // lo > hi is considered to be empty.)
  //
  // Note: Don't construct an interval using the default constructor and
  // set_lo()/set_hi(), since this technique doesn't work with S1Interval and
  // is bad programming style anyways.  If you need to set both endpoints, use
  // the constructor above:
  //
  //   lat_bounds_ = R1Interval(lat_lo, lat_hi);
  R1Interval() : bounds_(1, 0) {}

  // Returns an empty interval.
  static inline R1Interval Empty() { return R1Interval(); }

  // Convenience method to construct an interval containing a single point.
  static R1Interval FromPoint(double p) { return R1Interval(p, p); }

  // Convenience method to construct the minimal interval containing
  // the two given points.  This is equivalent to starting with an empty
  // interval and calling AddPoint() twice, but it is more efficient.
  static R1Interval FromPointPair(double p1, double p2) {
    if (p1 <= p2) {
      return R1Interval(p1, p2);
    } else {
      return R1Interval(p2, p1);
    }
  }

  // Accessors methods.
  double lo() const { return bounds_[0]; }
  double hi() const { return bounds_[1]; }

  // Methods to modify one endpoint of an existing R1Interval.  Do not use
  // these methods if you want to replace both endpoints of the interval; use
  // a constructor instead.  For example:
  //
  //   *lat_bounds = R1Interval(lat_lo, lat_hi);
  void set_lo(double p) { bounds_[0] = p; }
  void set_hi(double p) { bounds_[1] = p; }

  // Methods that allow the R1Interval to be accessed as a vector.  (The
  // recommended style is to use lo() and hi() whenever possible, but these
  // methods are useful when the endpoint to be selected is not constant.)
  double operator[](int i) const { return bounds_[i]; }
  double& operator[](int i) { return bounds_[i]; }
  Vector2_d const& bounds() const { return bounds_; }
  Vector2_d* mutable_bounds() { return &bounds_; }

  // Return true if the interval is empty, i.e. it contains no points.
  bool is_empty() const { return lo() > hi(); }

  // Return the center of the interval.  For empty intervals,
  // the result is arbitrary.
  double GetCenter() const { return 0.5 * (lo() + hi()); }

  // Return the length of the interval.  The length of an empty interval
  // is negative.
  double GetLength() const { return hi() - lo(); }

  bool Contains(double p) const { return p >= lo() && p <= hi(); }

  bool InteriorContains(double p) const { return p > lo() && p < hi(); }

  // Return true if this interval contains the interval 'y'.
  bool Contains(R1Interval const& y) const {
    if (y.is_empty())
      return true;
    return y.lo() >= lo() && y.hi() <= hi();
  }

  // Return true if the interior of this interval contains the entire
  // interval 'y' (including its boundary).
  bool InteriorContains(R1Interval const& y) const {
    if (y.is_empty())
      return true;
    return y.lo() > lo() && y.hi() < hi();
  }

  // Return true if this interval intersects the given interval,
  // i.e. if they have any points in common.
  bool Intersects(R1Interval const& y) const {
    if (lo() <= y.lo()) {
      return y.lo() <= hi() && y.lo() <= y.hi();
    } else {
      return lo() <= y.hi() && lo() <= hi();
    }
  }

  // Return true if the interior of this interval intersects
  // any point of the given interval (including its boundary).
  bool InteriorIntersects(R1Interval const& y) const {
    return y.lo() < hi() && lo() < y.hi() && lo() < hi() && y.lo() <= y.hi();
  }

  // Return the Hausdorff distance to the given interval 'y'. For two
  // R1Intervals x and y, this distance is defined as
  //     h(x, y) = max_{p in x} min_{q in y} d(p, q).
  double GetDirectedHausdorffDistance(R1Interval const& y) const {
    if (is_empty())
      return 0.0;
    if (y.is_empty())
      return HUGE_VAL;
    return std::max(0.0, std::max(hi() - y.hi(), y.lo() - lo()));
  }

  // Expand the interval so that it contains the given point "p".
  void AddPoint(double p) {
    if (is_empty()) {
      set_lo(p);
      set_hi(p);
    } else if (p < lo()) {
      set_lo(p);
    }  // NOLINT
    else if (p > hi()) {
      set_hi(p);
    }  // NOLINT
  }

  // Expand the interval so that it contains the given interval "y".
  void AddInterval(R1Interval const& y) {
    if (y.is_empty())
      return;
    if (is_empty()) {
      *this = y;
      return;
    }
    if (y.lo() < lo())
      set_lo(y.lo());
    if (y.hi() > hi())
      set_hi(y.hi());
  }

  // Return the closest point in the interval to the given point "p".
  // The interval must be non-empty.
  double Project(double p) const {
    DCHECK(!is_empty());
    return std::max(lo(), std::min(hi(), p));
  }

  // Return an interval that has been expanded on each side by the given
  // distance "margin".  If "margin" is negative, then shrink the interval on
  // each side by "margin" instead.  The resulting interval may be empty.  Any
  // expansion of an empty interval remains empty.
  R1Interval Expanded(double margin) const {
    if (is_empty())
      return *this;
    return R1Interval(lo() - margin, hi() + margin);
  }

  // Return the smallest interval that contains this interval and the
  // given interval "y".
  R1Interval Union(R1Interval const& y) const {
    if (is_empty())
      return y;
    if (y.is_empty())
      return *this;
    return R1Interval(std::min(lo(), y.lo()), std::max(hi(), y.hi()));
  }

  // Return the intersection of this interval with the given interval.
  // Empty intervals do not need to be special-cased.
  R1Interval Intersection(R1Interval const& y) const {
    return R1Interval(std::max(lo(), y.lo()), std::min(hi(), y.hi()));
  }

  // Return true if two intervals contain the same set of points.
  bool operator==(R1Interval const& y) const {
    return (lo() == y.lo() && hi() == y.hi()) || (is_empty() && y.is_empty());
  }

  // Return true if two intervals do not contain the same set of points.
  bool operator!=(R1Interval const& y) const { return !operator==(y); }

  // Return true if this interval can be transformed into the given interval
  // by moving each endpoint by at most "max_error".  The empty interval is
  // considered to be positioned arbitrarily on the real line, thus any
  // interval with (length <= 2*max_error) matches the empty interval.
  bool ApproxEquals(R1Interval const& y, double max_error = 1e-15) const {
    if (is_empty())
      return y.GetLength() <= 2 * max_error;
    if (y.is_empty())
      return GetLength() <= 2 * max_error;
    return (std::fabs(y.lo() - lo()) <= max_error &&
            std::fabs(y.hi() - hi()) <= max_error);
  }

 private:
  Vector2_d bounds_;
};

inline std::ostream& operator<<(std::ostream& os, R1Interval const& x) {
  return os << "[" << x.lo() << ", " << x.hi() << "]";
}

#endif  // S2_R1INTERVAL_H_
