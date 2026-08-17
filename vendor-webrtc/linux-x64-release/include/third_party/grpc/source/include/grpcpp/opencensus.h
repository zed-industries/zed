//
//
// Copyright 2019 gRPC authors.
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

#ifndef GRPCPP_OPENCENSUS_H
#define GRPCPP_OPENCENSUS_H

#include "opencensus/stats/view_descriptor.h"
#include "opencensus/tags/tag_map.h"
#include "opencensus/trace/span.h"
#include "opencensus/trace/span_context.h"

namespace grpc {
class ServerContext;
// These symbols in this file will not be included in the binary unless
// grpc_opencensus_plugin build target was added as a dependency. At the moment
// it is only setup to be built with Bazel.

// Registers the OpenCensus plugin with gRPC, so that it will be used for future
// RPCs. This must be called before any views are created.
void RegisterOpenCensusPlugin();

// RPC stats definitions, defined by
// https://github.com/census-instrumentation/opencensus-specs/blob/master/stats/gRPC.md

// Registers the cumulative gRPC views so that they will be exported by any
// registered stats exporter. For on-task stats, construct a View using the
// ViewDescriptors below.
void RegisterOpenCensusViewsForExport();

// Returns the tracing Span for the current RPC.
::opencensus::trace::Span GetSpanFromServerContext(ServerContext* context);

namespace experimental {

// The tag keys set when recording RPC stats.
::opencensus::tags::TagKey ClientMethodTagKey();
::opencensus::tags::TagKey ClientStatusTagKey();
::opencensus::tags::TagKey ServerMethodTagKey();
::opencensus::tags::TagKey ServerStatusTagKey();

// Names of measures used by the plugin--users can create views on these
// measures but should not record data for them.
extern const absl::string_view kRpcClientSentMessagesPerRpcMeasureName;
extern const absl::string_view kRpcClientSentBytesPerRpcMeasureName;
extern const absl::string_view kRpcClientReceivedMessagesPerRpcMeasureName;
extern const absl::string_view kRpcClientReceivedBytesPerRpcMeasureName;
extern const absl::string_view kRpcClientRoundtripLatencyMeasureName;
extern const absl::string_view kRpcClientServerLatencyMeasureName;
extern const absl::string_view kRpcClientStartedRpcsMeasureName;
extern const absl::string_view kRpcClientRetriesPerCallMeasureName;
extern const absl::string_view kRpcClientTransparentRetriesPerCallMeasureName;
extern const absl::string_view kRpcClientRetryDelayPerCallMeasureName;
extern const absl::string_view kRpcClientTransportLatencyMeasureName;

extern const absl::string_view kRpcServerSentMessagesPerRpcMeasureName;
extern const absl::string_view kRpcServerSentBytesPerRpcMeasureName;
extern const absl::string_view kRpcServerReceivedMessagesPerRpcMeasureName;
extern const absl::string_view kRpcServerReceivedBytesPerRpcMeasureName;
extern const absl::string_view kRpcServerServerLatencyMeasureName;
extern const absl::string_view kRpcServerStartedRpcsMeasureName;

// Canonical gRPC view definitions.
const ::opencensus::stats::ViewDescriptor& ClientStartedRpcs();
const ::opencensus::stats::ViewDescriptor& ClientCompletedRpcs();
const ::opencensus::stats::ViewDescriptor& ClientRoundtripLatency();
const ::opencensus::stats::ViewDescriptor&
ClientSentCompressedMessageBytesPerRpc();
const ::opencensus::stats::ViewDescriptor&
ClientReceivedCompressedMessageBytesPerRpc();
const ::opencensus::stats::ViewDescriptor& ClientTransportLatency();

const ::opencensus::stats::ViewDescriptor& ServerStartedRpcs();
const ::opencensus::stats::ViewDescriptor& ServerCompletedRpcs();
const ::opencensus::stats::ViewDescriptor&
ServerSentCompressedMessageBytesPerRpc();
const ::opencensus::stats::ViewDescriptor&
ServerReceivedCompressedMessageBytesPerRpc();
const ::opencensus::stats::ViewDescriptor& ServerServerLatency();

const ::opencensus::stats::ViewDescriptor& ClientSentMessagesPerRpcCumulative();
const ::opencensus::stats::ViewDescriptor& ClientSentBytesPerRpcCumulative();
const ::opencensus::stats::ViewDescriptor&
ClientReceivedMessagesPerRpcCumulative();
const ::opencensus::stats::ViewDescriptor&
ClientReceivedBytesPerRpcCumulative();
const ::opencensus::stats::ViewDescriptor& ClientRoundtripLatencyCumulative();
const ::opencensus::stats::ViewDescriptor& ClientServerLatencyCumulative();
const ::opencensus::stats::ViewDescriptor& ClientStartedRpcsCumulative();
const ::opencensus::stats::ViewDescriptor& ClientCompletedRpcsCumulative();
const ::opencensus::stats::ViewDescriptor& ClientRetriesPerCallCumulative();
const ::opencensus::stats::ViewDescriptor& ClientRetriesCumulative();
const ::opencensus::stats::ViewDescriptor&
ClientTransparentRetriesPerCallCumulative();
const ::opencensus::stats::ViewDescriptor& ClientTransparentRetriesCumulative();
const ::opencensus::stats::ViewDescriptor& ClientRetryDelayPerCallCumulative();

const ::opencensus::stats::ViewDescriptor& ServerSentBytesPerRpcCumulative();
const ::opencensus::stats::ViewDescriptor&
ServerReceivedBytesPerRpcCumulative();
const ::opencensus::stats::ViewDescriptor& ServerServerLatencyCumulative();
const ::opencensus::stats::ViewDescriptor& ServerStartedRpcsCumulative();
const ::opencensus::stats::ViewDescriptor& ServerCompletedRpcsCumulative();
const ::opencensus::stats::ViewDescriptor& ServerSentMessagesPerRpcCumulative();
const ::opencensus::stats::ViewDescriptor&
ServerReceivedMessagesPerRpcCumulative();

const ::opencensus::stats::ViewDescriptor& ClientSentMessagesPerRpcMinute();
const ::opencensus::stats::ViewDescriptor& ClientSentBytesPerRpcMinute();
const ::opencensus::stats::ViewDescriptor& ClientReceivedMessagesPerRpcMinute();
const ::opencensus::stats::ViewDescriptor& ClientReceivedBytesPerRpcMinute();
const ::opencensus::stats::ViewDescriptor& ClientRoundtripLatencyMinute();
const ::opencensus::stats::ViewDescriptor& ClientServerLatencyMinute();
const ::opencensus::stats::ViewDescriptor& ClientStartedRpcsMinute();
const ::opencensus::stats::ViewDescriptor& ClientCompletedRpcsMinute();
const ::opencensus::stats::ViewDescriptor& ClientRetriesPerCallMinute();
const ::opencensus::stats::ViewDescriptor& ClientRetriesMinute();
const ::opencensus::stats::ViewDescriptor&
ClientTransparentRetriesPerCallMinute();
const ::opencensus::stats::ViewDescriptor& ClientTransparentRetriesMinute();
const ::opencensus::stats::ViewDescriptor& ClientRetryDelayPerCallMinute();

const ::opencensus::stats::ViewDescriptor& ServerSentMessagesPerRpcMinute();
const ::opencensus::stats::ViewDescriptor& ServerSentBytesPerRpcMinute();
const ::opencensus::stats::ViewDescriptor& ServerReceivedMessagesPerRpcMinute();
const ::opencensus::stats::ViewDescriptor& ServerReceivedBytesPerRpcMinute();
const ::opencensus::stats::ViewDescriptor& ServerServerLatencyMinute();
const ::opencensus::stats::ViewDescriptor& ServerStartedRpcsMinute();
const ::opencensus::stats::ViewDescriptor& ServerCompletedRpcsMinute();

const ::opencensus::stats::ViewDescriptor& ClientSentMessagesPerRpcHour();
const ::opencensus::stats::ViewDescriptor& ClientSentBytesPerRpcHour();
const ::opencensus::stats::ViewDescriptor& ClientReceivedMessagesPerRpcHour();
const ::opencensus::stats::ViewDescriptor& ClientReceivedBytesPerRpcHour();
const ::opencensus::stats::ViewDescriptor& ClientRoundtripLatencyHour();
const ::opencensus::stats::ViewDescriptor& ClientServerLatencyHour();
const ::opencensus::stats::ViewDescriptor& ClientStartedRpcsHour();
const ::opencensus::stats::ViewDescriptor& ClientCompletedRpcsHour();
const ::opencensus::stats::ViewDescriptor& ClientRetriesPerCallHour();
const ::opencensus::stats::ViewDescriptor& ClientRetriesHour();
const ::opencensus::stats::ViewDescriptor&
ClientTransparentRetriesPerCallHour();
const ::opencensus::stats::ViewDescriptor& ClientTransparentRetriesHour();
const ::opencensus::stats::ViewDescriptor& ClientRetryDelayPerCallHour();

const ::opencensus::stats::ViewDescriptor& ServerSentMessagesPerRpcHour();
const ::opencensus::stats::ViewDescriptor& ServerSentBytesPerRpcHour();
const ::opencensus::stats::ViewDescriptor& ServerReceivedMessagesPerRpcHour();
const ::opencensus::stats::ViewDescriptor& ServerReceivedBytesPerRpcHour();
const ::opencensus::stats::ViewDescriptor& ServerServerLatencyHour();
const ::opencensus::stats::ViewDescriptor& ServerStartedRpcsHour();
const ::opencensus::stats::ViewDescriptor& ServerCompletedRpcsHour();

// Thread compatible.
class CensusContext {
 public:
  CensusContext() : span_(::opencensus::trace::Span::BlankSpan()), tags_({}) {}

