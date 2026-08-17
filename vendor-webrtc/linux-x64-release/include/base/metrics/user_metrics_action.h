// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_USER_METRICS_ACTION_H_
#define BASE_METRICS_USER_METRICS_ACTION_H_

namespace base {

// UserMetricsAction exists purely to standardize on the parameters passed to
// UserMetrics. That way, our toolset can scan the source code reliable for
// constructors and extract the associated string constants.
// WARNING: When using UserMetricsAction you should use a string literal
// parameter e.g.
//   RecordAction(UserMetricsAction("my action name"));
// This ensures that our processing scripts can associate this action's hash
// with its metric name. Therefore, it will be possible to retrieve the metric
// name from the hash later on.
// Please see tools/metrics/actions/extract_actions.py for details.
struct UserMetricsAction {
  const char* str_;
  explicit constexpr UserMetricsAction(const char* str) noexcept : str_(str) {}
};

}  // namespace base

#endif  // BASE_METRICS_USER_METRICS_ACTION_H_
