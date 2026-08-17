// Copyright 2012 Google Inc. All Rights Reserved.
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

#ifndef S2_R2RECT_H_
#define S2_R2RECT_H_

#include <iosfwd>

#include "base/check.h"
#include "s2/_fpcontractoff.h"
#include "s2/r1interval.h"
#include "s2/r2.h"

// An R2Rect represents a closed axis-aligned rectangle in the (x,y) plane.
//
// This class is intended to be copied by value as desired.  It uses
// the default copy constructor and assignment operator, however it is
// not a "plain old datatype" (POD) because it has virtual functions.
class R2Rect {
 public:
  // Construct a rectangle from the given lower-left and upper-right points.
  R2Rect(R2Point const& lo, R2Point const& hi);

  // Construct a rectangle from the given intervals in x and y.  The two
  // intervals must either be both empty or both non-empty.
  R2Rect(R1Interval const& x, R1Interval const& y);

  // The default constructor creates an empty R2Rect.
  R2Rect();

  // The canonical empty rectangle.  Use is_empty() to test for empty
  // rectangles, since they have more than one representation.
  static R2Rect Empty();

  // Construct a rectangle from a center point and size in each dimension.
  // Both components of size should be non-negative, i.e. this method cannot
  // be used to create an empty rectangle.
  static R2Rect FromCenterSize(R2Point const& center, R2Point const& size);

  // Convenience method to construct a rectangle containing a single point.
  static R2Rect FromPoint(R2Point const& p);

  // Convenience method to construct the minimal bounding rectangle containing
  // the two given points.  This is equivalent to starting with an empty
  // rectangle and calling AddPoint() twice.  Note that it is different than
  // the R2Rect(lo, hi) constructor, where the first point is always
  // used as the lower-left corner of the resulting rectangle.
  static R2Rect FromPointPair(R2Point const& p1, R2Point const& p2);

  // Accessor methods.
  R1Interval const& x() const { return bounds_[0]; }
  R1Interval const& y() const { return bounds_[1]; }
  R2Point lo() const { return R2Point(x().lo(), y().lo()); }
  R2Point hi() const { return R2Point(x().hi(), y().hi()); }

  // Methods that allow the R2Rect to be accessed as a vector.
  R1Interval const& operator[](int i) const { return bounds_[i]; }
  R1Interval& operator[](int i) { return bounds_[i]; }

  // Return true if the rectangle is valid, which essentially just means
  // that if the bound for either axis is empty then both must be.
  bool is_valid() const;

  // Return true if the rectangle is empty, i.e. it contains no points at all.
  bool is_empty() const;

  // Return the k-th vertex of the rectangle (k = 0,1,2,3) in CCW order.
  // Vertex 0 is in the lower-left corner.  For convenience, the argument is
  // reduced modulo 4 to the range [0..3].
  R2Point GetVertex(int k) const;

  // Return the vertex in direction "i" along the x-axis (0=left, 1=right) and
  // direction "j" along the y-axis (0=down, 1=up).  Equivalently, return the
  // vertex constructed by selecting endpoint "i" of the x-interval (0=lo,
  // 1=hi) and vertex "j" of the y-interval.
  R2Point GetVertex(int i, int j) const;

  // Return the center of the rectangle in (x,y)-space.
  R2Point GetCenter() const;

  // Return the width and height of this rectangle in (x,y)-space.  Empty
  // rectangles have a negative width and height.
  R2Point GetSize() const;

  // Return true if the rectangle contains the given point.  Note that
  // rectangles are closed regions, i.e. they contain their boundary.
  bool Contains(R2Point const& p) const;

  // Return true if and only if the given point is contained in the interior
  // of the region (i.e. the region excluding its boundary).
  bool InteriorContains(R2Point const& p) const;

  // Return true if and only if the rectangle contains the given other
  // rectangle.
  bool Contains(R2Rect const& other) const;

  // Return true if and only if the interior of this rectangle contains all
  // points of the given other rectangle (including its boundary).
  bool InteriorContains(R2Rect const& other) const;

