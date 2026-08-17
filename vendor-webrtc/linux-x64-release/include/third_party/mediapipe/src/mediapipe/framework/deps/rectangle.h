// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Class for axis-aligned rectangles represented as two corner points
// (min_x, min_y) and (max_x, max_y).  The methods such as Contain, Intersect
// and IsEmpty() assume that the points in region include the 4 boundary edges.
// The default box is initialized so that IsEmpty() is true.  Note that the
// use of corner points supports both right-handed (Cartesian) and left-
// handed (image) coordinate systems.

#ifndef MEDIAPIPE_DEPS_RECTANGLE_H_
#define MEDIAPIPE_DEPS_RECTANGLE_H_

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <iosfwd>
#include <limits>
#include <ostream>

#include "mediapipe/framework/deps/point2.h"

template <typename T>
class Rectangle;

template <typename T>
std::ostream& operator<<(std::ostream&, const Rectangle<T>&);

template <typename T>
class Rectangle {
 public:
  typedef Rectangle<T> Self;

  // Default constructed rectangle which is empty.
  Rectangle() { SetEmpty(); }

  // Creates a rectangle from the minimum point and the dimensions.
  Rectangle(const T& x, const T& y, const T& width, const T& height);

  // Creates a rectangle given two points.  The resulting rectangle will
  // have non-negative width and height.
  Rectangle(const Point2<T>& p0, const Point2<T>& p1);

  // Same as above but using vectors as input.
  Rectangle(const Vector2<T>& p0, const Vector2<T>& p1);

  // Sets min to be very large numbers and max to be very large negative numbers
  // so that points can be used to correctly extend the rectangle.
  void SetEmpty();

  // A rectangle is empty if there are no points inside of it.  A degenerate
  // rectangle where the corners are coincident has zero area but is not empty.
  bool IsEmpty() const { return min_.x() > max_.x() || min_.y() > max_.y(); }

  bool operator==(const Rectangle&) const;
  bool operator!=(const Rectangle&) const;

  // Width and height are both max - min, which may be negative if SetEmpty()
  // was called or the user explicity set the min and max points.
  T Width() const { return max_.x() - min_.x(); }
  T Height() const { return max_.y() - min_.y(); }

  // Computes the area, which is negative if the width xor height is negative.
  // The value is undefined if SetEmpty() is called.
  // Watch out for large integer rectangles because the area may overflow.
  T Area() const { return Width() * Height(); }

  // Accessors are provided for both points and sides.
  const T& xmin() const { return min_.x(); }
  const T& xmax() const { return max_.x(); }
  const T& ymin() const { return min_.y(); }
  const T& ymax() const { return max_.y(); }

  // Returns the min and max corner points.
  const Point2<T>& min_xy() const { return min_; }
  const Point2<T>& max_xy() const { return max_; }

  // Sets the geometry of the rectangle given two points.
  // The resulting rectangle will have non-negative width and height.
  void Set(const Point2<T>& p0, const Point2<T>& p1);

  // Same as above using vectors as input.
  void Set(const Vector2<T>& p0, const Vector2<T>& p1);

  // Sets the geometry of the rectangle given a minimum point and dimensions.
  void Set(const T& x, const T& y, const T& width, const T& height);

  // Sets the min and max values, and min greater than max is allowable,
  // but the user has to be aware of the consequences such as negative width
  // and height.  Both point and side accessors are provided.
  void set_xmin(const T& x) { min_.set_x(x); }
  void set_xmax(const T& x) { max_.set_x(x); }
  void set_ymin(const T& y) { min_.set_y(y); }
  void set_ymax(const T& y) { max_.set_y(y); }

  void set_min_xy(const Point2<T>& p) { min_.Set(p.x(), p.y()); }
  void set_max_xy(const Point2<T>& p) { max_.Set(p.x(), p.y()); }

  // Expands a rectangle to contain a point or vector.
  void Expand(const T& x, const T& y);
  void Expand(const Point2<T>& p);
  void Expand(const Vector2<T>& p);

  // Expands a rectangle to contain another rectangle.
  void Expand(const Rectangle& other);

  // Returns the union of this rectangle with another rectangle, which
  // is the smallest rectangle that contains both rectangles.
  Rectangle Union(const Rectangle& other) const;

  // Returns the intersection of this rectangle with another rectangle.
  // If the intersection is empty, returns a rectangle initialized by
  // SetEmpty().
  Rectangle Intersect(const Rectangle& other) const;

  // Tests if this rectangle has a non-empty intersection with another rectangle
  // including the boundary.
  bool Intersects(const Rectangle& other) const;

  // Tests if a point is inside or on any of the 4 edges of the rectangle.
  bool Contains(const T& x, const T& y) const;
  bool Contains(const Point2<T>& pt) const;
  bool Contains(const Vector2<T>& pt) const;

  // Tests if a rectangle is inside or on any of the 4 edges of the rectangle.
  bool Contains(const Rectangle& other) const;

  // Translates this rectangle by a vector.
  void Translate(const Vector2<T>& v);

  // Adds a border around the rectangle by subtracting the border size from the
  // min point and adding it to the max point.  The border size can be
  // negative.
  void AddBorder(const T& border_size);

  // Debug printing.
  friend std::ostream& operator<< <T>(std::ostream&, const Rectangle&);

 private:
  Point2<T> min_;
  Point2<T> max_;
};

//
// Inline method definitions.  These are not placed in the definition of the
// class to keep the class interface more readable.
//

template <typename T>
Rectangle<T>::Rectangle(const Point2<T>& p0, const Point2<T>& p1) {
  Set(p0, p1);
}

