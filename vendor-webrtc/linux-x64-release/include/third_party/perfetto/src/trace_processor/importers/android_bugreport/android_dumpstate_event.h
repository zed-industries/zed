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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_EVENT_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_EVENT_H_

#include <cstdint>
#include <cstring>
#include <optional>

#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/containers/string_pool.h"

namespace perfetto ::trace_processor {

struct alignas(8) AndroidDumpstateEvent {
  enum class EventType : int32_t {
    kNull,
    // A battery stats history event, given in the checkin format.
    // eg. "+w=2" or "+ws" or "Ejb=15" or "Pst=off"
    kBatteryStatsHistoryEvent,
  };

  EventType type;

  // The raw event, the contents of which depends on the event type.
  std::string raw_event;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_EVENT_H_
