// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_UTIL_STRONG_ALIAS_H_
#define IPCZ_SRC_UTIL_STRONG_ALIAS_H_

#include <utility>

namespace ipcz {

// Type-safe alternative for C++ type aliases. This is derived from Chromium's
// base::StrongAlias, with minimal features as needed by ipcz implementation.
template <typename TagType, typename UnderlyingType>
class StrongAlias {
 public:
  constexpr StrongAlias() = default;
  constexpr explicit StrongAlias(const UnderlyingType& v) : value_(v) {}
  constexpr explicit StrongAlias(UnderlyingType&& v) noexcept
      : value_(std::move(v)) {}

  constexpr UnderlyingType* operator->() { return &value_; }
  constexpr const UnderlyingType* operator->() const { return &value_; }

  constexpr UnderlyingType& value() & { return value_; }
  constexpr const UnderlyingType& value() const& { return value_; }

  constexpr explicit operator const UnderlyingType&() const& { return value_; }

  constexpr bool operator==(const StrongAlias& other) const {
    return value_ == other.value_;
  }
  constexpr bool operator!=(const StrongAlias& other) const {
    return value_ != other.value_;
  }
  constexpr bool operator<(const StrongAlias& other) const {
    return value_ < other.value_;
  }
  constexpr bool operator<=(const StrongAlias& other) const {
    return value_ <= other.value_;
  }
  constexpr bool operator>(const StrongAlias& other) const {
    return value_ > other.value_;
  }
  constexpr bool operator>=(const StrongAlias& other) const {
    return value_ >= other.value_;
  }

  // Support for absl::Hash.
  template <typename H>
  friend H AbslHashValue(H h, const StrongAlias& alias) {
    return H::combine(std::move(h), alias.value_);
  }

 protected:
  UnderlyingType value_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_UTIL_STRONG_ALIAS_H_
