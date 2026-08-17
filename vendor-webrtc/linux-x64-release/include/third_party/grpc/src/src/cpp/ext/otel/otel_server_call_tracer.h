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

#ifndef GRPC_SRC_CPP_EXT_OTEL_OTEL_SERVER_CALL_TRACER_H
#define GRPC_SRC_CPP_EXT_OTEL_OTEL_SERVER_CALL_TRACER_H

#include <grpc/support/port_platform.h>

#include "absl/strings/strip.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/cpp/ext/otel/otel_plugin.h"

namespace grpc {
namespace internal {

// OpenTelemetryPluginImpl::ServerCallTracer implementation

class OpenTelemetryPluginImpl::ServerCallTracer
    : public grpc_core::ServerCallTracer {
 public:
  ServerCallTracer(
      OpenTelemetryPluginImpl* otel_plugin,
      std::shared_ptr<OpenTelemetryPluginImpl::ServerScopeConfig> scope_config)
      : start_time_(absl::Now()),
        injected_labels_from_plugin_options_(
            otel_plugin->plugin_options().size()),
        otel_plugin_(otel_plugin),
        scope_config_(std::move(scope_config)) {}

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

  // Please refer to `grpc_transport_stream_op_batch_payload` for details on
  // arguments.
  void RecordSendInitialMetadata(
      grpc_metadata_batch* send_initial_metadata) override;

  void RecordSendTrailingMetadata(
      grpc_metadata_batch* /*send_trailing_metadata*/) override;

  void RecordSendMessage(const grpc_core::Message& send_message) override;
  void RecordSendCompressedMessage(
      const grpc_core::Message& send_compressed_message) override;

  void RecordReceivedInitialMetadata(
      grpc_metadata_batch* recv_initial_metadata) override;

  void RecordReceivedMessage(const grpc_core::Message& recv_message) override;
  void RecordReceivedDecompressedMessage(
      const grpc_core::Message& recv_decompressed_message) override;

  void RecordReceivedTrailingMetadata(
      grpc_metadata_batch* /*recv_trailing_metadata*/) override {}

  void RecordCancel(grpc_error_handle /*cancel_error*/) override {
    elapsed_time_ = absl::Now() - start_time_;
  }

  void RecordEnd(const grpc_call_final_info* final_info) override;

  void RecordIncomingBytes(
      const TransportByteSize& transport_byte_size) override;
  void RecordOutgoingBytes(
      const TransportByteSize& transport_byte_size) override;

  void RecordAnnotation(absl::string_view annotation) override;

  void RecordAnnotation(const Annotation& /*annotation*/) override {
    // Not implemented
  }
  std::shared_ptr<grpc_core::TcpCallTracer> StartNewTcpTrace() override {
    // No TCP trace.
    return nullptr;
  }

 private:
  absl::string_view MethodForStats() const {
    absl::string_view method = absl::StripPrefix(path_.as_string_view(), "/");
    if (registered_method_ ||
        (otel_plugin_->generic_method_attribute_filter() != nullptr &&
         otel_plugin_->generic_method_attribute_filter()(method))) {
      return method;
    }
    return "other";
  }

  absl::Time start_time_;
  absl::Duration elapsed_time_;
  grpc_core::Slice path_;
  bool registered_method_;
  std::vector<std::unique_ptr<LabelsIterable>>
      injected_labels_from_plugin_options_;
  OpenTelemetryPluginImpl* otel_plugin_;
  std::shared_ptr<OpenTelemetryPluginImpl::ServerScopeConfig> scope_config_;
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

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_OTEL_OTEL_SERVER_CALL_TRACER_H
