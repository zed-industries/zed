/*
 * Copyright (C) 2017 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_TRACING_CORE_TRACE_WRITER_H_
#define INCLUDE_PERFETTO_EXT_TRACING_CORE_TRACE_WRITER_H_

#include <functional>

#include "perfetto/base/export.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/protozero/message_handle.h"
#include "perfetto/tracing/trace_writer_base.h"

namespace perfetto {

namespace protos {
namespace pbzero {
class TracePacket;
}  // namespace pbzero
}  // namespace protos

// See comments in include/perfetto/tracing/trace_writer_base.h
class PERFETTO_EXPORT_COMPONENT TraceWriter : public TraceWriterBase {
 public:
  using TracePacketHandle =
      protozero::MessageHandle<protos::pbzero::TracePacket>;

  TraceWriter();
  ~TraceWriter() override;

  virtual WriterID writer_id() const = 0;

 private:
  TraceWriter(const TraceWriter&) = delete;
  TraceWriter& operator=(const TraceWriter&) = delete;
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACING_CORE_TRACE_WRITER_H_
