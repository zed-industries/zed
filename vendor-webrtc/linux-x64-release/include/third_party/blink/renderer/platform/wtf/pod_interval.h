/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_INTERVAL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_INTERVAL_H_

#ifndef NDEBUG
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#endif

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace WTF {

// Class representing a closed interval which can hold an arbitrary
// Plain Old Datatype (POD) as its endpoints and a piece of user
// data. An important characteristic for the algorithms we use is that
// if two intervals have identical endpoints but different user data,
// they are not considered to be equal. This situation can arise when
// representing the vertical extents of bounding boxes of overlapping
// triangles, where the pointer to the triangle is the user data of
// the interval.
//
// *Note* that the destructors of type T and UserData will *not* be
// called by this class. They must not allocate any memory that is
// required to be cleaned up in their destructors.
//
// The following constructors and operators must be implemented on
// type T:
//
//   - Copy constructor (if user data is desired)
//   - operator<
//   - operator==
//   - operator=
//
// If the UserData type is specified, it must support a copy
// constructor and assignment operator.
//
// In debug mode, printing of intervals and the data they contain is
// enabled. This requires the following template specializations to be
// available:
//
//   template<> struct ValueToString<T> {
//       static String toString(const T& t);
//   };
//   template<> struct ValueToString<UserData> {
//       static String toString(const UserData& t);
//   };
//
// Note that this class requires a copy constructor and assignment
// operator in order to be stored in the red-black tree.

#ifndef NDEBUG
template <class T>
struct ValueToString;
#endif

template <class T, class UserData = void*>
class PODInterval {
  DISALLOW_NEW();

 public:
  // Constructor from endpoints. This constructor only works when the
  // UserData type is a pointer or other type which can be initialized
  // with 0.
  PODInterval(const T& low, const T& high)
      : low_(low), high_(high), data_(0), min_low_(low), max_high_(high) {}

  // Constructor from two endpoints plus explicit user data.
  PODInterval(const T& low, const T& high, const UserData data)
      : low_(low), high_(high), data_(data), min_low_(low), max_high_(high) {}

  const T& Low() const { return low_; }
  const T& High() const { return high_; }
  const UserData& Data() const { return data_; }

  bool Overlaps(const T& low, const T& high) const {
    if (this->High() < low)
      return false;
    if (high < this->Low())
      return false;
    return true;
  }

  bool Overlaps(const PODInterval& other) const {
    return Overlaps(other.Low(), other.High());
  }

  // Returns true if this interval is "less" than the other. The
  // comparison is performed on the low endpoints of the intervals.
  bool operator<(const PODInterval& other) const { return Low() < other.Low(); }

  // Returns true if this interval is strictly equal to the other,
  // including comparison of the user data.
  bool operator==(const PODInterval& other) const {
    return (Low() == other.Low() && High() == other.High() &&
            Data() == other.Data());
  }

  const T& MinLow() const { return min_low_; }
  void SetMinLow(const T& min_low) { min_low_ = min_low; }

  const T& MaxHigh() const { return max_high_; }
  void SetMaxHigh(const T& max_high) { max_high_ = max_high; }

#ifndef NDEBUG
  // Support for printing PODIntervals.
  String ToString() const {
    StringBuilder builder;
    builder.Append("[PODInterval (");
    builder.Append(ValueToString<T>::ToString(Low()));
    builder.Append(", ");
    builder.Append(ValueToString<T>::ToString(High()));
    builder.Append("), data=");
    builder.Append(ValueToString<UserData>::ToString(Data()));
    builder.Append(", minLow=");
    builder.Append(ValueToString<T>::ToString(MinLow()));
    builder.Append(", maxHigh=");
    builder.Append(ValueToString<T>::ToString(MaxHigh()));
    builder.Append(']');
    return builder.ToString();
  }
#endif

 private:
  T low_;
  T high_;
  GC_PLUGIN_IGNORE("https://crbug.com/513116") UserData data_;
  T min_low_;
  T max_high_;
};

#ifndef NDEBUG
template <>
struct ValueToString<float> {
  STATIC_ONLY(ValueToString);
  static String ToString(const float value) { return String::Number(value); }
};
template <>
struct ValueToString<double> {
  static String ToString(const double value) { return String::Number(value); }
};
template <>
struct ValueToString<int> {
  static String ToString(const int& value) { return String::Number(value); }
};
#endif

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_INTERVAL_H_
