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
// Class to handle two-dimensional points.
//
// The aim of this class is to be able to do sensible geometric operations
// with points and vectors, which are distinct mathematical concepts.
// Operators +, -, =, ==, <, etc. are overloaded with the proper semantics
// (e.g. Point = Point + constant * vector or Vector = Point - Point).
// For more about Point expressions, see Goldman, Ronald N., "Illicit
// Expressions in Vector Algebra," ACM Transactions on Graphics, 4(3),
// pp. 223-243, July 1985 (http://portal.acm.org/citation.cfm?id=282969).
//
// Please be careful about overflows when using points with integer types
// The calculations are carried with the same type as the vector's components
// type, e.g. if you are using uint8 as the base type, all values will be modulo
// 256.  This feature is necessary to use the class in a more general framework
// where T != plain old data type.

#ifndef MEDIAPIPE_DEPS_POINT2_H_
#define MEDIAPIPE_DEPS_POINT2_H_

#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <iosfwd>

#include "mediapipe/framework/deps/mathutil.h"
#include "mediapipe/framework/deps/vector.h"
#include "mediapipe/framework/port/logging.h"

// Template class for 2D points
template <typename T>
class Point2 {
 public:
  typedef T ElementType;
  typedef Vector2<T> Coords;

  Point2() {}
  Point2(const T& x, const T& y) : c_(x, y) {}
  explicit Point2(const Coords& v) : c_(v) {}

  Coords ToVector() const { return c_; }

  void Set(const T& x, const T& y) { *this = Point2(x, y); }

  T* Data() { return c_.Data(); }
  const T* Data() const { return c_.Data(); }

  void Clear() { *this = Point2(); }

  Point2& operator+=(const Coords& v) {
    c_ += v;
    return *this;
  }
  Point2& operator-=(const Coords& v) {
    c_ -= v;
    return *this;
  }

  const T& operator[](std::size_t b) const { return Data()[b]; }
  T& operator[](std::size_t b) { return Data()[b]; }

  const T& x() const { return (*this)[0]; }
  const T& y() const { return (*this)[1]; }
  void set_x(const T& x) { (*this)[0] = x; }
  void set_y(const T& y) { (*this)[1] = y; }

  // Compares two points, returns true if all their components are within
  // a difference of a tolerance.
  bool aequal(const Point2& p, double tolerance) const {
    using std::abs;
    return (abs(c_[0] - p.c_[0]) <= tolerance) &&
           (abs(c_[1] - p.c_[1]) <= tolerance);
  }

 private:
  // Friend arithmetic operators.
  friend Point2 operator+(const Point2& p, const Coords& v) {
    return Point2(p.c_ + v);
  }
  friend Point2 operator+(const Coords& v, const Point2& p) {
    return Point2(v + p.c_);
  }
  friend Point2 operator-(const Point2& p, const Coords& v) {
    return Point2(p.c_ - v);
  }
  friend Coords operator-(const Point2& p1, const Point2& p2) {
    return p1.c_ - p2.c_;
  }

  // Friend relational nonmember operators.
  friend bool operator==(const Point2& a, const Point2& b) {
    return a.c_ == b.c_;
  }
  friend bool operator!=(const Point2& a, const Point2& b) {
    return a.c_ != b.c_;
  }
  friend bool operator<(const Point2& a, const Point2& b) {
    return a.c_ < b.c_;
  }
  friend bool operator>(const Point2& a, const Point2& b) {
    return a.c_ > b.c_;
  }
  friend bool operator<=(const Point2& a, const Point2& b) {
    return a.c_ <= b.c_;
  }
  friend bool operator>=(const Point2& a, const Point2& b) {
    return a.c_ >= b.c_;
  }

  // Streaming operator.
  friend std::ostream& operator<<(std::ostream& out, const Point2& p) {
    return out << "Point with coordinates: (" << p.c_[0] << ", " << p.c_[1]
               << ")";
  }

  Coords c_;  // coordinates
};

typedef Point2<uint8_t> Point2_b;
typedef Point2<int> Point2_i;
typedef Point2<float> Point2_f;
typedef Point2<double> Point2_d;

#endif  // MEDIAPIPE_DEPS_POINT2_H_
