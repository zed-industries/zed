/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_EVENT_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_EVENT_H_

#include <cstdint>
#include <cstring>
#include <optional>

#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/containers/string_pool.h"

namespace perfetto ::trace_processor {

struct alignas(8) AndroidLogEvent {
  enum class Format : int32_t {
    kPersistentLog,
    kBugreport,
  };

  static std::optional<Format> DetectFormat(base::StringView line);
  static bool IsAndroidLogcat(const uint8_t* data, size_t size);

  bool operator==(const AndroidLogEvent& o) const {
    return pid == o.pid && tid == o.tid && prio == o.prio && tag == o.tag &&
           msg == o.msg;
  }

  uint32_t pid;
  uint32_t tid;
  uint32_t prio;  // Refer to enum ::protos::pbzero::AndroidLogPriority.
  StringPool::Id tag;
  StringPool::Id msg;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_EVENT_H_
