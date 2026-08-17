//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CPP_EXT_OTEL_OTEL_CLIENT_CALL_TRACER_H
#define GRPC_SRC_CPP_EXT_OTEL_OTEL_CLIENT_CALL_TRACER_H

#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stdint.h>

#include <memory>
#include <string>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "absl/time/time.h"
#include "opentelemetry/trace/span.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/telemetry/tcp_tracer.h"
#include "src/core/util/sync.h"
#include "src/cpp/ext/otel/otel_plugin.h"

namespace grpc {
namespace internal {

class OpenTelemetryPluginImpl::ClientCallTracer
    : public grpc_core::ClientCallTracer {
 public:
  class CallAttemptTracer
      : public grpc_core::ClientCallTracer::CallAttemptTracer {
   public:
    CallAttemptTracer(const OpenTelemetryPluginImpl::ClientCallTracer* parent,
                      uint64_t attempt_num, bool is_transparent_retry,
                      bool arena_allocated);

    std::string TraceId() override {
      // Not implemented
      return "";
    }

    std::string SpanId() override {
      // Not implemented
      return "";
    }

    bool IsSampled() override {
      // Not implemented
      return false;
    }

    void RecordSendInitialMetadata(
        grpc_metadata_batch* send_initial_metadata) override;
    void RecordSendTrailingMetadata(
        grpc_metadata_batch* /*send_trailing_metadata*/) override {}
    void RecordSendMessage(const grpc_core::Message& send_message) override;
    void RecordSendCompressedMessage(
        const grpc_core::Message& send_compressed_message) override;
    void RecordReceivedInitialMetadata(
        grpc_metadata_batch* recv_initial_metadata) override;
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
    void RecordAnnotation(absl::string_view /*annotation*/) override;
    void RecordAnnotation(const Annotation& /*annotation*/) override;
    std::shared_ptr<grpc_core::TcpCallTracer> StartNewTcpTrace() override;
    void SetOptionalLabel(OptionalLabelKey key,
                          grpc_core::RefCountedStringValue value) override;

   private:
    void PopulateLabelInjectors(grpc_metadata_batch* metadata);

    const ClientCallTracer* parent_;
    const bool arena_allocated_;
    // Start time (for measuring latency).
    absl::Time start_time_;
    std::unique_ptr<LabelsIterable> injected_labels_;
    // Avoid std::map to avoid per-call allocations.
    std::array<grpc_core::RefCountedStringValue,
               static_cast<size_t>(OptionalLabelKey::kSize)>
        optional_labels_;
    std::vector<std::unique_ptr<LabelsIterable>>
        injected_labels_from_plugin_options_;
    bool is_trailers_only_ = false;
    // TODO(roth, ctiller): Won't need atomic here once chttp2 is migrated
    // to promises, after which we can ensure that the transport invokes
    // the RecordIncomingBytes() and RecordOutgoingBytes() methods inside
    // the call's party.
    std::atomic<uint64_t> incoming_bytes_{0};
    std::atomic<uint64_t> outgoing_bytes_{0};
    opentelemetry::nostd::shared_ptr<opentelemetry::trace::Span> span_;
    uint64_t send_seq_num_ = 0;
    uint64_t recv_seq_num_ = 0;
  };

  ClientCallTracer(
      const grpc_core::Slice& path, grpc_core::Arena* arena,
      bool registered_method, OpenTelemetryPluginImpl* otel_plugin,
      std::shared_ptr<OpenTelemetryPluginImpl::ClientScopeConfig> scope_config);
  ~ClientCallTracer() override;

  std::string TraceId() override {
    // Not implemented
    return "";
  }

  std::string SpanId() override {
    // Not implemented
    return "";
  }

  bool IsSampled() override {
    // Not implemented
    return false;
  }

  CallAttemptTracer* StartNewAttempt(bool is_transparent_retry) override;
  void RecordAnnotation(absl::string_view /*annotation*/) override;
  void RecordAnnotation(const Annotation& /*annotation*/) override;

 private:
  absl::string_view MethodForStats() const;

  // Client method.
  grpc_core::Slice path_;
  grpc_core::Arena* arena_;
  const bool registered_method_;
  OpenTelemetryPluginImpl* otel_plugin_;
  std::shared_ptr<OpenTelemetryPluginImpl::ClientScopeConfig> scope_config_;
  grpc_core::Mutex mu_;
  // Non-transparent attempts per call (including first attempt)
  uint64_t retries_ ABSL_GUARDED_BY(&mu_) = 0;
  // Transparent retries per call
  uint64_t transparent_retries_ ABSL_GUARDED_BY(&mu_) = 0;
  opentelemetry::nostd::shared_ptr<opentelemetry::trace::Span> span_;
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_OTEL_OTEL_CLIENT_CALL_TRACER_H
