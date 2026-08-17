// Copyright 2021 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_UTIL_TIME_H
#define GRPC_SRC_CORE_UTIL_TIME_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stdint.h>

#include <limits>
#include <optional>
#include <ostream>
#include <string>

#include "src/core/util/time_precise.h"
#include "src/core/util/useful.h"

#define GRPC_LOG_EVERY_N_SEC_DELAYED_DEBUG(n, format, ...)      \
  do {                                                          \
    static std::atomic<uint64_t> prev{0};                       \
    uint64_t now = grpc_core::Timestamp::FromTimespecRoundDown( \
                       gpr_now(GPR_CLOCK_MONOTONIC))            \
                       .milliseconds_after_process_epoch();     \
    if (prev == 0) prev = now;                                  \
    if (now - prev > (n) * 1000) {                              \
      prev = now;                                               \
      VLOG(2) << absl::StrFormat(format, __VA_ARGS__);          \
    }                                                           \
  } while (0)

namespace grpc_core {

namespace time_detail {

inline int64_t MillisAdd(int64_t a, int64_t b) {
  if (a == std::numeric_limits<int64_t>::max() ||
      b == std::numeric_limits<int64_t>::max()) {
    return std::numeric_limits<int64_t>::max();
  }
  if (a == std::numeric_limits<int64_t>::min() ||
      b == std::numeric_limits<int64_t>::min()) {
    return std::numeric_limits<int64_t>::min();
  }
  return SaturatingAdd(a, b);
}

constexpr inline int64_t MillisMul(int64_t millis, int64_t mul) {
  return millis >= std::numeric_limits<int64_t>::max() / mul
             ? std::numeric_limits<int64_t>::max()
         : millis <= std::numeric_limits<int64_t>::min() / mul
             ? std::numeric_limits<int64_t>::min()
             : millis * mul;
}

}  // namespace time_detail

class Duration;

// Timestamp represents a discrete point in time.
class Timestamp {
 public:
  // Base interface for time providers.
  class Source {
   public:
    // Return the current time.
    virtual Timestamp Now() = 0;
    virtual void InvalidateCache() {}

   protected:
    // We don't delete through this interface, so non-virtual dtor is fine.
    ~Source() = default;
  };

  class ScopedSource : public Source {
   public:
    ScopedSource() : previous_(thread_local_time_source_) {
      thread_local_time_source_ = this;
    }
    ScopedSource(const ScopedSource&) = delete;
    ScopedSource& operator=(const ScopedSource&) = delete;
    void InvalidateCache() override { previous_->InvalidateCache(); }

   protected:
    ~ScopedSource() { thread_local_time_source_ = previous_; }
    Source* previous() const { return previous_; }

   private:
    Source* const previous_;
  };

  constexpr Timestamp() = default;
  // Constructs a Timestamp from a gpr_timespec.
  static Timestamp FromTimespecRoundDown(gpr_timespec t);
  static Timestamp FromTimespecRoundUp(gpr_timespec t);

  // Construct a Timestamp from a gpr_cycle_counter.
  static Timestamp FromCycleCounterRoundUp(gpr_cycle_counter c);
  static Timestamp FromCycleCounterRoundDown(gpr_cycle_counter c);

  static Timestamp Now() { return thread_local_time_source_->Now(); }

  static constexpr Timestamp FromMillisecondsAfterProcessEpoch(int64_t millis) {
    return Timestamp(millis);
  }

  static constexpr Timestamp ProcessEpoch() { return Timestamp(0); }

  static constexpr Timestamp InfFuture() {
    return Timestamp(std::numeric_limits<int64_t>::max());
  }

  static constexpr Timestamp InfPast() {
    return Timestamp(std::numeric_limits<int64_t>::min());
  }

  constexpr bool operator==(Timestamp other) const {
    return millis_ == other.millis_;
  }
  constexpr bool operator!=(Timestamp other) const {
    return millis_ != other.millis_;
  }
  constexpr bool operator<(Timestamp other) const {
    return millis_ < other.millis_;
  }
  constexpr bool operator<=(Timestamp other) const {
    return millis_ <= other.millis_;
  }
  constexpr bool operator>(Timestamp other) const {
    return millis_ > other.millis_;
  }
  constexpr bool operator>=(Timestamp other) const {
    return millis_ >= other.millis_;
  }
  Timestamp& operator+=(Duration duration);

