// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_COMMON_LAZY_NOW_H_
#define BASE_TASK_COMMON_LAZY_NOW_H_

#include <optional>

#include "base/base_export.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/time/time.h"

namespace base {

class TickClock;

// Now() is somewhat expensive so it makes sense not to call Now() unless we
// really need to and to avoid subsequent calls if already called once.
// LazyNow objects are expected to be short-living to represent accurate time.
class BASE_EXPORT LazyNow {
 public:
  explicit LazyNow(TimeTicks now);
  explicit LazyNow(std::optional<TimeTicks> now, const TickClock* tick_clock);
  explicit LazyNow(const TickClock* tick_clock);
  LazyNow(const LazyNow&) = delete;
  LazyNow& operator=(const LazyNow&) = delete;

  LazyNow(LazyNow&& move_from) noexcept;

  // Result will not be updated on any subsesequent calls.
  TimeTicks Now();

  bool has_value() const { return !!now_; }

 private:
  std::optional<TimeTicks> now_;
  // RAW_PTR_EXCLUSION: The pointee doesn't need UaF protection (it has the same
  // lifetime as the thread/sequence).
  RAW_PTR_EXCLUSION const TickClock* tick_clock_;  // Not owned.
};

}  // namespace base

#endif  // BASE_TASK_COMMON_LAZY_NOW_H_
