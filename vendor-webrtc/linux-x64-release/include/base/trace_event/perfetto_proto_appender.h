// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_PERFETTO_PROTO_APPENDER_H_
#define BASE_TRACE_EVENT_PERFETTO_PROTO_APPENDER_H_

#include <vector>

#include "base/memory/raw_ptr.h"
#include "base/trace_event/trace_arguments.h"
#include "third_party/perfetto/include/perfetto/protozero/contiguous_memory_range.h"
#include "third_party/perfetto/protos/perfetto/trace/track_event/debug_annotation.pbzero.h"

namespace base::trace_event {

class BASE_EXPORT PerfettoProtoAppender
    : public ConvertableToTraceFormat::ProtoAppender {
 public:
  explicit PerfettoProtoAppender(
      perfetto::protos::pbzero::DebugAnnotation* proto);
  ~PerfettoProtoAppender() override;

  // ConvertableToTraceFormat::ProtoAppender:
  void AddBuffer(uint8_t* begin, uint8_t* end) override;
  size_t Finalize(uint32_t field_id) override;

 private:
  std::vector<protozero::ContiguousMemoryRange> ranges_;
  raw_ptr<perfetto::protos::pbzero::DebugAnnotation> annotation_proto_;
};

}  // namespace base::trace_event

#endif  // BASE_TRACE_EVENT_PERFETTO_PROTO_APPENDER_H_