  // Return true if this rectangle and the given other rectangle have any
  // points in common.
  bool Intersects(R2Rect const& other) const;

  // Return true if and only if the interior of this rectangle intersects
  // any point (including the boundary) of the given other rectangle.
  bool InteriorIntersects(R2Rect const& other) const;

  // Expand the rectangle to include the given point.  The rectangle is
  // expanded by the minimum amount possible.
  void AddPoint(R2Point const& p);

  // Expand the rectangle to include the given other rectangle.  This is the
  // same as replacing the rectangle by the union of the two rectangles, but
  // is somewhat more efficient.
  void AddRect(R2Rect const& other);

  // Return the closest point in the rectangle to the given point "p".
  // The rectangle must be non-empty.
  R2Point Project(R2Point const& p) const;

  // Return a rectangle that has been expanded on each side in the x-direction
  // by margin.x(), and on each side in the y-direction by margin.y().  If
  // either margin is empty, then shrink the interval on the corresponding
  // sides instead.  The resulting rectangle may be empty.  Any expansion of
  // an empty rectangle remains empty.
  R2Rect Expanded(R2Point const& margin) const;
  R2Rect Expanded(double margin) const;

  // Return the smallest rectangle containing the union of this rectangle and
  // the given rectangle.
  R2Rect Union(R2Rect const& other) const;

  // Return the smallest rectangle containing the intersection of this
  // rectangle and the given rectangle.
  R2Rect Intersection(R2Rect const& other) const;

  // Return true if two rectangles contains the same set of points.
  bool operator==(R2Rect const& other) const;

  // Return true if the x- and y-intervals of the two rectangles are the same
  // up to the given tolerance (see r1interval.h for details).
  bool ApproxEquals(R2Rect const& other, double max_error = 1e-15) const;

 private:
  R1Interval bounds_[2];
};

inline R2Rect::R2Rect(R2Point const& lo, R2Point const& hi) {
  bounds_[0] = R1Interval(lo.x(), hi.x());
  bounds_[1] = R1Interval(lo.y(), hi.y());
  DCHECK(is_valid());
}

inline R2Rect::R2Rect(R1Interval const& x, R1Interval const& y) {
  bounds_[0] = x;
  bounds_[1] = y;
  DCHECK(is_valid());
}

inline R2Rect::R2Rect() {
  // The default R1Interval constructor creates an empty interval.
  DCHECK(is_valid());
}

inline R2Rect R2Rect::Empty() {
  return R2Rect(R1Interval::Empty(), R1Interval::Empty());
}

inline bool R2Rect::is_valid() const {
  // The x/y ranges must either be both empty or both non-empty.
  return x().is_empty() == y().is_empty();
}

inline bool R2Rect::is_empty() const {
  return x().is_empty();
}

inline R2Rect R2Rect::FromPoint(R2Point const& p) {
  return R2Rect(p, p);
}

inline R2Point R2Rect::GetVertex(int k) const {
  // Twiddle bits to return the points in CCW order (lower left, lower right,
  // upper right, upper left).
  int j = (k >> 1) & 1;
  return GetVertex(j ^ (k & 1), j);
}

inline R2Point R2Rect::GetVertex(int i, int j) const {
  return R2Point(bounds_[0][i], bounds_[1][j]);
}

inline R2Point R2Rect::GetCenter() const {
  return R2Point(x().GetCenter(), y().GetCenter());
}

inline R2Point R2Rect::GetSize() const {
  return R2Point(x().GetLength(), y().GetLength());
}

inline bool R2Rect::Contains(R2Point const& p) const {
  return x().Contains(p.x()) && y().Contains(p.y());
}

inline bool R2Rect::InteriorContains(R2Point const& p) const {
  return x().InteriorContains(p.x()) && y().InteriorContains(p.y());
}

inline R2Rect R2Rect::Expanded(double margin) const {
  return Expanded(R2Point(margin, margin));
}

inline bool R2Rect::operator==(R2Rect const& other) const {
  return x() == other.x() && y() == other.y();
}

std::ostream& operator<<(std::ostream& os, R2Rect const& r);

#endif  // S2_R2RECT_H_
