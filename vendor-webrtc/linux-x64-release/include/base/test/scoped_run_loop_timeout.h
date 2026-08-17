// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_RUN_LOOP_TIMEOUT_H_
#define BASE_TEST_SCOPED_RUN_LOOP_TIMEOUT_H_

#include <optional>
#include <string>

#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/run_loop.h"
#include "base/time/time.h"

namespace content {
FORWARD_DECLARE_TEST(ContentBrowserTest, RunTimeoutInstalled);
}

namespace base::test {

FORWARD_DECLARE_TEST(TaskEnvironmentTest, SetsDefaultRunTimeout);

// Configures all RunLoop::Run() calls on the current thread to run the
// supplied |on_timeout| callback if they run for longer than |timeout|.
//
// Specifying Run() timeouts per-thread avoids the need to cope with Run()s
// executing concurrently with ScopedRunLoopTimeout initialization or
// teardown, and allows "default" timeouts to be specified by suites, rather
// than explicitly configuring them for every RunLoop, in each test.
//
// This is used by test classes including TaskEnvironment and TestSuite to
// set a default Run() timeout on the main thread of all tests which use them.
//
// Tests which have steps which need to Run() for longer than their suite's
// default (if any) allows can override the active timeout by creating a nested
// ScopedRunLoopTimeout on their stack, e.g:
//
//   ScopedRunLoopTimeout default_timeout(kDefaultRunTimeout);
//   ... do other test stuff ...
//   RunLoop().Run(); // Run for up to kDefaultRunTimeout.
//   ...
//   {
//     ScopedRunLoopTimeout specific_timeout(kTestSpecificTimeout);
//     RunLoop().Run(); // Run for up to kTestSpecificTimeout.
//   }
//   ...
//   RunLoop().Run(); // Run for up to kDefaultRunTimeout.
//
// The currently-active timeout can also be temporarily disabled:
//   ScopedDisableRunLoopTimeout disable_timeout;
//
// By default LOG(FATAL) will be invoked on Run() timeout. Test binaries
// can opt-in to using ADD_FAILURE() instead by calling
// SetAddGTestFailureOnTimeout() during process initialization.
//
// TaskEnvironment applies a default Run() timeout.

class ScopedRunLoopTimeout {
 public:
  // This callback is the one called upon run loop timeouts.
  // RunLoop inner mechanism will call this callback after having quit the run
  // loop. Implementer might chose to log locations, crash the process, dump a
  // stack trace, depending on the desired behaviour for run loop timeouts.
  // Invoking `on_timeout_log` might return a personalized timeouts message
  // string. This callback was sent at ScopedRunLoopTimeout creation. Invoking
  // this callback is not mandatory, as it depends on the desired behaviour of
  // this function.
  using TimeoutCallback = base::RepeatingCallback<void(
      const Location& timeout_enabled_from_here,
      RepeatingCallback<std::string()> on_timeout_log,
      const Location& run_from_here)>;

  ScopedRunLoopTimeout(const Location& timeout_enabled_from_here,
                       TimeDelta timeout);
  ~ScopedRunLoopTimeout();

  // Invokes |on_timeout_log| if |timeout| expires, and appends it to the
  // logged error message. If `timeout` is not specified the current timeout is
  // used and only the log message is overridden.
  ScopedRunLoopTimeout(const Location& timeout_enabled_from_here,
                       std::optional<TimeDelta> timeout,
                       RepeatingCallback<std::string()> on_timeout_log);

  ScopedRunLoopTimeout(const ScopedRunLoopTimeout&) = delete;
  ScopedRunLoopTimeout& operator=(const ScopedRunLoopTimeout&) = delete;

  // Returns true if there is a Run() timeout configured on the current thread.
  static bool ExistsForCurrentThread();

  // Important note:
  // The two following static methods will alter the behaviour on run loop
  // timeouts. If both methods are being called (whatever the ordering), the
  // behaviour will be chained, which means that both callbacks will be invoked.
  // If the custom callback handling is reset (`SetTimeoutCallbackForTesting`
  // called with `nullptr`), then we reset the behaviour to its previous state,
  // which is, if `SetAddGTestFailureOnTimeout`, it will invoke GTest timeout
  // handling. Otherwise, it will invoke the default function.

  // Add GTest timeout handler.
  static void SetAddGTestFailureOnTimeout();

  // Add provided callback as timeout handler.
  static void SetTimeoutCallbackForTesting(std::unique_ptr<TimeoutCallback> cb);

 private:
  TimeoutCallback GetTimeoutCallback();

 protected:
  FRIEND_TEST_ALL_PREFIXES(ScopedRunLoopRunTimeoutTest, TimesOut);
  FRIEND_TEST_ALL_PREFIXES(ScopedRunLoopRunTimeoutTest, RunTasksUntilTimeout);
  FRIEND_TEST_ALL_PREFIXES(TaskEnvironmentTest, SetsDefaultRunTimeout);
  FRIEND_TEST_ALL_PREFIXES(content::ContentBrowserTest, RunTimeoutInstalled);

  // Exposes the RunLoopTimeout to the friend tests (see above).
  static const RunLoop::RunLoopTimeout* GetTimeoutForCurrentThread();

  raw_ptr<const RunLoop::RunLoopTimeout> const nested_timeout_;
  RunLoop::RunLoopTimeout run_timeout_;
};

class ScopedDisableRunLoopTimeout {
 public:
  ScopedDisableRunLoopTimeout();
  ~ScopedDisableRunLoopTimeout();

  ScopedDisableRunLoopTimeout(const ScopedDisableRunLoopTimeout&) = delete;
  ScopedDisableRunLoopTimeout& operator=(const ScopedDisableRunLoopTimeout&) =
      delete;

 private:
  const raw_ptr<const RunLoop::RunLoopTimeout> nested_timeout_;
};

}  // namespace base::test

#endif  // BASE_TEST_SCOPED_RUN_LOOP_TIMEOUT_H_
