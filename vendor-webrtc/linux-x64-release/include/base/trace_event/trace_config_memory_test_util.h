// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACE_CONFIG_MEMORY_TEST_UTIL_H_
#define BASE_TRACE_EVENT_TRACE_CONFIG_MEMORY_TEST_UTIL_H_

#include "base/strings/stringprintf.h"
#include "base/trace_event/memory_dump_manager.h"

namespace base {
namespace trace_event {

class TraceConfigMemoryTestUtil {
 public:
  static std::string GetTraceConfig_LegacyPeriodicTriggers(int light_period,
                                                           int heavy_period) {
    return StringPrintf(
        "{"
        "\"enable_argument_filter\":false,"
        "\"enable_package_name_filter\":false,"
        "\"enable_systrace\":false,"
        "\"excluded_categories\":["
        "\"*\""
        "],"
        "\"included_categories\":["
        "\"%s\""
        "],"
        "\"memory_dump_config\":{"
        "\"allowed_dump_modes\":[\"background\",\"light\",\"detailed\"],"
        "\"heap_profiler_options\":{"
        "\"breakdown_threshold_bytes\":2048"
        "},"
        "\"triggers\":["
        "{"
        "\"mode\":\"light\","
        "\"periodic_interval_ms\":%d"
        "},"
        "{"
        "\"mode\":\"detailed\","
        "\"periodic_interval_ms\":%d"
        "}"
        "]"
        "},"
        "\"record_mode\":\"record-until-full\""
        "}",
        MemoryDumpManager::kTraceCategory, light_period, heavy_period);
  }

  static std::string GetTraceConfig_PeriodicTriggers(int light_period,
                                                     int heavy_period) {
    return StringPrintf(
        "{"
        "\"enable_argument_filter\":false,"
        "\"enable_package_name_filter\":false,"
        "\"enable_systrace\":false,"
        "\"excluded_categories\":["
        "\"*\""
        "],"
        "\"included_categories\":["
        "\"%s\""
        "],"
        "\"memory_dump_config\":{"
        "\"allowed_dump_modes\":[\"background\",\"light\",\"detailed\"],"
        "\"heap_profiler_options\":{"
        "\"breakdown_threshold_bytes\":2048"
        "},"
        "\"triggers\":["
        "{"
        "\"min_time_between_dumps_ms\":%d,"
        "\"mode\":\"light\","
        "\"type\":\"periodic_interval\""
        "},"
        "{"
        "\"min_time_between_dumps_ms\":%d,"
        "\"mode\":\"detailed\","
        "\"type\":\"periodic_interval\""
        "}"
        "]"
        "},"
        "\"record_mode\":\"record-until-full\""
        "}",
        MemoryDumpManager::kTraceCategory, light_period, heavy_period);
  }

  static std::string GetTraceConfig_EmptyTriggers() {
    return StringPrintf(
        "{"
        "\"enable_argument_filter\":false,"
        "\"enable_package_name_filter\":false,"
        "\"enable_systrace\":false,"
        "\"excluded_categories\":["
        "\"*\""
        "],"
        "\"included_categories\":["
        "\"%s\""
        "],"
        "\"memory_dump_config\":{"
        "\"allowed_dump_modes\":[\"background\",\"light\",\"detailed\"],"
        "\"triggers\":["
        "]"
        "},"
        "\"record_mode\":\"record-until-full\""
        "}",
        MemoryDumpManager::kTraceCategory);
  }

  static std::string GetTraceConfig_NoTriggers() {
    return StringPrintf(
        "{"
        "\"enable_argument_filter\":false,"
        "\"enable_systrace\":false,"
        "\"excluded_categories\":["
        "\"*\""
        "],"
        "\"included_categories\":["
        "\"%s\""
        "],"
        "\"record_mode\":\"record-until-full\""
        "}",
        MemoryDumpManager::kTraceCategory);
  }

  static std::string GetTraceConfig_BackgroundTrigger(int period_ms) {
    return StringPrintf(
        "{"
        "\"enable_argument_filter\":false,"
        "\"enable_package_name_filter\":false,"
        "\"enable_systrace\":false,"
        "\"excluded_categories\":["
        "\"*\""
        "],"
        "\"included_categories\":["
        "\"%s\""
        "],"
        "\"memory_dump_config\":{"
        "\"allowed_dump_modes\":[\"background\"],"
        "\"triggers\":["
        "{"
        "\"min_time_between_dumps_ms\":%d,"
        "\"mode\":\"background\","
        "\"type\":\"periodic_interval\""
        "}"
        "]"
        "},"
        "\"record_mode\":\"record-until-full\""
        "}",
        MemoryDumpManager::kTraceCategory, period_ms);
  }
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_TRACE_CONFIG_MEMORY_TEST_UTIL_H_
