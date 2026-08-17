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
// Simple classes to handle vectors in 2D, 3D, and 4D.
#ifndef MEDIAPIPE_DEPS_VECTOR_H_
#define MEDIAPIPE_DEPS_VECTOR_H_

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <iosfwd>
#include <iostream>  // NOLINT(readability/streams)
#include <limits>
#include <type_traits>

#include "absl/log/absl_check.h"
#include "absl/utility/utility.h"

template <typename T>
class Vector2;
template <typename T>
class Vector3;
template <typename T>
class Vector4;

namespace mediapipe {
namespace deps {
namespace internal_vector {

// CRTP base class for all Vector templates.
template <template <typename> class VecTemplate, typename T, std::size_t N>
class BasicVector {
  using D = VecTemplate<T>;

 protected:
  // FloatType is the type returned by Norm() and Angle().  These methods are
  // special because they return floating-point values even when VType is an
  // integer.
  typedef typename std::conditional<std::is_integral<T>::value, double, T>::type
      FloatType;

  using IdxSeqN = typename absl::make_index_sequence<N>;

  template <std::size_t I, typename F, typename... As>
  static auto Reduce(F f, As*... as) -> decltype(f(as[I]...)) {
    return f(as[I]...);
  }

  template <typename R = D, std::size_t... Is, typename F, typename... As>
  static R GenerateEach(absl::index_sequence<Is...>, F f, As*... as) {
    return R(Reduce<Is>(f, as...)...);
  }

  // Generate<R>(f,a,b,...) returns an R(...), where the constructor arguments
  // are created as a transform. R(f(a[0],b[0],...), f(a[1],b[1],...), ...),
  // and with a,b,...  all optional.
  template <typename R = D, typename F, typename... As>
  static R Generate(F f, As&&... as) {
    return GenerateEach<R>(IdxSeqN(), f, std::forward<As>(as).Data()...);
  }

 public:
  enum { SIZE = N };
  static int Size() { return SIZE; }

  void Clear() { AsD() = D(); }

  T& operator[](int b) {
    ABSL_DCHECK_GE(b, 0);
    ABSL_DCHECK_LT(b, SIZE);
    return static_cast<D&>(*this).Data()[b];
  }
  T operator[](int b) const {
    ABSL_DCHECK_GE(b, 0);
    ABSL_DCHECK_LT(b, SIZE);
    return static_cast<const D&>(*this).Data()[b];
  }

  // TODO: Relationals should be nonmembers.
  bool operator==(const D& b) const {
    const T* ap = static_cast<const D&>(*this).Data();
    return std::equal(ap, ap + this->Size(), b.Data());
  }
  bool operator!=(const D& b) const { return !(AsD() == b); }
  bool operator<(const D& b) const {
    const T* ap = static_cast<const D&>(*this).Data();
    const T* bp = b.Data();
    return std::lexicographical_compare(ap, ap + this->Size(), bp,
                                        bp + b.Size());
  }
  bool operator>(const D& b) const { return b < AsD(); }
  bool operator<=(const D& b) const { return !(AsD() > b); }
  bool operator>=(const D& b) const { return !(AsD() < b); }

  D& operator+=(const D& b) {
    PlusEq(static_cast<D&>(*this).Data(), b.Data(), IdxSeqN{});
    return static_cast<D&>(*this);
  }

  D& operator-=(const D& b) {
    MinusEq(static_cast<D&>(*this).Data(), b.Data(), IdxSeqN{});
    return static_cast<D&>(*this);
  }

  D& operator*=(T k) {
    MulEq(static_cast<D&>(*this).Data(), k, IdxSeqN{});
    return static_cast<D&>(*this);
  }

  D& operator/=(T k) {
    DivEq(static_cast<D&>(*this).Data(), k, IdxSeqN{});
    return static_cast<D&>(*this);
  }

  D operator+(const D& b) const { return D(AsD()) += b; }
  D operator-(const D& b) const { return D(AsD()) -= b; }
  D operator*(T k) const { return D(AsD()) *= k; }
  D operator/(T k) const { return D(AsD()) /= k; }

  friend D operator-(const D& a) {
    return Generate([](const T& x) { return -x; }, a);
  }

  // Convert from another vector type
  template <typename T2>
  static D Cast(const VecTemplate<T2>& b) {
    return Generate([](const T2& x) { return static_cast<T>(x); }, b);
  }

  // multiply two vectors component by component
  D MulComponents(const D& b) const {
    return Generate([](const T& x, const T& y) { return x * y; }, AsD(), b);
  }
  // divide two vectors component by component
  D DivComponents(const D& b) const {
    return Generate([](const T& x, const T& y) { return x / y; }, AsD(), b);
  }

