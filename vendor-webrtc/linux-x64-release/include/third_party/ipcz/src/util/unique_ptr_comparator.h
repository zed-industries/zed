// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_UTIL_UNIQUE_PTR_COMPARATOR_H_
#define IPCZ_SRC_UTIL_UNIQUE_PTR_COMPARATOR_H_

#include <memory>

namespace ipcz {

// Cribbed from Chromium base, this defines a transparent comparator so that
// unique_ptr keys can be compared against raw pointers for lookup in various
// types of associative containers.
struct UniquePtrComparator {
  using is_transparent = int;

  template <typename T, class Deleter = std::default_delete<T>>
  bool operator()(const std::unique_ptr<T, Deleter>& lhs,
                  const std::unique_ptr<T, Deleter>& rhs) const {
    return lhs < rhs;
  }

  template <typename T, class Deleter = std::default_delete<T>>
  bool operator()(const T* lhs, const std::unique_ptr<T, Deleter>& rhs) const {
    return lhs < rhs.get();
  }

  template <typename T, class Deleter = std::default_delete<T>>
  bool operator()(const std::unique_ptr<T, Deleter>& lhs, const T* rhs) const {
    return lhs.get() < rhs;
  }
};

}  // namespace ipcz

#endif  // IPCZ_SRC_UTIL_UNIQUE_PTR_COMPARATOR_H_
