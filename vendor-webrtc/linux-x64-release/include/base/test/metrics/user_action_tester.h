// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_METRICS_USER_ACTION_TESTER_H_
#define BASE_TEST_METRICS_USER_ACTION_TESTER_H_

#include <map>
#include <string>
#include <string_view>
#include <vector>

#include "base/memory/scoped_refptr.h"
#include "base/metrics/user_metrics.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"

namespace base {

class TimeTicks;

// This class observes and collects user action notifications that are sent
// by the tests, so that they can be examined afterwards for correctness.
// Note: This class is NOT thread-safe.
class UserActionTester {
 public:
  UserActionTester();

  UserActionTester(const UserActionTester&) = delete;
  UserActionTester& operator=(const UserActionTester&) = delete;

  ~UserActionTester();

  // Returns the number of times the given |user_action| occurred.
  int GetActionCount(std::string_view user_action) const;

  // Returns the time values at which the given |user_action| has occurred.
  // The order of returned values is unspecified.
  std::vector<TimeTicks> GetActionTimes(std::string_view user_action) const;

  // Resets all user action counts to 0.
  void ResetCounts();

 private:
  typedef std::multimap<std::string, TimeTicks, std::less<>> UserActionTimesMap;

  // The callback that is notified when a user actions occurs.
  void OnUserAction(const std::string& user_action, TimeTicks action_time);

  // A map that tracks the times when a user action has occurred.
  UserActionTimesMap times_map_;

  // A test task runner used by user metrics.
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // The callback that is added to the global action callback list.
  base::ActionCallback action_callback_;
};

}  // namespace base

#endif  // BASE_TEST_METRICS_USER_ACTION_TESTER_H_
