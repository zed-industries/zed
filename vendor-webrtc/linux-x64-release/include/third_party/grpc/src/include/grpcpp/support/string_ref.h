//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPCPP_SUPPORT_STRING_REF_H
#define GRPCPP_SUPPORT_STRING_REF_H

#include <grpcpp/support/config.h>
#include <string.h>

#include <algorithm>
#include <iosfwd>
#include <iostream>
#include <iterator>

namespace grpc {

/// This class is a non owning reference to a string.
///
/// It should be a strict subset of the upcoming std::string_ref.
///
/// \see http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2012/n3442.html
///
/// The constexpr is dropped or replaced with const for legacy compiler
/// compatibility.
class string_ref {
 public:
  /// types
  typedef const char* const_iterator;
  typedef std::reverse_iterator<const_iterator> const_reverse_iterator;

  /// constants
  const static size_t npos;

  /// construct/copy.
  string_ref() : data_(nullptr), length_(0) {}
  string_ref(const string_ref& other)
      : data_(other.data_), length_(other.length_) {}
  // NOLINTNEXTLINE(bugprone-unhandled-self-assignment)
  string_ref& operator=(const string_ref& rhs) {
    data_ = rhs.data_;
    length_ = rhs.length_;
    return *this;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  string_ref(const char* s) : data_(s), length_(strlen(s)) {}
  string_ref(const char* s, size_t l) : data_(s), length_(l) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  string_ref(const std::string& s) : data_(s.data()), length_(s.length()) {}

  /// iterators
  const_iterator begin() const { return data_; }
  const_iterator end() const { return data_ + length_; }
  const_iterator cbegin() const { return data_; }
  const_iterator cend() const { return data_ + length_; }
  const_reverse_iterator rbegin() const {
    return const_reverse_iterator(end());
  }
  const_reverse_iterator rend() const {
    return const_reverse_iterator(begin());
  }
  const_reverse_iterator crbegin() const {
    return const_reverse_iterator(end());
  }
  const_reverse_iterator crend() const {
    return const_reverse_iterator(begin());
  }

  /// capacity
  size_t size() const { return length_; }
  size_t length() const { return length_; }
  size_t max_size() const { return length_; }
  bool empty() const { return length_ == 0; }

  /// element access
  const char* data() const { return data_; }

  /// string operations
  int compare(string_ref x) const {
    size_t min_size = length_ < x.length_ ? length_ : x.length_;
    int r = memcmp(data_, x.data_, min_size);
    if (r < 0) return -1;
    if (r > 0) return 1;
    if (length_ < x.length_) return -1;
    if (length_ > x.length_) return 1;
    return 0;
  }

  bool starts_with(string_ref x) const {
    return length_ >= x.length_ && (memcmp(data_, x.data_, x.length_) == 0);
  }

  bool ends_with(string_ref x) const {
    return length_ >= x.length_ &&
           (memcmp(data_ + (length_ - x.length_), x.data_, x.length_) == 0);
  }

  size_t find(string_ref s) const {
    auto it = std::search(cbegin(), cend(), s.cbegin(), s.cend());
    return it == cend() ? npos : std::distance(cbegin(), it);
  }

  size_t find(char c) const {
    auto it = std::find(cbegin(), cend(), c);
    return it == cend() ? npos : std::distance(cbegin(), it);
  }

  string_ref substr(size_t pos, size_t n = npos) const {
    if (pos > length_) pos = length_;
    if (n > (length_ - pos)) n = length_ - pos;
    return string_ref(data_ + pos, n);
  }

 private:
  const char* data_;
  size_t length_;
};

/// Comparison operators
inline bool operator==(string_ref x, string_ref y) { return x.compare(y) == 0; }
inline bool operator!=(string_ref x, string_ref y) { return x.compare(y) != 0; }
inline bool operator<(string_ref x, string_ref y) { return x.compare(y) < 0; }
inline bool operator<=(string_ref x, string_ref y) { return x.compare(y) <= 0; }
inline bool operator>(string_ref x, string_ref y) { return x.compare(y) > 0; }
inline bool operator>=(string_ref x, string_ref y) { return x.compare(y) >= 0; }

inline std::ostream& operator<<(std::ostream& out, const string_ref& string) {
  return out << std::string(string.begin(), string.end());
}

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_STRING_REF_H
