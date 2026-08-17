/*
 * Copyright (C) 2015 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_IDLE_TASK_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_IDLE_TASK_RUNNER_H_

#include <memory>

#include "base/location.h"
#include "base/memory/ptr_util.h"
#include "gin/public/v8_idle_task_runner.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier_std.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_functional.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class V8IdleTaskRunner : public gin::V8IdleTaskRunner {
  USING_FAST_MALLOC(V8IdleTaskRunner);

 public:
  explicit V8IdleTaskRunner(ThreadScheduler* scheduler)
      : scheduler_(scheduler) {}

  V8IdleTaskRunner(const V8IdleTaskRunner&) = delete;
  V8IdleTaskRunner& operator=(const V8IdleTaskRunner&) = delete;

  ~V8IdleTaskRunner() override = default;
  void PostIdleTask(std::unique_ptr<v8::IdleTask> task) override {
    DCHECK(RuntimeEnabledFeatures::V8IdleTasksEnabled());
    scheduler_->PostIdleTask(
        FROM_HERE,
        ConvertToBaseOnceCallback(WTF::CrossThreadBindOnce(
            [](std::unique_ptr<v8::IdleTask> task, base::TimeTicks deadline) {
              task->Run(deadline.since_origin().InSecondsF());
            },
            std::move(task))));
  }

 private:
  ThreadScheduler* scheduler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_IDLE_TASK_RUNNER_H_
