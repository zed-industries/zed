/*
 * Copyright (C) 2018 The Android Open Source Project
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
#ifndef SRC_TRACING_CORE_TRACE_WRITER_FOR_TESTING_H_
#define SRC_TRACING_CORE_TRACE_WRITER_FOR_TESTING_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>
#include <vector>

#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/trace_writer.h"
#include "perfetto/protozero/root_message.h"
#include "perfetto/protozero/scattered_heap_buffer.h"
#include "perfetto/protozero/scattered_stream_writer.h"
#include "protos/perfetto/trace/trace_packet.gen.h"

namespace perfetto {

// A specialization of TraceWriter for testing which writes into memory
// allocated by the ScatteredHeapBuffer.
// See //include/perfetto/ext/tracing/core/trace_writer.h for docs.
class TraceWriterForTesting : public TraceWriter {
 public:
  TraceWriterForTesting();
  ~TraceWriterForTesting() override;

  // TraceWriter implementation. See documentation in trace_writer.h.
  // TracePacketHandle is defined in trace_writer.h
  TracePacketHandle NewTracePacket() override;
  void FinishTracePacket() override;
  void Flush(std::function<void()> callback = {}) override;

  std::vector<protos::gen::TracePacket> GetAllTracePackets();
  protos::gen::TracePacket GetOnlyTracePacket();

  WriterID writer_id() const override;
  uint64_t written() const override;
  uint64_t drop_count() const override;

 private:
  TraceWriterForTesting(const TraceWriterForTesting&) = delete;
  TraceWriterForTesting& operator=(const TraceWriterForTesting&) = delete;

  protozero::ScatteredHeapBuffer delegate_;
  protozero::ScatteredStreamWriter stream_;

  // The packet returned via NewTracePacket(). Its owned by this class,
  // TracePacketHandle has just a pointer to it.
  //
  // The caller of NewTracePacket can use TakeStreamWriter() and use the stream
  // writer directly: in that case:
  // * cur_packet_->size() is not up to date. Only the stream writer has the
  //   correct information.
  // * cur_packet_->nested_message() is always nullptr.
  // * cur_packet_->size_field() is still used to track the start of the current
  //   packet.
  std::unique_ptr<protozero::RootMessage<protos::pbzero::TracePacket>>
      cur_packet_;

  size_t cur_packet_written_start_ = 0;
};

}  // namespace perfetto

#endif  // SRC_TRACING_CORE_TRACE_WRITER_FOR_TESTING_H_
