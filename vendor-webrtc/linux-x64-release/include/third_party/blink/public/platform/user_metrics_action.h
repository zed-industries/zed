// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_USER_METRICS_ACTION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_USER_METRICS_ACTION_H_

namespace blink {

// WebKit equivalent to base::UserMetricsAction.  Included here so that it's
// self-contained within WebKit.
class UserMetricsAction {
 public:
  UserMetricsAction(const UserMetricsAction&) = delete;
  UserMetricsAction& operator=(const UserMetricsAction&) = delete;

  explicit UserMetricsAction(const char* action) : action_(action) {}
  const char* Action() const { return action_; }

 private:
  const char* const action_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_USER_METRICS_ACTION_H_
