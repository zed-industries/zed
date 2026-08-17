// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_TASK_TYPE_NAMES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_TASK_TYPE_NAMES_H_

#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/perfetto/include/perfetto/tracing/string_helpers.h"

namespace blink {
namespace scheduler {

class PLATFORM_EXPORT TaskTypeNames {
  STATIC_ONLY(TaskTypeNames);

 public:
  static perfetto::StaticString TaskTypeToString(TaskType task_type);
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_TASK_TYPE_NAMES_H_
