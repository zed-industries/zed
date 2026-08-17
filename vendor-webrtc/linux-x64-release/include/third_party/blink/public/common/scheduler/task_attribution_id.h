// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SCHEDULER_TASK_ATTRIBUTION_ID_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SCHEDULER_TASK_ATTRIBUTION_ID_H_

#include <compare>
#include <cstdint>

namespace blink::scheduler {

using TaskAttributionIdType = uint32_t;

// TaskAttributionId represents the ID of a task scope, encompassing a task and
// its continuations. It enables comparison and incrementation operations on it,
// while abstracting the underlying value from callers.
class TaskAttributionId {
 public:
  TaskAttributionId() = default;
  explicit TaskAttributionId(TaskAttributionIdType value) : value_(value) {}
  TaskAttributionId(const TaskAttributionId&) = default;
  TaskAttributionId& operator=(const TaskAttributionId&) = default;
  TaskAttributionIdType value() const { return value_; }

  bool operator==(const TaskAttributionId& id) const {
    return value_ == id.value_;
  }
  std::strong_ordering operator<=>(const TaskAttributionId& id) const {
    return value_ <=> id.value_;
  }
  TaskAttributionId NextId() const { return TaskAttributionId(value_ + 1); }

 private:
  TaskAttributionIdType value_ = {0};
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SCHEDULER_TASK_ATTRIBUTION_ID_H_
