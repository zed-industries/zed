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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_READER_H_

#include <chrono>
#include <cstdint>
#include <optional>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/android_bugreport/android_log_event.h"
#include "src/trace_processor/importers/android_bugreport/chunked_line_reader.h"

namespace perfetto ::trace_processor {

class TraceProcessorContext;

// Helper struct to deduplicate events.
// When reading bug reports log data will be present in a dumpstate file and in
// the log cat files.
struct TimestampedAndroidLogEvent {
  // Log timestamp. We use ms resolution because dumpstate files only write at
  // this resolution.
  std::chrono::milliseconds ts;
  AndroidLogEvent event;
  // Flag to track whether a given event was already matched by the
  // deduplication logic. When set to true we will no longer consider this event
  // as a candidate for deduplication.
  bool matched;

  // Only sort by time to find duplicates at the same ts.
  bool operator<(const TimestampedAndroidLogEvent& other) const {
    return ts < other.ts;
  }
};

// Parses log lines coming from persistent logcat (FS/data/misc/logd), interns
// string in the TP string pools and populates a vector of AndroidLogEvent
// structs. Does NOT insert log events into any table (for testing isolation),
// the caller is in charge to do that.
// It supports the following formats (auto-detected):
// 1) 12-31 23:59:00.123456 <pid> <tid> I tag: message
//    This is typically found in persistent logcat (FS/data/misc/logd/)
// 2) 06-24 15:57:11.346 <uid> <pid> <tid> D Tag: Message
//    This is typically found in the recent logcat dump in bugreport-xxx.txt
class AndroidLogReader : public ChunkedLineReader {
 public:
  // Log cat will not write year into the trace so the caller needs to figure it
  // out. If not provided the reader will make a best guess.
  explicit AndroidLogReader(TraceProcessorContext* context);
  AndroidLogReader(TraceProcessorContext* context,
                   int32_t year,
                   bool wait_for_tz = false);

  ~AndroidLogReader() override;

  base::Status ParseLine(base::StringView line) override;
  void EndOfStream(base::StringView leftovers) override;

  // Called for each event parsed from the stream.
  // `event_ts_ns` is the ts of the event as read from the log.
  // Default implementation just calls `SendToSorter`.
  virtual base::Status ProcessEvent(std::chrono::nanoseconds event_ts,
                                    AndroidLogEvent event);

 protected:
  // Sends the given event to the sorting stage.
  // `event_ts` is the ts of the event as read from the log and will be
  // converted to a trace_ts (with necessary clock conversions applied)
  base::Status SendToSorter(std::chrono::nanoseconds event_ts,
                            AndroidLogEvent event);

  // Send any events to the sorter that have not already had their timestamp
  // adjusted based on the timezone. This is meant to be called once the TZ
  // offset becomes known, or we reach the end of the input without any TZ info.
  base::Status FlushNonTzAdjustedEvents();

 private:
  TraceProcessorContext* const context_;
  std::optional<AndroidLogEvent::Format> format_;
  int32_t year_;
  bool wait_for_tz_;
  std::vector<TimestampedAndroidLogEvent> non_tz_adjusted_events_;
};

// Same as AndroidLogReader (sends events to sorter), but also stores them in a
// vector that can later be feed to a `DedupingAndroidLogReader` instance.
class BufferingAndroidLogReader : public AndroidLogReader {
 public:
  BufferingAndroidLogReader(TraceProcessorContext* context,
                            int32_t year,
                            bool wait_for_tz = false)
      : AndroidLogReader(context, year, wait_for_tz) {}
  ~BufferingAndroidLogReader() override;

  base::Status ProcessEvent(std::chrono::nanoseconds event_ts,
                            AndroidLogEvent event) override;

  std::vector<TimestampedAndroidLogEvent> ConsumeBufferedEvents() && {
    return std::move(events_);
  }

 private:
  std::vector<TimestampedAndroidLogEvent> events_;
};

// Similar to `AndroidLogReader` but this class will not forward duplicate
// events. These are events already present in a given vector of events.
class DedupingAndroidLogReader : public AndroidLogReader {
 public:
  // Creates a reader that will not forward events already present in the given
  // vector. Note that entries in the vector will only be matched once. That is
  // when a match is found in the vector the event is not send to the sorter,
  // but the event is removed from the vector (seen flag is set to true) so that
  // subsequent event will not match that entry.
  DedupingAndroidLogReader(TraceProcessorContext* context,
                           int32_t year,
                           bool wait_for_tz,
                           std::vector<TimestampedAndroidLogEvent> events);
  DedupingAndroidLogReader(TraceProcessorContext* context,
                           int32_t year,
                           std::vector<TimestampedAndroidLogEvent> events)
      : DedupingAndroidLogReader(context, year, false, events) {}
  ~DedupingAndroidLogReader() override;

  base::Status ProcessEvent(std::chrono::nanoseconds event_ts,
                            AndroidLogEvent event) override;

 private:
  std::vector<TimestampedAndroidLogEvent> events_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_READER_H_