  bool is_process_epoch() const { return millis_ == 0; }

  uint64_t milliseconds_after_process_epoch() const { return millis_; }

  gpr_timespec as_timespec(gpr_clock_type type) const;

  std::string ToString() const;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const Timestamp& t) {
    sink.Append(t.ToString());
  }

 private:
  explicit constexpr Timestamp(int64_t millis) : millis_(millis) {}

  int64_t millis_ = 0;
  static thread_local Timestamp::Source* thread_local_time_source_;
};

class ScopedTimeCache final : public Timestamp::ScopedSource {
 public:
  Timestamp Now() override;

  void InvalidateCache() override {
    cached_time_ = std::nullopt;
    Timestamp::ScopedSource::InvalidateCache();
  }
  void TestOnlySetNow(Timestamp now) { cached_time_ = now; }

 private:
  std::optional<Timestamp> cached_time_;
};

// Duration represents a span of time.
class Duration {
 public:
  constexpr Duration() noexcept : millis_(0) {}

  static Duration FromTimespec(gpr_timespec t);
  static Duration FromSecondsAndNanoseconds(int64_t seconds, int32_t nanos);
  static Duration FromSecondsAsDouble(double seconds);

  static constexpr Duration Zero() { return Duration(0); }

  // Smallest representatable positive duration.
  static constexpr Duration Epsilon() { return Duration(1); }

  static constexpr Duration NegativeInfinity() {
    return Duration(std::numeric_limits<int64_t>::min());
  }

  static constexpr Duration Infinity() {
    return Duration(std::numeric_limits<int64_t>::max());
  }

  static constexpr Duration Hours(int64_t hours) {
    return Minutes(time_detail::MillisMul(hours, 60));
  }

  static constexpr Duration Minutes(int64_t minutes) {
    return Seconds(time_detail::MillisMul(minutes, 60));
  }

  static constexpr Duration Seconds(int64_t seconds) {
    return Milliseconds(time_detail::MillisMul(seconds, GPR_MS_PER_SEC));
  }

  static constexpr Duration Milliseconds(int64_t millis) {
    return Duration(millis);
  }

  static constexpr Duration MicrosecondsRoundDown(int64_t micros) {
    return Duration(micros / GPR_US_PER_MS);
  }

  static constexpr Duration NanosecondsRoundDown(int64_t nanos) {
    return Duration(nanos / GPR_NS_PER_MS);
  }

  static constexpr Duration MicrosecondsRoundUp(int64_t micros) {
    return Duration((micros / GPR_US_PER_MS) + (micros % GPR_US_PER_MS != 0));
  }

  static constexpr Duration NanosecondsRoundUp(int64_t nanos) {
    return Duration((nanos / GPR_NS_PER_MS) + (nanos % GPR_NS_PER_MS != 0));
  }

  constexpr bool operator==(Duration other) const {
    return millis_ == other.millis_;
  }
  constexpr bool operator!=(Duration other) const {
    return millis_ != other.millis_;
  }
  constexpr bool operator<(Duration other) const {
    return millis_ < other.millis_;
  }
  constexpr bool operator<=(Duration other) const {
    return millis_ <= other.millis_;
  }
  constexpr bool operator>(Duration other) const {
    return millis_ > other.millis_;
  }
  constexpr bool operator>=(Duration other) const {
    return millis_ >= other.millis_;
  }
  Duration& operator/=(int64_t divisor) {
    if (millis_ == std::numeric_limits<int64_t>::max()) {
      *this = divisor < 0 ? NegativeInfinity() : Infinity();
    } else if (millis_ == std::numeric_limits<int64_t>::min()) {
      *this = divisor < 0 ? Infinity() : NegativeInfinity();
    } else {
      millis_ /= divisor;
    }
    return *this;
  }
  Duration& operator*=(double multiplier);
  Duration& operator+=(Duration other) {
    millis_ += other.millis_;
    return *this;
  }

