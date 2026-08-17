// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_PRIORITY_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_PRIORITY_CHANGE_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_task_priority.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class TaskPriorityChangeEventInit;

class CORE_EXPORT TaskPriorityChangeEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TaskPriorityChangeEvent* Create(const AtomicString& type,
                                         const TaskPriorityChangeEventInit*);

  TaskPriorityChangeEvent(const AtomicString& type,
                          const TaskPriorityChangeEventInit*);

  TaskPriorityChangeEvent(const TaskPriorityChangeEvent&) = delete;
  TaskPriorityChangeEvent& operator=(const TaskPriorityChangeEvent&) = delete;

  ~TaskPriorityChangeEvent() override;

  const AtomicString& InterfaceName() const override;

  V8TaskPriority previousPriority() const;

 private:
  const V8TaskPriority previous_priority_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_PRIORITY_CHANGE_EVENT_H_
