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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_PIXEL_MM_KSWAPD_EVENT_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_PIXEL_MM_KSWAPD_EVENT_TRACKER_H_

#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/util/descriptors.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;

class PixelMmKswapdEventTracker {
 public:
  explicit PixelMmKswapdEventTracker(TraceProcessorContext*);

  void ParsePixelMmKswapdWake(int64_t timestamp, uint32_t pid);
  void ParsePixelMmKswapdDone(int64_t timestamp,
                              uint32_t pid,
                              protozero::ConstBytes);

 private:
  TraceProcessorContext* context_;
  const StringId kswapd_efficiency_name_;
  const StringId efficiency_pct_name_;
  const StringId pages_scanned_name_;
  const StringId pages_reclaimed_name_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_PIXEL_MM_KSWAPD_EVENT_TRACKER_H_
