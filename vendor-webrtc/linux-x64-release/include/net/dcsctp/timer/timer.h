/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_TIMER_TIMER_H_
#define NET_DCSCTP_TIMER_TIMER_H_

#include <stdint.h>

#include <algorithm>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/time_delta.h"
#include "net/dcsctp/public/timeout.h"
#include "rtc_base/strong_alias.h"

namespace dcsctp {

using TimerID = webrtc::StrongAlias<class TimerIDTag, uint32_t>;
using TimerGeneration = webrtc::StrongAlias<class TimerGenerationTag, uint32_t>;

enum class TimerBackoffAlgorithm {
  // The base duration will be used for any restart.
  kFixed,
  // An exponential backoff is used for restarts, with a 2x multiplier, meaning
  // that every restart will use a duration that is twice as long as the
  // previous.
  kExponential,
};

struct TimerOptions {
  explicit TimerOptions(webrtc::TimeDelta duration)
      : TimerOptions(duration, TimerBackoffAlgorithm::kExponential) {}
  TimerOptions(webrtc::TimeDelta duration,
               TimerBackoffAlgorithm backoff_algorithm)
      : TimerOptions(duration, backoff_algorithm, std::nullopt) {}
  TimerOptions(webrtc::TimeDelta duration,
               TimerBackoffAlgorithm backoff_algorithm,
               std::optional<int> max_restarts)
      : TimerOptions(duration,
                     backoff_algorithm,
                     max_restarts,
                     webrtc::TimeDelta::PlusInfinity()) {}
  TimerOptions(webrtc::TimeDelta duration,
               TimerBackoffAlgorithm backoff_algorithm,
               std::optional<int> max_restarts,
               webrtc::TimeDelta max_backoff_duration)
      : TimerOptions(duration,
                     backoff_algorithm,
                     max_restarts,
                     max_backoff_duration,
                     webrtc::TaskQueueBase::DelayPrecision::kLow) {}
  TimerOptions(webrtc::TimeDelta duration,
               TimerBackoffAlgorithm backoff_algorithm,
               std::optional<int> max_restarts,
               webrtc::TimeDelta max_backoff_duration,
               webrtc::TaskQueueBase::DelayPrecision precision)
      : duration(duration),
        backoff_algorithm(backoff_algorithm),
        max_restarts(max_restarts),
        max_backoff_duration(max_backoff_duration),
        precision(precision) {}

  // The initial timer duration. Can be overridden with `set_duration`.
  const webrtc::TimeDelta duration;
  // If the duration should be increased (using exponential backoff) when it is
  // restarted. If not set, the same duration will be used.
  const TimerBackoffAlgorithm backoff_algorithm;
  // The maximum number of times that the timer will be automatically restarted,
  // or std::nullopt if there is no limit.
  const std::optional<int> max_restarts;
  // The maximum timeout value for exponential backoff.
  const webrtc::TimeDelta max_backoff_duration;
  // The precision of the webrtc::TaskQueueBase used for scheduling.
  const webrtc::TaskQueueBase::DelayPrecision precision;
};

// A high-level timer (in contrast to the low-level `Timeout` class).
//
// Timers are started and can be stopped or restarted. When a timer expires,
// the provided `on_expired` callback will be triggered. A timer is
// automatically restarted, as long as the number of restarts is below the
// configurable `max_restarts` parameter. The `is_running` property can be
// queried to know if it's still running after having expired.
//
// When a timer is restarted, it will use a configurable `backoff_algorithm` to
// possibly adjust the duration of the next expiry. It is also possible to
// return a new base duration (which is the duration before it's adjusted by the
// backoff algorithm).
class Timer {
 public:
  // The maximum timer duration - one day.
  static constexpr webrtc::TimeDelta kMaxTimerDuration =
      webrtc::TimeDelta::Seconds(24 * 3600);

