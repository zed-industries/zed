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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_EVENT_PARSER_IMPL_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_EVENT_PARSER_IMPL_H_

#include <cstdint>

#include "src/trace_processor/importers/android_bugreport/android_log_event.h"
#include "src/trace_processor/importers/common/trace_parser.h"

namespace perfetto ::trace_processor {

class TraceProcessorContext;

class AndroidLogEventParserImpl : public AndroidLogEventParser {
 public:
  explicit AndroidLogEventParserImpl(TraceProcessorContext* context)
      : context_(context) {}
  ~AndroidLogEventParserImpl() override;

  void ParseAndroidLogEvent(int64_t, AndroidLogEvent) override;

 private:
  TraceProcessorContext* const context_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_LOG_EVENT_PARSER_IMPL_H_
