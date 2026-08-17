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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_EVENT_PARSER_IMPL_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_EVENT_PARSER_IMPL_H_

#include <cstdint>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "src/trace_processor/importers/android_bugreport/android_dumpstate_event.h"
#include "src/trace_processor/importers/common/trace_parser.h"

namespace perfetto ::trace_processor {

class TraceProcessorContext;

class AndroidDumpstateEventParserImpl : public AndroidDumpstateEventParser {
 public:
  explicit AndroidDumpstateEventParserImpl(TraceProcessorContext* context)
      : context_(context) {}
  ~AndroidDumpstateEventParserImpl() override;

  void ParseAndroidDumpstateEvent(int64_t, AndroidDumpstateEvent) override;

 private:
  TraceProcessorContext* const context_;

  struct TokenizedBatteryStatsHistoryItem {
    // absolute timestamp of the event.
    int64_t ts;
    // in the event "+w=123" prefix would hold "+""
    base::StringView prefix;
    // in the event "+w=123" key would hold "w"
    base::StringView key;
    // in the event "+w=123" value would hold "123"
    base::StringView value;
  };

  base::Status ProcessBatteryStatsHistoryItem(int64_t ts,
                                              const std::string& raw_event);
  base::StatusOr<bool> ProcessBatteryStatsHistoryEvent(
      const TokenizedBatteryStatsHistoryItem& item);
  base::StatusOr<bool> ProcessBatteryStatsHistoryState(
      const TokenizedBatteryStatsHistoryItem& item);
  base::StatusOr<bool> ProcessBatteryStatsHistoryBatteryCounter(
      const TokenizedBatteryStatsHistoryItem& item);
  base::StatusOr<bool> ProcessBatteryStatsHistoryWakeLocks(
      const TokenizedBatteryStatsHistoryItem& item);
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_EVENT_PARSER_IMPL_H_