  // When expired, the timer handler can optionally return a new non-zero
  // duration which will be set as `duration` and used as base duration when the
  // timer is restarted and as input to the backoff algorithm. If zero is
  // returned, the current duration is used.
  using OnExpired = std::function<webrtc::TimeDelta()>;

  // TimerManager will have pointers to these instances, so they must not move.
  Timer(const Timer&) = delete;
  Timer& operator=(const Timer&) = delete;

  ~Timer();

  // Starts the timer if it's stopped or restarts the timer if it's already
  // running. The `expiration_count` will be reset.
  void Start();

  // Stops the timer. This can also be called when the timer is already stopped.
  // The `expiration_count` will be reset.
  void Stop();

  // Sets the base duration. The actual timer duration may be larger depending
  // on the backoff algorithm.
  void set_duration(webrtc::TimeDelta duration) {
    duration_ = std::min(duration, kMaxTimerDuration);
  }

  // Retrieves the base duration. The actual timer duration may be larger
  // depending on the backoff algorithm.
  webrtc::TimeDelta duration() const { return duration_; }

  // Returns the number of times the timer has expired.
  int expiration_count() const { return expiration_count_; }

  // Returns the timer's options.
  const TimerOptions& options() const { return options_; }

  // Returns the name of the timer.
  absl::string_view name() const { return name_; }

  // Indicates if this timer is currently running.
  bool is_running() const { return is_running_; }

 private:
  friend class TimerManager;
  using UnregisterHandler = std::function<void()>;
  Timer(TimerID id,
        absl::string_view name,
        OnExpired on_expired,
        UnregisterHandler unregister,
        std::unique_ptr<Timeout> timeout,
        const TimerOptions& options);

  // Called by TimerManager. Will trigger the callback and increment
  // `expiration_count`. The timer will automatically be restarted at the
  // duration as decided by the backoff algorithm, unless the
  // `TimerOptions::max_restarts` has been reached and then it will be stopped
  // and `is_running()` will return false.
  void Trigger(TimerGeneration generation);

  const TimerID id_;
  const std::string name_;
  const TimerOptions options_;
  const OnExpired on_expired_;
  const UnregisterHandler unregister_handler_;
  const std::unique_ptr<Timeout> timeout_;

  webrtc::TimeDelta duration_;

  // Increased on each start, and is matched on Trigger, to avoid races. And by
  // race, meaning that a timeout - which may be evaluated/expired on a
  // different thread while this thread has stopped that timer already. Note
  // that the entire socket is not thread-safe, so `TimerManager::HandleTimeout`
  // is never executed concurrently with any timer starting/stopping.
  //
  // This will wrap around after 4 billion timer restarts, and if it wraps
  // around, it would just trigger _this_ timer in advance (but it's hard to
  // restart it 4 billion times within its duration).
  TimerGeneration generation_ = TimerGeneration(0);
  bool is_running_ = false;
  // Incremented each time time has expired and reset when stopped or restarted.
  int expiration_count_ = 0;
};

// Creates and manages timers.
class TimerManager {
 public:
  explicit TimerManager(
      std::function<std::unique_ptr<Timeout>(
          webrtc::TaskQueueBase::DelayPrecision)> create_timeout)
      : create_timeout_(std::move(create_timeout)) {}

  // Creates a timer with name `name` that will expire (when started) after
  // `options.duration` and call `on_expired`. There are more `options` that
  // affects the behavior. Note that timers are created initially stopped.
  std::unique_ptr<Timer> CreateTimer(absl::string_view name,
                                     Timer::OnExpired on_expired,
                                     const TimerOptions& options);

  void HandleTimeout(TimeoutID timeout_id);

 private:
  const std::function<std::unique_ptr<Timeout>(
      webrtc::TaskQueueBase::DelayPrecision)>
      create_timeout_;
  std::map<TimerID, Timer*> timers_;
  TimerID next_id_ = TimerID(0);
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_TIMER_TIMER_H_
