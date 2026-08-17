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

#ifndef MEDIAPIPE_FRAMEWORK_COLLECTION_ITEM_ID_H_
#define MEDIAPIPE_FRAMEWORK_COLLECTION_ITEM_ID_H_

#include "mediapipe/framework/deps/strong_int.h"

namespace mediapipe {

namespace tool {
class TagMap;
}  // namespace tool

// TagMap allows access to a collection using a tag and index value.
// The underlying data in the collection is stored in a flat array.
// CollectionItemId is the index into that array.  Although this type is
// conceptually an int we don't allow implicit type conversion so as to
// avoid confusion where a user accidentally forgets to query the TagMap
// to get an actual CollectionItemId.
// For example, accidentally using Inputs().Get(2) when Inputs().Index(2)
// was meant will cause a type error.
class CollectionItemId {
 public:
  // Static function to return an invalid id.
  static const CollectionItemId GetInvalid() { return CollectionItemId(); }

  // Construct an invalid CollectionItemId.
  constexpr CollectionItemId() : value_(-1) {}

  // Use the default copy constructor, assignment, and destructor.
  CollectionItemId(const CollectionItemId&) = default;
  ~CollectionItemId() = default;
  CollectionItemId& operator=(const CollectionItemId&) = default;

  bool IsValid() const { return value_ >= 0; }
  // Accesses the raw value.
  constexpr int value() const { return value_; }

  // Unary operators.
  bool operator!() const { return value_ == 0; }
  const CollectionItemId operator+() const { return CollectionItemId(value_); }
  const CollectionItemId operator-() const { return CollectionItemId(-value_); }

  // Increment and decrement operators.
  CollectionItemId& operator++() {  // ++x
    ++value_;
    return *this;
  }
  const CollectionItemId operator++(int postfix_flag) {  // x++
    CollectionItemId temp(*this);
    ++value_;
    return temp;
  }
  CollectionItemId& operator--() {  // --x
    --value_;
    return *this;
  }
  const CollectionItemId operator--(int postfix_flag) {  // x--
    CollectionItemId temp(*this);
    --value_;
    return temp;
  }

  // Action-Assignment operators.
  CollectionItemId& operator+=(CollectionItemId arg) {
    value_ += arg.value_;
    return *this;
  }
  CollectionItemId operator+(CollectionItemId arg) const {
    return CollectionItemId(value_ + arg.value_);
  }
  template <typename ArgType>
  CollectionItemId operator+(ArgType arg) const {
    return CollectionItemId(value_ + arg);
  }

  CollectionItemId& operator-=(CollectionItemId arg) {
    value_ -= arg.value_;
    return *this;
  }
  CollectionItemId operator-(CollectionItemId arg) const {
    return CollectionItemId(value_ - arg.value_);
  }
  template <typename ArgType>
  CollectionItemId operator-(ArgType arg) const {
    return CollectionItemId(value_ - arg);
  }

  template <typename ArgType>
  CollectionItemId& operator*=(ArgType arg) {
    value_ *= arg;
    return *this;
  }
  CollectionItemId operator*(CollectionItemId arg) const {
    return CollectionItemId(value_ * arg.value_);
  }
  template <typename ArgType>
  CollectionItemId operator*(ArgType arg) const {
    return CollectionItemId(value_ * arg);
  }

  template <typename ArgType>
  CollectionItemId& operator/=(ArgType arg) {
    value_ /= arg;
    return *this;
  }
  CollectionItemId operator/(CollectionItemId arg) const {
    return CollectionItemId(value_ / arg.value_);
  }
  template <typename ArgType>
  CollectionItemId operator/(ArgType arg) const {
    return CollectionItemId(value_ / arg);
  }

  template <typename ArgType>
  CollectionItemId& operator%=(ArgType arg) {
    value_ %= arg;
    return *this;
  }
  CollectionItemId operator%(CollectionItemId arg) const {
    return CollectionItemId(value_ % arg.value_);
  }
  template <typename ArgType>
  CollectionItemId operator%(ArgType arg) const {
    return CollectionItemId(value_ % arg);
  }

  inline bool operator>(CollectionItemId rhs) const {
    return value_ > rhs.value_;
  }
  inline bool operator>=(CollectionItemId rhs) const {
    return value_ >= rhs.value_;
  }
  inline bool operator<(CollectionItemId rhs) const {
    return value_ < rhs.value_;
  }
  inline bool operator<=(CollectionItemId rhs) const {
    return value_ <= rhs.value_;
  }
  inline bool operator==(CollectionItemId rhs) const {
    return value_ == rhs.value_;
  }
  inline bool operator!=(CollectionItemId rhs) const {
    return value_ != rhs.value_;
  }

 private:
  friend class mediapipe::tool::TagMap;

  // Initialization from a value.
  explicit constexpr CollectionItemId(int init_value) : value_(init_value) {}

  // The integer value of type int.
  int value_;
};

std::ostream& operator<<(std::ostream& os, CollectionItemId arg);

CollectionItemId operator+(int lhs, CollectionItemId rhs);
CollectionItemId operator-(int lhs, CollectionItemId rhs);
CollectionItemId operator*(int lhs, CollectionItemId rhs);

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_COLLECTION_ITEM_ID_H_
