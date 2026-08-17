// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_WINDOW_IDLE_TASKS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_WINDOW_IDLE_TASKS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {
class IdleRequestOptions;
class LocalDOMWindow;
class V8IdleRequestCallback;

class CORE_EXPORT WindowIdleTasks {
  STATIC_ONLY(WindowIdleTasks);

 public:
  static int requestIdleCallback(LocalDOMWindow&,
                                 V8IdleRequestCallback*,
                                 const IdleRequestOptions*);
  static void cancelIdleCallback(LocalDOMWindow&, int id);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_WINDOW_IDLE_TASKS_H_
