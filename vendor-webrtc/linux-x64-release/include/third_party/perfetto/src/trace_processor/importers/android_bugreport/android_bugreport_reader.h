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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BUGREPORT_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BUGREPORT_READER_H_

#include <cstddef>
#include <cstdint>
#include <set>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/status.h"
#include "src/trace_processor/importers/android_bugreport/android_log_reader.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/util/zip_reader.h"

namespace perfetto ::trace_processor {

namespace util {
class ZipReader;
}

class TraceProcessorContext;

// Trace importer for Android bugreport.zip archives.
class AndroidBugreportReader {
 public:
  static bool IsAndroidBugReport(
      const std::vector<util::ZipFile>& zip_file_entries);
  static base::Status Parse(TraceProcessorContext* context,
                            std::vector<util::ZipFile> zip_file_entries);

 private:
  struct BugReportFile {
    tables::TraceFileTable::Id id;
    int32_t year;
    util::ZipFile file;
  };
  struct LogFile {
    tables::TraceFileTable::Id id;
    int64_t timestamp;
    util::ZipFile file;
    // Sort files to ease the job of the line-based sort. Unfortunately
    // lines within each file are not 100% timestamp-ordered, due to things like
    // kernel messages where log time != event time.
    bool operator<(const LogFile& other) const {
      return timestamp < other.timestamp;
    }
  };

  static std::optional<BugReportFile> ExtractBugReportFile(
      std::vector<util::ZipFile>& vector);

  AndroidBugreportReader(TraceProcessorContext* context,
                         BugReportFile bug_report,
                         std::set<LogFile> ordered_log_files);
  ~AndroidBugreportReader();
  base::Status ParseImpl();

  base::StatusOr<std::vector<TimestampedAndroidLogEvent>> ParseDumpstateTxt();
  base::Status ParsePersistentLogcat(std::vector<TimestampedAndroidLogEvent>);

  TraceProcessorContext* const context_;
  BugReportFile bug_report_;
  // Log files conveniently sorted by their file timestamp (see operator< in
  // LogFile)
  std::set<LogFile> ordered_log_files_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BUGREPORT_READER_H_