  // Element-wise max.  {max(a[0],b[0]), max(a[1],b[1]), ...}
  friend D Max(const D& a, const D& b) {
    return Generate([](const T& x, const T& y) { return std::max(x, y); }, a,
                    b);
  }

  // Element-wise min.  {min(a[0],b[0]), min(a[1],b[1]), ...}
  friend D Min(const D& a, const D& b) {
    return Generate([](const T& x, const T& y) { return std::min(x, y); }, a,
                    b);
  }

  T DotProd(const D& b) const {
    return Dot(static_cast<T>(0), static_cast<const D&>(*this).Data(), b.Data(),
               IdxSeqN{});
  }

  // Squared Euclidean norm (the dot product with itself).
  T Norm2() const { return DotProd(AsD()); }

  // Euclidean norm. For integer T, correct only if Norm2 does not overflow.
  FloatType Norm() const {
    using std::sqrt;
    return sqrt(Norm2());
  }

  // Normalized vector if the norm is nonzero. Not for integer types.
  D Normalize() const {
    static_assert(!std::is_integral<T>::value, "must be floating point");
    T n = Norm();
    if (n != T(0.0)) {
      n = T(1.0) / n;
    }
    return D(AsD()) *= n;
  }

  // Compose a vector from the sqrt of each component.
  D Sqrt() const {
    return Generate(
        [](const T& x) {
          using std::sqrt;
          return sqrt(x);
        },
        AsD());
  }

  // Take the floor of each component.
  D Floor() const {
    return Generate([](const T& x) { return floor(x); }, AsD());
  }

  // Take the ceil of each component.
  D Ceil() const {
    return Generate([](const T& x) { return ceil(x); }, AsD());
  }

  // Round of each component.
  D FRound() const {
    using std::rint;
    return Generate([](const T& x) { return rint(x); }, AsD());
  }

  // Round of each component and return an integer vector.
  VecTemplate<int> IRound() const {
    using std::lrint;
    return Generate<VecTemplate<int>>([](const T& x) { return lrint(x); },
                                      AsD());
  }

  // True if any of the components is not a number.
  bool IsNaN() const {
    bool r = false;
    const T* ap = AsD().Data();
    for (int i = 0; i < SIZE; ++i) r = r || isnan(ap[i]);
    return r;
  }

  // A Vector populated with all NaN values.
  static D NaN() {
    return Generate([] { return std::numeric_limits<T>::quiet_NaN(); });
  }

  friend std::ostream& operator<<(std::ostream& out, const D& v) {
    out << "[";
    const char* sep = "";
    for (int i = 0; i < SIZE; ++i) {
      out << sep;
      Print(out, v[i]);
      sep = ", ";
    }
    return out << "]";
  }

  // These are only public for technical reasons.
  template <typename K>
  D MulScalarInternal(const K& k) const {
    return Generate([k](const T& x) { return k * x; }, AsD());
  }
  template <typename K>
  D DivScalarInternal(const K& k) const {
    return Generate([k](const T& x) { return k / x; }, AsD());
  }

 private:
  const D& AsD() const { return static_cast<const D&>(*this); }
  D& AsD() { return static_cast<D&>(*this); }

  // ostream << uint8 prints the ASCII character, which is not useful.
  // Cast to int so that numbers will be printed instead.
  template <typename U>
  static void Print(std::ostream& out, const U& v) {
    out << v;
  }
  static void Print(std::ostream& out, uint8_t v) {
    out << static_cast<int>(v);
  }

  // Ignores its arguments so that side-effects of variadic unpacking can occur.
  static void Ignore(std::initializer_list<bool>) {}

  template <std::size_t... Is>
  static T Dot(T sum, const T* a, const T* b, absl::index_sequence<Is...>) {
    Ignore({(sum += a[Is] * b[Is], true)...});
    return sum;
  }

  template <std::size_t... Is>
  static void PlusEq(T* a, const T* b, absl::index_sequence<Is...>) {
    Ignore({(a[Is] += b[Is], true)...});
  }

  template <std::size_t... Is>
  static void MinusEq(T* a, const T* b, absl::index_sequence<Is...>) {
    Ignore({(a[Is] -= b[Is], true)...});
  }

  template <std::size_t... Is>
  static void MulEq(T* a, T b, absl::index_sequence<Is...>) {
    Ignore({(a[Is] *= b, true)...});
  }

  template <std::size_t... Is>
  static void DivEq(T* a, T b, absl::index_sequence<Is...>) {
    Ignore({(a[Is] /= b, true)...});
  }
};

// These templates must be defined outside of BasicVector so that the
// template specialization match algorithm must deduce 'a'.
template <typename K, template <typename> class VT2, typename T2,
          std::size_t N2>
VT2<T2> operator*(const K& k, const BasicVector<VT2, T2, N2>& a) {
  return a.MulScalarInternal(k);
}
template <typename K, template <typename> class VT2, typename T2,
          std::size_t N2>
VT2<T2> operator/(const K& k, const BasicVector<VT2, T2, N2>& a) {
  return a.DivScalarInternal(k);
}

}  // namespace internal_vector
}  // namespace deps
}  // namespace mediapipe

