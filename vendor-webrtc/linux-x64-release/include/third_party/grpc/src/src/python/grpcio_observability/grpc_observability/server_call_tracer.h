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

#ifndef GRPC_PYTHON_OPENCENSUS_SERVER_CALL_TRACER_H
#define GRPC_PYTHON_OPENCENSUS_SERVER_CALL_TRACER_H

#include <grpc/support/port_platform.h>

#include <atomic>

#include "absl/strings/string_view.h"
#include "absl/strings/strip.h"
#include "constants.h"
#include "metadata_exchange.h"
#include "python_observability_context.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/telemetry/call_tracer.h"

namespace grpc_observability {

class PythonOpenCensusServerCallTracerFactory
    : public grpc_core::ServerCallTracerFactory {
 public:
  grpc_core::ServerCallTracer* CreateNewServerCallTracer(
      grpc_core::Arena* arena,
      const grpc_core::ChannelArgs& channel_args) override;
  explicit PythonOpenCensusServerCallTracerFactory(
      const std::vector<Label>& exchange_labels, const char* identifier);

  bool IsServerTraced(const grpc_core::ChannelArgs& args) override;

 private:
  const std::vector<Label> exchange_labels_;
  std::string identifier_;
};

inline absl::string_view GetMethod(const grpc_core::Slice& path) {
  if (path.empty()) {
    return "";
  }
  // Check for leading '/' and trim it if present.
  return absl::StripPrefix(path.as_string_view(), "/");
}

class PythonOpenCensusServerCallTracer : public grpc_core::ServerCallTracer {
 public:
  // Maximum size of server stats that are sent on the wire.
  static constexpr uint32_t kMaxServerStatsLen = 16;

  PythonOpenCensusServerCallTracer(const std::vector<Label>& exchange_labels,
                                   std::string identifier)
      : start_time_(absl::Now()),
        recv_message_count_(0),
        sent_message_count_(0),
        labels_injector_(exchange_labels),
        identifier_(identifier) {}

  std::string TraceId() override;

  std::string SpanId() override;

  bool IsSampled() override;

  // Please refer to `grpc_transport_stream_op_batch_payload` for details on
  // arguments.
  // It's not a requirement to have this metric thus left unimplemented.
  void RecordSendInitialMetadata(
      grpc_metadata_batch* send_initial_metadata) override;

  void RecordSendTrailingMetadata(
      grpc_metadata_batch* send_trailing_metadata) override;

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

  void RecordCancel(grpc_error_handle /*cancel_error*/) override;

  void RecordEnd(const grpc_call_final_info* final_info) override;

  void RecordIncomingBytes(
      const TransportByteSize& transport_byte_size) override;

  void RecordOutgoingBytes(
      const TransportByteSize& transport_byte_size) override;

  void RecordAnnotation(absl::string_view annotation) override;

  void RecordAnnotation(const Annotation& annotation) override;

  std::shared_ptr<grpc_core::TcpCallTracer> StartNewTcpTrace() override;

 private:
  PythonCensusContext context_;
  // server method
  grpc_core::Slice path_;
  absl::string_view method_;
  absl::Time start_time_;
  absl::Duration elapsed_time_;
  uint64_t recv_message_count_;
  uint64_t sent_message_count_;
  // Buffer needed for grpc_slice to reference it when adding metadata to
  // response.
  char stats_buf_[kMaxServerStatsLen];
  PythonLabelsInjector labels_injector_;
  std::vector<Label> labels_from_peer_;
  std::string identifier_;
  bool registered_method_ = false;
  // TODO(roth, ctiller): Won't need atomic here once chttp2 is migrated
  // to promises, after which we can ensure that the transport invokes
  // the RecordIncomingBytes() and RecordOutgoingBytes() methods inside
  // the call's party.
  std::atomic<uint64_t> incoming_bytes_{0};
  std::atomic<uint64_t> outgoing_bytes_{0};
};

}  // namespace grpc_observability

#endif  // GRPC_PYTHON_OPENCENSUS_SERVER_CALL_TRACER_H
