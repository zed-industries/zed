// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TIME_UTILS_H_
#define TIME_UTILS_H_

#include <stdint.h>

namespace time_utils {

uint64_t GetTimestamp();

class PeriodicTimer {
 public:
  PeriodicTimer(int interval_ms);
  ~PeriodicTimer();

  void Start();
  void Stop();
  // Wait for next tick. Returns false if interrupted by Stop() or not started.
  bool Wait();

 private:
  PeriodicTimer(const PeriodicTimer&) = delete;
  void operator=(const PeriodicTimer&) = delete;

  const int interval_ms_;
  int timer_fd_;
};

}  // namespace time_utils

#endif  // TIME_UTILS_