// ======================================================================
template <typename T>
class Vector2
    : public mediapipe::deps::internal_vector::BasicVector<Vector2, T, 2> {
 private:
  using Base = mediapipe::deps::internal_vector::BasicVector<::Vector2, T, 2>;
  using VType = T;

 public:
  typedef VType BaseType;
  using FloatType = typename Base::FloatType;
  using Base::SIZE;

  Vector2() : c_() {}
  Vector2(T x, T y) {
    c_[0] = x;
    c_[1] = y;
  }
  explicit Vector2(const Vector3<T>& b) : Vector2(b.x(), b.y()) {}
  explicit Vector2(const Vector4<T>& b) : Vector2(b.x(), b.y()) {}

  T* Data() { return c_; }
  const T* Data() const { return c_; }

  void x(T v) { c_[0] = v; }
  void y(T v) { c_[1] = v; }
  T x() const { return c_[0]; }
  T y() const { return c_[1]; }

  bool aequal(const Vector2& vb, FloatType margin) const {
    using std::fabs;
    return (fabs(c_[0] - vb.c_[0]) < margin) &&
           (fabs(c_[1] - vb.c_[1]) < margin);
  }

  void Set(T x, T y) { *this = Vector2(x, y); }

  // Cross product.  Be aware that if T is an integer type, the high bits
  // of the result are silently discarded.
  T CrossProd(const Vector2& vb) const {
    return c_[0] * vb.c_[1] - c_[1] * vb.c_[0];
  }

  // Returns the angle between "this" and v in radians. If either vector is
  // zero-length, or nearly zero-length, the result will be zero, regardless of
  // the other value.
  FloatType Angle(const Vector2& v) const {
    using std::atan2;
    return atan2(CrossProd(v), this->DotProd(v));
  }

  // return a vector orthogonal to the current one
  // with the same norm and counterclockwise to it
  Vector2 Ortho() const { return Vector2(-c_[1], c_[0]); }

  // TODO: unify Fabs/Abs between all Vector classes.
  Vector2 Fabs() const {
    using std::fabs;
    return Vector2(fabs(c_[0]), fabs(c_[1]));
  }
  Vector2 Abs() const {
    static_assert(std::is_integral<VType>::value, "use Fabs for float_types");
    static_assert(static_cast<VType>(-1) == -1, "type must be signed");
    static_assert(sizeof(c_[0]) <= sizeof(int), "Abs truncates to int");
    return Vector2(abs(c_[0]), abs(c_[1]));
  }

 private:
  VType c_[SIZE];
};

template <typename T>
class Vector3
    : public mediapipe::deps::internal_vector::BasicVector<Vector3, T, 3> {
 private:
  using Base = mediapipe::deps::internal_vector::BasicVector<::Vector3, T, 3>;
  using VType = T;

 public:
  typedef VType BaseType;
  using FloatType = typename Base::FloatType;
  using Base::SIZE;

  Vector3() : c_() {}
  Vector3(T x, T y, T z) {
    c_[0] = x;
    c_[1] = y;
    c_[2] = z;
  }
  Vector3(const Vector2<T>& b, T z) : Vector3(b.x(), b.y(), z) {}
  explicit Vector3(const Vector4<T>& b) : Vector3(b.x(), b.y(), b.z()) {}

  T* Data() { return c_; }
  const T* Data() const { return c_; }

  void x(const T& v) { c_[0] = v; }
  void y(const T& v) { c_[1] = v; }
  void z(const T& v) { c_[2] = v; }
  T x() const { return c_[0]; }
  T y() const { return c_[1]; }
  T z() const { return c_[2]; }

  bool aequal(const Vector3& vb, FloatType margin) const {
    using std::abs;
    return (abs(c_[0] - vb.c_[0]) < margin) &&
           (abs(c_[1] - vb.c_[1]) < margin) && (abs(c_[2] - vb.c_[2]) < margin);
  }

  void Set(T x, T y, T z) { *this = Vector3(x, y, z); }

  // Cross product.  Be aware that if VType is an integer type, the high bits
  // of the result are silently discarded.
  Vector3 CrossProd(const Vector3& vb) const {
    return Vector3(c_[1] * vb.c_[2] - c_[2] * vb.c_[1],
                   c_[2] * vb.c_[0] - c_[0] * vb.c_[2],
                   c_[0] * vb.c_[1] - c_[1] * vb.c_[0]);
  }

  // Returns a unit vector orthogonal to this one.
  Vector3 Ortho() const {
    int k = LargestAbsComponent() - 1;
    if (k < 0) k = 2;
    Vector3 temp;
    temp[k] = T(1);
    return CrossProd(temp).Normalize();
  }

  // Returns the angle between two vectors in radians. If either vector is
  // zero-length, or nearly zero-length, the result will be zero, regardless of
  // the other value.
  FloatType Angle(const Vector3& va) const {
    using std::atan2;
    return atan2(CrossProd(va).Norm(), this->DotProd(va));
  }

  Vector3 Fabs() const { return Abs(); }

  Vector3 Abs() const {
    static_assert(
        !std::is_integral<VType>::value || static_cast<VType>(-1) == -1,
        "type must be signed");
    using std::abs;
    return Vector3(abs(c_[0]), abs(c_[1]), abs(c_[2]));
  }

  // return the index of the largest component (fabs)
  int LargestAbsComponent() const {
    Vector3 temp = Abs();
    return temp[0] > temp[1]   ? temp[0] > temp[2] ? 0 : 2
           : temp[1] > temp[2] ? 1
                               : 2;
  }

  // return the index of the smallest, median ,largest component of the vector
  Vector3<int> ComponentOrder() const {
    using std::swap;
    Vector3<int> temp(0, 1, 2);
    if (c_[temp[0]] > c_[temp[1]]) swap(temp[0], temp[1]);
    if (c_[temp[1]] > c_[temp[2]]) swap(temp[1], temp[2]);
    if (c_[temp[0]] > c_[temp[1]]) swap(temp[0], temp[1]);
    return temp;
  }

 private:
  VType c_[SIZE];
};

