/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_READER_H_

#include "src/trace_processor/importers/android_bugreport/android_battery_stats_reader.h"
#include "src/trace_processor/importers/android_bugreport/android_log_reader.h"
#include "src/trace_processor/importers/android_bugreport/chunked_line_reader.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

// Trace importer for Android dumpstate files.
class AndroidDumpstateReader : public ChunkedLineReader {
 public:
  // Create a reader that will only forward events that are not present in the
  // given list.
  AndroidDumpstateReader(TraceProcessorContext* context, int32_t year = 0);
  ~AndroidDumpstateReader() override;

  base::Status ParseLine(base::StringView line) override;
  void EndOfStream(base::StringView leftovers) override;

  BufferingAndroidLogReader& log_reader() { return log_reader_; }

 protected:
  void MaybeSetTzOffsetFromAlarmService(base::StringView line);

 private:
  enum class Section { kOther = 0, kDumpsys, kLog, kBatteryStats };
  TraceProcessorContext* const context_;
  AndroidBatteryStatsReader battery_stats_reader_;
  BufferingAndroidLogReader log_reader_;
  Section current_section_ = Section::kOther;
  base::StringView current_serivce_ = "";
  StringId current_section_id_ = StringId::Null();
  StringId current_service_id_ = StringId::Null();
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_DUMPSTATE_READER_H_
