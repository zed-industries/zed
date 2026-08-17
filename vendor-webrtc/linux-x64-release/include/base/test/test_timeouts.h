// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_TIMEOUTS_H_
#define BASE_TEST_TEST_TIMEOUTS_H_

#include "base/check.h"
#include "base/time/time.h"

// Returns common timeouts to use in tests. Makes it possible to adjust
// the timeouts for different environments (like TSan).
class TestTimeouts {
 public:
  TestTimeouts() = delete;
  TestTimeouts(const TestTimeouts&) = delete;
  TestTimeouts& operator=(const TestTimeouts&) = delete;

  // Initializes the timeouts. Non thread-safe. Should be called exactly once
  // by the test suite.
  static void Initialize();

  // Timeout for actions that are expected to finish "almost instantly".  This
  // is used in various tests to post delayed tasks and usually functions more
  // like a delay value than a timeout.
  static base::TimeDelta tiny_timeout() {
    DCHECK(initialized_);
    return tiny_timeout_;
  }

  // Timeout to wait for something to happen. If you are not sure
  // which timeout to use, this is the one you want.
  static base::TimeDelta action_timeout() {
    DCHECK(initialized_);
    return action_timeout_;
  }

  // Timeout longer than the above, suitable to wait on success conditions which
  // can take a while to achieve but still should expire on failure before
  // |test_launcher_timeout()| terminates the process. Note that
  // test_launcher_timeout() can be reached nonetheless when multiple such
  // actions are compounded in the same test.
  static base::TimeDelta action_max_timeout() {
    DCHECK(initialized_);
    return action_max_timeout_;
  }

  // Timeout for a single test launched used built-in test launcher.
  // Do not use outside of the test launcher.
  static base::TimeDelta test_launcher_timeout() {
    DCHECK(initialized_);
    return test_launcher_timeout_;
  }

 private:
  static bool initialized_;

  static base::TimeDelta tiny_timeout_;
  static base::TimeDelta action_timeout_;
  static base::TimeDelta action_max_timeout_;
  static base::TimeDelta test_launcher_timeout_;
};

#endif  // BASE_TEST_TEST_TIMEOUTS_H_
