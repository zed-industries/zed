/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_INTERCEPTOR_TRACE_WRITER_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_INTERCEPTOR_TRACE_WRITER_H_

#include <atomic>
#include <cstdint>
#include <functional>
#include <memory>

#include "perfetto/protozero/message_handle.h"
#include "perfetto/protozero/scattered_heap_buffer.h"
#include "perfetto/tracing/interceptor.h"
#include "perfetto/tracing/internal/data_source_internal.h"
#include "perfetto/tracing/trace_writer_base.h"
#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto {
namespace internal {

// A heap-backed trace writer used to reroute trace packets to an interceptor.
class InterceptorTraceWriter : public TraceWriterBase {
 public:
  InterceptorTraceWriter(std::unique_ptr<InterceptorBase::ThreadLocalState> tls,
                         InterceptorBase::TracePacketCallback packet_callback,
                         DataSourceStaticState* static_state,
                         uint32_t instance_index);
  ~InterceptorTraceWriter() override;

  // TraceWriterBase implementation.
  protozero::MessageHandle<protos::pbzero::TracePacket> NewTracePacket()
      override;
  void FinishTracePacket() override;
  void Flush(std::function<void()> callback = {}) override;
  uint64_t written() const override;
  uint64_t drop_count() const override;

 private:
  std::unique_ptr<InterceptorBase::ThreadLocalState> tls_;
  InterceptorBase::TracePacketCallback packet_callback_;

  protozero::HeapBuffered<protos::pbzero::TracePacket> cur_packet_;
  uint64_t bytes_written_ = 0;

  // Static state of the data source we are intercepting.
  DataSourceStaticState* const static_state_;

  // Index of the data source tracing session which we are intercepting
  // (0...kMaxDataSourceInstances - 1). Used to look up this interceptor's
  // session state (i.e., the Interceptor class instance) in the
  // DataSourceStaticState::instances array.
  const uint32_t instance_index_;

  const uint32_t sequence_id_;

  static std::atomic<uint32_t> next_sequence_id_;
};

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_INTERCEPTOR_TRACE_WRITER_H_