template <typename T>
class Vector4
    : public mediapipe::deps::internal_vector::BasicVector<Vector4, T, 4> {
 private:
  using Base = mediapipe::deps::internal_vector::BasicVector<::Vector4, T, 4>;
  using VType = T;

 public:
  typedef VType BaseType;
  using FloatType = typename Base::FloatType;
  using Base::SIZE;

  Vector4() : c_() {}
  Vector4(T x, T y, T z, T w) {
    c_[0] = x;
    c_[1] = y;
    c_[2] = z;
    c_[3] = w;
  }

  Vector4(const Vector2<T>& b, T z, T w) : Vector4(b.x(), b.y(), z, w) {}
  Vector4(const Vector2<T>& a, const Vector2<T>& b)
      : Vector4(a.x(), a.y(), b.x(), b.y()) {}
  Vector4(const Vector3<T>& b, T w) : Vector4(b.x(), b.y(), b.z(), w) {}

  T* Data() { return c_; }
  const T* Data() const { return c_; }

  bool aequal(const Vector4& vb, FloatType margin) const {
    using std::fabs;
    return (fabs(c_[0] - vb.c_[0]) < margin) &&
           (fabs(c_[1] - vb.c_[1]) < margin) &&
           (fabs(c_[2] - vb.c_[2]) < margin) &&
           (fabs(c_[3] - vb.c_[3]) < margin);
  }

  void x(const T& v) { c_[0] = v; }
  void y(const T& v) { c_[1] = v; }
  void z(const T& v) { c_[2] = v; }
  void w(const T& v) { c_[3] = v; }
  T x() const { return c_[0]; }
  T y() const { return c_[1]; }
  T z() const { return c_[2]; }
  T w() const { return c_[3]; }

  void Set(T x, T y, T z, T w) { *this = Vector4(x, y, z, w); }

  Vector4 Fabs() const {
    using std::fabs;
    return Vector4(fabs(c_[0]), fabs(c_[1]), fabs(c_[2]), fabs(c_[3]));
  }

  Vector4 Abs() const {
    static_assert(std::is_integral<VType>::value, "use Fabs for float types");
    static_assert(static_cast<VType>(-1) == -1, "type must be signed");
    static_assert(sizeof(c_[0]) <= sizeof(int), "Abs truncates to int");
    return Vector4(abs(c_[0]), abs(c_[1]), abs(c_[2]), abs(c_[3]));
  }

 private:
  VType c_[SIZE];
};

typedef Vector2<uint8_t> Vector2_b;
typedef Vector2<int16_t> Vector2_s;
typedef Vector2<int> Vector2_i;
typedef Vector2<float> Vector2_f;
typedef Vector2<double> Vector2_d;

typedef Vector3<uint8_t> Vector3_b;
typedef Vector3<int16_t> Vector3_s;
typedef Vector3<int> Vector3_i;
typedef Vector3<float> Vector3_f;
typedef Vector3<double> Vector3_d;

typedef Vector4<uint8_t> Vector4_b;
typedef Vector4<int16_t> Vector4_s;
typedef Vector4<int> Vector4_i;
typedef Vector4<float> Vector4_f;
typedef Vector4<double> Vector4_d;

#endif  // MEDIAPIPE_DEPS_VECTOR_H_