  explicit CensusContext(absl::string_view name,
                         const ::opencensus::tags::TagMap& tags)
      : span_(::opencensus::trace::Span::StartSpan(name)), tags_(tags) {}

  explicit CensusContext(const ::opencensus::tags::TagMap& tags)
      : span_(::opencensus::trace::Span::BlankSpan()), tags_(tags) {}

  CensusContext(absl::string_view name, const ::opencensus::trace::Span* parent,
                const ::opencensus::tags::TagMap& tags)
      : span_(::opencensus::trace::Span::StartSpan(name, parent)),
        tags_(tags) {}

  CensusContext(absl::string_view name,
                const ::opencensus::trace::SpanContext& parent_ctxt)
      : span_(::opencensus::trace::Span::StartSpanWithRemoteParent(
            name, parent_ctxt)),
        tags_({}) {}

  CensusContext(absl::string_view name,
                const ::opencensus::trace::SpanContext& parent_ctxt,
                const ::opencensus::tags::TagMap& tags)
      : span_(::opencensus::trace::Span::StartSpanWithRemoteParent(
            name, parent_ctxt)),
        tags_(tags) {}

  void AddSpanAttribute(absl::string_view key,
                        opencensus::trace::AttributeValueRef attribute) {
    span_.AddAttribute(key, attribute);
  }

  void AddSpanAnnotation(absl::string_view description,
                         opencensus::trace::AttributesRef attributes) {
    span_.AddAnnotation(description, attributes);
  }

  const ::opencensus::trace::Span& Span() const { return span_; }
  const ::opencensus::tags::TagMap& tags() const { return tags_; }

  ::opencensus::trace::SpanContext Context() const { return Span().context(); }
  void EndSpan() { Span().End(); }

 private:
  ::opencensus::trace::Span span_;
  ::opencensus::tags::TagMap tags_;
};

}  // namespace experimental

}  // namespace grpc

#endif  // GRPCPP_OPENCENSUS_H