  constexpr int64_t millis() const { return millis_; }
  double seconds() const { return static_cast<double>(millis_) / 1000.0; }

  // NOLINTNEXTLINE: google-explicit-constructor
  operator grpc_event_engine::experimental::EventEngine::Duration() const;

  gpr_timespec as_timespec() const;

  std::string ToString() const;

  // Returns the duration in the JSON form corresponding to a
  // google.protobuf.Duration proto, as defined here:
  // https://developers.google.com/protocol-buffers/docs/proto3#json
  std::string ToJsonString() const;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const Duration& t) {
    sink.Append(t.ToString());
  }

 private:
  explicit constexpr Duration(int64_t millis) : millis_(millis) {}

  int64_t millis_;
};

inline std::ostream& operator<<(std::ostream& out, const Duration& d) {
  return out << d.ToString();
}

inline std::ostream& operator<<(std::ostream& out, const Timestamp& d) {
  return out << d.ToString();
}

inline Duration operator+(Duration lhs, Duration rhs) {
  return Duration::Milliseconds(
      time_detail::MillisAdd(lhs.millis(), rhs.millis()));
}

inline Duration operator-(Duration lhs, Duration rhs) {
  return Duration::Milliseconds(
      time_detail::MillisAdd(lhs.millis(), -rhs.millis()));
}

inline Timestamp operator+(Timestamp lhs, Duration rhs) {
  return Timestamp::FromMillisecondsAfterProcessEpoch(time_detail::MillisAdd(
      lhs.milliseconds_after_process_epoch(), rhs.millis()));
}

inline Timestamp operator-(Timestamp lhs, Duration rhs) {
  return Timestamp::FromMillisecondsAfterProcessEpoch(time_detail::MillisAdd(
      lhs.milliseconds_after_process_epoch(), -rhs.millis()));
}

inline Timestamp operator+(Duration lhs, Timestamp rhs) { return rhs + lhs; }

inline Duration operator-(Timestamp lhs, Timestamp rhs) {
  if (rhs == Timestamp::InfPast() && lhs != Timestamp::InfPast()) {
    return Duration::Infinity();
  }
  if (rhs == Timestamp::InfFuture() && lhs != Timestamp::InfFuture()) {
    return Duration::NegativeInfinity();
  }
  return Duration::Milliseconds(
      time_detail::MillisAdd(lhs.milliseconds_after_process_epoch(),
                             -rhs.milliseconds_after_process_epoch()));
}

inline Duration operator*(Duration lhs, double rhs) {
  if (lhs == Duration::Infinity()) {
    return rhs < 0 ? Duration::NegativeInfinity() : Duration::Infinity();
  }
  if (lhs == Duration::NegativeInfinity()) {
    return rhs < 0 ? Duration::Infinity() : Duration::NegativeInfinity();
  }
  return Duration::FromSecondsAsDouble(lhs.millis() * rhs / 1000.0);
}

inline Duration operator*(double lhs, Duration rhs) { return rhs * lhs; }

inline Duration operator/(Duration lhs, int64_t rhs) {
  lhs /= rhs;
  return lhs;
}

inline Duration Duration::FromSecondsAndNanoseconds(int64_t seconds,
                                                    int32_t nanos) {
  return Seconds(seconds) + NanosecondsRoundDown(nanos);
}

inline Duration Duration::FromSecondsAsDouble(double seconds) {
  double millis = seconds * 1000.0;
  if (millis >= static_cast<double>(std::numeric_limits<int64_t>::max())) {
    return Infinity();
  }
  if (millis <= static_cast<double>(std::numeric_limits<int64_t>::min())) {
    return NegativeInfinity();
  }
  return Milliseconds(static_cast<int64_t>(millis));
}

inline Duration& Duration::operator*=(double multiplier) {
  *this = *this * multiplier;
  return *this;
}

inline Timestamp& Timestamp::operator+=(Duration duration) {
  return *this = (*this + duration);
}

void TestOnlySetProcessEpoch(gpr_timespec epoch);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_TIME_H
