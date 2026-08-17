// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_COPY_ONLY_INT_H_
#define BASE_TEST_COPY_ONLY_INT_H_

#include <utility>

#include "base/functional/callback.h"
#include "third_party/abseil-cpp/absl/cleanup/cleanup.h"

namespace base {

// A copy-only (not moveable) class that holds an integer. This is designed for
// testing containers. See also MoveOnlyInt.
class CopyOnlyInt {
 public:
  explicit CopyOnlyInt(int data = 1) : data_(data) {}
  CopyOnlyInt(const CopyOnlyInt& other) : data_(other.data_) { ++num_copies_; }
  ~CopyOnlyInt();

  friend bool operator==(const CopyOnlyInt& lhs,
                         const CopyOnlyInt& rhs) = default;
  friend auto operator<=>(const CopyOnlyInt& lhs,
                          const CopyOnlyInt& rhs) = default;

  int data() const { return data_; }

  static void reset_num_copies() { num_copies_ = 0; }

  static int num_copies() { return num_copies_; }

  // Called with the value of `data()` when an instance of `CopyOnlyInt` is
  // destroyed. Returns an `absl::Cleanup` scoper that automatically
  // unregisters the callback when the scoper is destroyed.
  static auto SetScopedDestructionCallback(
      RepeatingCallback<void(int)> callback) {
    GetDestructionCallbackStorage() = std::move(callback);
    return absl::Cleanup([] { GetDestructionCallbackStorage().Reset(); });
  }

 private:
  static RepeatingCallback<void(int)>& GetDestructionCallbackStorage();

  volatile int data_;

  static int num_copies_;

  CopyOnlyInt(CopyOnlyInt&&) = delete;
  CopyOnlyInt& operator=(CopyOnlyInt&) = delete;
};

}  // namespace base

#endif  // BASE_TEST_COPY_ONLY_INT_H_
