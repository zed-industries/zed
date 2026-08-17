// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_USER_METRICS_H_
#define BASE_METRICS_USER_METRICS_H_

#include <string>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/metrics/user_metrics_action.h"
#include "base/task/single_thread_task_runner.h"

namespace base {

class TimeTicks;

// This module provides some helper functions for logging actions tracked by
// the user metrics system.

// For best practices on deciding when to emit a user action, see
// https://chromium.googlesource.com/chromium/src.git/+/HEAD/tools/metrics/actions/README.md

// Record that the user performed an action.
// This function must be called after the task runner has been set with
// SetRecordActionTaskRunner().
//
// "Action" here means a user-generated event:
//   good: "Reload", "CloseTab", and "IMEInvoked"
//   not good: "SSLDialogShown", "PageLoaded", "DiskFull"
// We use this to gather anonymized information about how users are
// interacting with the browser.
// WARNING: In calls to this function, UserMetricsAction should be followed by a
// string literal parameter and not a variable e.g.
//   RecordAction(UserMetricsAction("my action name"));
// This ensures that our processing scripts can associate this action's hash
// with its metric name. Therefore, it will be possible to retrieve the metric
// name from the hash later on.
//
// Once a new recorded action is added, run
//   tools/metrics/actions/extract_actions.py
// to add the metric to actions.xml, then update the <owner>s and <description>
// sections. Make sure to include the actions.xml file when you upload your code
// for review!
//
// For more complicated situations (like when there are many different
// possible actions), see RecordComputedAction().
BASE_EXPORT void RecordAction(const UserMetricsAction& action);

// This function has identical input and behavior to RecordAction(), but is
// not automatically found by the action-processing scripts.  It can be used
// when it's a pain to enumerate all possible actions, but if you use this
// you need to also update the rules for extracting known actions in
// tools/metrics/actions/extract_actions.py.
// This function must be called after the task runner has been set with
// SetRecordActionTaskRunner().
BASE_EXPORT void RecordComputedAction(const std::string& action);

// Similar to RecordComputedAction, but also takes the time at which the action
// was observed.
BASE_EXPORT void RecordComputedActionAt(const std::string& action,
                                        TimeTicks action_time);

// Similar to RecordComputedActionAt, but takes the amount of time elasped since
// the action was observed.
BASE_EXPORT void RecordComputedActionSince(const std::string& action,
                                           TimeDelta time_since);

// Called with the action string.
using ActionCallback = RepeatingCallback<void(const std::string&, TimeTicks)>;

// Add/remove action callbacks (see above).
// These functions must be called after the task runner has been set with
// SetRecordActionTaskRunner().
BASE_EXPORT void AddActionCallback(const ActionCallback& callback);
BASE_EXPORT void RemoveActionCallback(const ActionCallback& callback);

// Set the task runner on which to record actions.
BASE_EXPORT void SetRecordActionTaskRunner(
    scoped_refptr<SingleThreadTaskRunner> task_runner);

// Returns the task runner used to record actions. Returns null when not set.
// This function is thread safe.
BASE_EXPORT scoped_refptr<SingleThreadTaskRunner> GetRecordActionTaskRunner();

}  // namespace base

#endif  // BASE_METRICS_USER_METRICS_H_
