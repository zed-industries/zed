/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This implementation is borrowed from Chromium.

#ifndef RTC_BASE_CONTAINERS_MOVE_ONLY_INT_H_
#define RTC_BASE_CONTAINERS_MOVE_ONLY_INT_H_

namespace webrtc {

// A move-only class that holds an integer. This is designed for testing
// containers. See also CopyOnlyInt.
class MoveOnlyInt {
 public:
  explicit MoveOnlyInt(int data = 1) : data_(data) {}
  MoveOnlyInt(const MoveOnlyInt& other) = delete;
  MoveOnlyInt& operator=(const MoveOnlyInt& other) = delete;
  MoveOnlyInt(MoveOnlyInt&& other) : data_(other.data_) { other.data_ = 0; }
  ~MoveOnlyInt() { data_ = 0; }

  MoveOnlyInt& operator=(MoveOnlyInt&& other) {
    data_ = other.data_;
    other.data_ = 0;
    return *this;
  }

  friend bool operator==(const MoveOnlyInt& lhs, const MoveOnlyInt& rhs) {
    return lhs.data_ == rhs.data_;
  }

  friend bool operator!=(const MoveOnlyInt& lhs, const MoveOnlyInt& rhs) {
    return !operator==(lhs, rhs);
  }

  friend bool operator<(const MoveOnlyInt& lhs, int rhs) {
    return lhs.data_ < rhs;
  }

  friend bool operator<(int lhs, const MoveOnlyInt& rhs) {
    return lhs < rhs.data_;
  }

  friend bool operator<(const MoveOnlyInt& lhs, const MoveOnlyInt& rhs) {
    return lhs.data_ < rhs.data_;
  }

  friend bool operator>(const MoveOnlyInt& lhs, const MoveOnlyInt& rhs) {
    return rhs < lhs;
  }

  friend bool operator<=(const MoveOnlyInt& lhs, const MoveOnlyInt& rhs) {
    return !(rhs < lhs);
  }

  friend bool operator>=(const MoveOnlyInt& lhs, const MoveOnlyInt& rhs) {
    return !(lhs < rhs);
  }

  int data() const { return data_; }

 private:
  volatile int data_;
};

}  // namespace webrtc

#endif  // RTC_BASE_CONTAINERS_MOVE_ONLY_INT_H_
