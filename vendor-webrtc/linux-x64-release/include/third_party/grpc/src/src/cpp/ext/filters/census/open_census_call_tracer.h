//
//
// Copyright 2021 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CPP_EXT_FILTERS_CENSUS_OPEN_CENSUS_CALL_TRACER_H
#define GRPC_SRC_CPP_EXT_FILTERS_CENSUS_OPEN_CENSUS_CALL_TRACER_H

#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <grpcpp/opencensus.h>
#include <stdint.h>

#include <atomic>
#include <memory>
#include <string>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "absl/time/time.h"
#include "opencensus/trace/span.h"
#include "opencensus/trace/span_context.h"
#include "opencensus/trace/span_id.h"
#include "opencensus/trace/trace_id.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/telemetry/tcp_tracer.h"
#include "src/core/util/sync.h"

// TODO(yashykt): This might not be the right place for this channel arg, but we
// don't have a better place for this right now.

// EXPERIMENTAL. If zero, disables observability tracing and observability
// logging (not yet implemented) on the client channel, defaults to true. Note
// that this does not impact metrics/stats collection. This channel arg is
// intended as a way to avoid cyclic execution of observability logging and
// trace especially when the sampling rate of RPCs is very high which would
// generate a lot of data.
//
#define GRPC_ARG_ENABLE_OBSERVABILITY "grpc.experimental.enable_observability"

namespace grpc {
namespace internal {

class OpenCensusCallTracer : public grpc_core::ClientCallTracer {
 public:
  class OpenCensusCallAttemptTracer : public CallAttemptTracer {
   public:
    OpenCensusCallAttemptTracer(OpenCensusCallTracer* parent,
                                uint64_t attempt_num, bool is_transparent_retry,
                                bool arena_allocated);

    std::string TraceId() override {
      return context_.Context().trace_id().ToHex();
    }

    std::string SpanId() override {
      return context_.Context().span_id().ToHex();
    }

    bool IsSampled() override { return context_.Span().IsSampled(); }

    void RecordSendInitialMetadata(
        grpc_metadata_batch* send_initial_metadata) override;
    void RecordSendTrailingMetadata(
        grpc_metadata_batch* /*send_trailing_metadata*/) override {}
    void RecordSendMessage(const grpc_core::Message& send_message) override;
    void RecordSendCompressedMessage(
        const grpc_core::Message& send_compressed_message) override;
    void RecordReceivedInitialMetadata(
        grpc_metadata_batch* /*recv_initial_metadata*/) override {}
    void RecordReceivedMessage(const grpc_core::Message& recv_message) override;
    void RecordReceivedDecompressedMessage(
        const grpc_core::Message& recv_decompressed_message) override;
    void RecordReceivedTrailingMetadata(
        absl::Status status, grpc_metadata_batch* recv_trailing_metadata,
        const grpc_transport_stream_stats* transport_stream_stats) override;
    void RecordIncomingBytes(
        const TransportByteSize& transport_byte_size) override;
    void RecordOutgoingBytes(
        const TransportByteSize& transport_byte_size) override;
    void RecordCancel(grpc_error_handle cancel_error) override;
    void RecordEnd() override;
    void RecordAnnotation(absl::string_view annotation) override;
    void RecordAnnotation(const Annotation& annotation) override;
    std::shared_ptr<grpc_core::TcpCallTracer> StartNewTcpTrace() override;
    void SetOptionalLabel(OptionalLabelKey,
                          grpc_core::RefCountedStringValue) override {}

    experimental::CensusContext* context() { return &context_; }

   private:
    // Maximum size of trace context is sent on the wire.
    static constexpr uint32_t kMaxTraceContextLen = 64;
    // Maximum size of tags that are sent on the wire.
    static constexpr uint32_t kMaxTagsLen = 2048;
    OpenCensusCallTracer* parent_;
    const bool arena_allocated_;
    experimental::CensusContext context_;
    // Start time (for measuring latency).
    absl::Time start_time_;
    // Number of messages in this RPC.
    uint64_t recv_message_count_ = 0;
    uint64_t sent_message_count_ = 0;
    // End status code
    absl::StatusCode status_code_;
    // TODO(roth, ctiller): Won't need atomic here once chttp2 is migrated
    // to promises, after which we can ensure that the transport invokes
    // the RecordIncomingBytes() and RecordOutgoingBytes() methods inside
    // the call's party.
    std::atomic<uint64_t> incoming_bytes_{0};
    std::atomic<uint64_t> outgoing_bytes_{0};
  };

  explicit OpenCensusCallTracer(grpc_core::Slice path, grpc_core::Arena* arena,
                                bool tracing_enabled);
  ~OpenCensusCallTracer() override;

  std::string TraceId() override {
    return context_.Context().trace_id().ToHex();
  }

  std::string SpanId() override { return context_.Context().span_id().ToHex(); }

  bool IsSampled() override { return context_.Span().IsSampled(); }

  void GenerateContext();
  OpenCensusCallAttemptTracer* StartNewAttempt(
      bool is_transparent_retry) override;
  void RecordAnnotation(absl::string_view annotation) override;
  void RecordAnnotation(const Annotation& annotation) override;

  // APIs to record API call latency
  void RecordApiLatency(absl::Duration api_latency,
                        absl::StatusCode status_code_);

 private:
  experimental::CensusContext CreateCensusContextForCallAttempt();

  // Client method.
  grpc_core::Slice path_;
  absl::string_view method_;
  experimental::CensusContext context_;
  grpc_core::Arena* arena_;
  const bool tracing_enabled_;
  grpc_core::Mutex mu_;
  // Non-transparent attempts per call
  uint64_t retries_ ABSL_GUARDED_BY(&mu_) = 0;
  // Transparent retries per call
  uint64_t transparent_retries_ ABSL_GUARDED_BY(&mu_) = 0;
  // Retry delay
  absl::Duration retry_delay_ ABSL_GUARDED_BY(&mu_);
  absl::Time time_at_last_attempt_end_ ABSL_GUARDED_BY(&mu_);
  uint64_t num_active_rpcs_ ABSL_GUARDED_BY(&mu_) = 0;
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_FILTERS_CENSUS_OPEN_CENSUS_CALL_TRACER_H
