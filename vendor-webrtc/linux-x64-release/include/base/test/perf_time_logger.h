// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_PERF_TIME_LOGGER_H_
#define BASE_TEST_PERF_TIME_LOGGER_H_

#include <string>

#include "base/timer/elapsed_timer.h"

namespace base {

// Automates calling LogPerfResult for the common case where you want
// to measure the time that something took. Call Done() when the test
// is complete if you do extra work after the test or there are stack
// objects with potentially expensive constructors. Otherwise, this
// class with automatically log on destruction.
class PerfTimeLogger {
 public:
  explicit PerfTimeLogger(const char* test_name);

  PerfTimeLogger(const PerfTimeLogger&) = delete;
  PerfTimeLogger& operator=(const PerfTimeLogger&) = delete;

  ~PerfTimeLogger();

  void Done();

 private:
  bool logged_;
  std::string test_name_;
  ElapsedTimer timer_;
};

}  // namespace base

#endif  // BASE_TEST_PERF_TIME_LOGGER_H_