template <typename T>
Rectangle<T>::Rectangle(const Vector2<T>& p0, const Vector2<T>& p1) {
  Set(p0, p1);
}

template <typename T>
Rectangle<T>::Rectangle(const T& x, const T& y, const T& width,
                        const T& height) {
  Set(x, y, width, height);
}

// The general version works only when T models Integer (there are more
// integer classes than float classes).
template <typename T>
void Rectangle<T>::SetEmpty() {
  T min_value = std::numeric_limits<T>::min();
  T max_value = std::numeric_limits<T>::max();
  min_.Set(max_value, max_value);
  max_.Set(min_value, min_value);
}

template <>
inline void Rectangle<float>::SetEmpty() {
  float max_value = std::numeric_limits<float>::max();
  min_.Set(max_value, max_value);
  max_.Set(-max_value, -max_value);
}

template <>
inline void Rectangle<double>::SetEmpty() {
  double max_value = std::numeric_limits<double>::max();
  min_.Set(max_value, max_value);
  max_.Set(-max_value, -max_value);
}

template <typename T>
bool Rectangle<T>::operator==(const Rectangle<T>& other) const {
  return min_ == other.min_ && max_ == other.max_;
}

template <typename T>
bool Rectangle<T>::operator!=(const Rectangle<T>& other) const {
  return !(*this == other);
}

template <typename T>
void Rectangle<T>::Set(const Vector2<T>& p0, const Vector2<T>& p1) {
  if (p0[0] <= p1[0])
    min_.set_x(p0[0]), max_.set_x(p1[0]);
  else
    max_.set_x(p0[0]), min_.set_x(p1[0]);

  if (p0[1] <= p1[1])
    min_.set_y(p0[1]), max_.set_y(p1[1]);
  else
    max_.set_y(p0[1]), min_.set_y(p1[1]);
}

template <typename T>
void Rectangle<T>::Set(const Point2<T>& p0, const Point2<T>& p1) {
  Set(p0.ToVector(), p1.ToVector());
}

template <typename T>
void Rectangle<T>::Set(const T& x, const T& y, const T& width,
                       const T& height) {
  min_.Set(x, y);
  max_.Set(x + width, y + height);
}

template <typename T>
void Rectangle<T>::Expand(const T& x, const T& y) {
  min_.Set(std::min(x, xmin()), std::min(y, ymin()));
  max_.Set(std::max(x, xmax()), std::max(y, ymax()));
}

template <typename T>
void Rectangle<T>::Expand(const Point2<T>& p) {
  Expand(p.x(), p.y());
}

template <typename T>
void Rectangle<T>::Expand(const Vector2<T>& v) {
  Expand(v[0], v[1]);
}

template <typename T>
void Rectangle<T>::Expand(const Rectangle<T>& other) {
  Expand(other.min_);
  Expand(other.max_);
}

template <typename T>
void Rectangle<T>::Translate(const Vector2<T>& v) {
  min_ += v;
  max_ += v;
}

template <typename T>
bool Rectangle<T>::Contains(const T& x, const T& y) const {
  return x >= xmin() && x <= xmax() && y >= ymin() && y <= ymax();
}

template <typename T>
bool Rectangle<T>::Contains(const Point2<T>& p) const {
  return Contains(p.x(), p.y());
}

template <typename T>
bool Rectangle<T>::Contains(const Vector2<T>& v) const {
  return Contains(v[0], v[1]);
}

template <typename T>
bool Rectangle<T>::Contains(const Rectangle<T>& r) const {
  return Contains(r.min_) && Contains(r.max_);
}

template <typename T>
Rectangle<T> Rectangle<T>::Union(const Rectangle<T>& r) const {
  return Rectangle<T>(
      Point2<T>(std::min(xmin(), r.xmin()), std::min(ymin(), r.ymin())),
      Point2<T>(std::max(xmax(), r.xmax()), std::max(ymax(), r.ymax())));
}

template <typename T>
Rectangle<T> Rectangle<T>::Intersect(const Rectangle<T>& r) const {
  Point2<T> pmin(std::max(xmin(), r.xmin()), std::max(ymin(), r.ymin()));
  Point2<T> pmax(std::min(xmax(), r.xmax()), std::min(ymax(), r.ymax()));

  if (pmin.x() > pmax.x() || pmin.y() > pmax.y())
    return Rectangle<T>();
  else
    return Rectangle<T>(pmin, pmax);
}

template <typename T>
bool Rectangle<T>::Intersects(const Rectangle<T>& r) const {
  return !(IsEmpty() || r.IsEmpty() || r.xmax() < xmin() || xmax() < r.xmin() ||
           r.ymax() < ymin() || ymax() < r.ymin());
}

template <typename T>
void Rectangle<T>::AddBorder(const T& border_size) {
  min_.Set(xmin() - border_size, ymin() - border_size);
  max_.Set(xmax() + border_size, ymax() + border_size);
}

template <typename T>
std::ostream& operator<<(std::ostream& out, const Rectangle<T>& r) {
  out << "[(" << r.xmin() << ", " << r.ymin() << "), (" << r.xmax() << ", "
      << r.ymax() << ")]";
  return out;
}

template <typename T>
class Rectangle;

typedef Rectangle<uint8_t> Rectangle_b;
typedef Rectangle<int> Rectangle_i;
typedef Rectangle<float> Rectangle_f;
typedef Rectangle<double> Rectangle_d;

#endif  // MEDIAPIPE_DEPS_RECTANGLE_H_
