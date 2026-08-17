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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BATTERY_STATS_HISTORY_STRING_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BATTERY_STATS_HISTORY_STRING_TRACKER_H_

#include <bitset>
#include <cstdint>
#include <optional>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

// This class is used to track the history string pool (hsp) items emitted by
// the battery stats checkin reader and consumed by AndroidDumpStateParserImpl
//
// The history string pool items are stored in a vector and not internerned in
// the trace processor storage, due to these strings needing further processing
// after being associated with a timestampped event post-sort. The final
// processed string will be interned there post-sort instead.
//
// The string pool items are added to the vector using the SetStringPoolItem
// method. The items are later retrieved using the GetUid and GetString
// methods.
class AndroidBatteryStatsHistoryStringTracker : public Destructible {
 public:
  ~AndroidBatteryStatsHistoryStringTracker() override;

  static AndroidBatteryStatsHistoryStringTracker* GetOrCreate(
      TraceProcessorContext* context) {
    if (!context->android_battery_stats_history_tracker) {
      context->android_battery_stats_history_tracker.reset(
          new AndroidBatteryStatsHistoryStringTracker());
    }
    return static_cast<AndroidBatteryStatsHistoryStringTracker*>(
        context->android_battery_stats_history_tracker.get());
  }

  // Returns the Uid (user ID) associated with the given HSP index.
  int32_t GetUid(int64_t index) {
    return index >= 0 ? hsp_items_[static_cast<uint64_t>(index)].uid : -1;
  }

  // Gets the string associated with the given HSP index.
  const std::string& GetString(int64_t index) {
    return index >= 0 ? hsp_items_[static_cast<uint64_t>(index)].str
                      : invalid_string_;
  }

  // Associate the given uid and string with the given HSP index.
  base::Status SetStringPoolItem(int64_t index,
                                 int32_t uid,
                                 const std::string& str);

  // Set the current version of the battery stats file being parsed.
  void battery_stats_version(uint32_t ver) { battery_stats_version_ = ver; }

  // Get the current version of the battery stats file being parsed.
  uint32_t battery_stats_version() { return battery_stats_version_; }

 private:
  struct HistoryStringPoolItem {
    // The linux User ID (UID) associated with the item.
    // max linux uid is 2^31 - 2. Battery stats sometimes gives us a uid of -1,
    // so make this signed.
    int32_t uid;
    // An arbitrary string associated with the HSP i
    std::string str;
  };

  const std::string invalid_string_ = "";

  // Use a vector to store the hsp items internally since these strings indices
  // start from zero and are consecutive.
  std::vector<HistoryStringPoolItem> hsp_items_;
  uint32_t battery_stats_version_ = 0;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BATTERY_STATS_HISTORY_STRING_TRACKER_H_
