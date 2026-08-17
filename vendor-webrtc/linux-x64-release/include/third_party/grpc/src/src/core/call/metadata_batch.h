//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CALL_METADATA_BATCH_H
#define GRPC_SRC_CORE_CALL_METADATA_BATCH_H

#include <grpc/impl/compression_types.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>
#include <stdlib.h>

#include <cstdint>
#include <optional>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/container/inlined_vector.h"
#include "absl/functional/function_ref.h"
#include "absl/log/check.h"
#include "absl/meta/type_traits.h"
#include "absl/strings/numbers.h"
#include "absl/strings/string_view.h"
#include "src/core/call/custom_metadata.h"
#include "src/core/call/metadata_compression_traits.h"
#include "src/core/call/parsed_metadata.h"
#include "src/core/call/simple_slice_based_metadata.h"
#include "src/core/lib/compression/compression_internal.h"
#include "src/core/lib/experiments/experiments.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/util/chunked_vector.h"
#include "src/core/util/if_list.h"
#include "src/core/util/packed_table.h"
#include "src/core/util/time.h"
#include "src/core/util/type_list.h"

namespace grpc_core {

///////////////////////////////////////////////////////////////////////////////
// Metadata traits

// Given a metadata key and a value, return the encoded size.
// Defaults to calling the key's Encode() method and then calculating the size
// of that, but can be overridden for specific keys if there's a better way of
// doing this.
// May return 0 if the size is unknown/unknowable.
template <typename Key>
size_t EncodedSizeOfKey(Key, const typename Key::ValueType& value) {
  return Key::Encode(value).size();
}

// grpc-timeout metadata trait.
// ValueType is defined as Timestamp - an absolute timestamp (i.e. a
// deadline!), that is converted to a duration by transports before being
// sent.
// TODO(ctiller): Move this elsewhere. During the transition we need to be able
// to name this in MetadataMap, but ultimately once the transition is done we
// should not need to.
struct GrpcTimeoutMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using ValueType = Timestamp;
  using MementoType = Duration;
  using CompressionTraits = TimeoutCompressor;
  static absl::string_view key() { return "grpc-timeout"; }
  static MementoType ParseMemento(Slice value,
                                  bool will_keep_past_request_lifetime,
                                  MetadataParseErrorFn on_error);
  static ValueType MementoToValue(MementoType timeout);
  static Slice Encode(ValueType x);
  static std::string DisplayValue(ValueType x) { return x.ToString(); }
  static std::string DisplayMemento(MementoType x) { return x.ToString(); }
};

// TE metadata trait.
struct TeMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  // HTTP2 says that TE can either be empty or "trailers".
  // Empty means this trait is not included, "trailers" means kTrailers, and
  // kInvalid is used to remember an invalid value.
  enum ValueType : uint8_t {
    kTrailers,
    kInvalid,
  };
  using MementoType = ValueType;
  using CompressionTraits = KnownValueCompressor<ValueType, kTrailers>;
  static absl::string_view key() { return "te"; }
  static MementoType ParseMemento(Slice value,
                                  bool will_keep_past_request_lifetime,
                                  MetadataParseErrorFn on_error);
  static ValueType MementoToValue(MementoType te) { return te; }
  static StaticSlice Encode(ValueType x) {
    CHECK(x == kTrailers);
    return StaticSlice::FromStaticString("trailers");
  }
  static const char* DisplayValue(ValueType te);
  static const char* DisplayMemento(MementoType te) { return DisplayValue(te); }
};

inline size_t EncodedSizeOfKey(TeMetadata, TeMetadata::ValueType x) {
  return x == TeMetadata::kTrailers ? 8 : 0;
}

// content-type metadata trait.
struct ContentTypeMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = true;
  // gRPC says that content-type can be application/grpc[;something]
  // Core has only ever verified the prefix.
  // IF we want to start verifying more, we can expand this type.
  enum ValueType : uint8_t {
    kApplicationGrpc,
    kEmpty,
    kInvalid,
  };
  using MementoType = ValueType;
  using CompressionTraits = KnownValueCompressor<ValueType, kApplicationGrpc>;
  static absl::string_view key() { return "content-type"; }
  static MementoType ParseMemento(Slice value,
                                  bool will_keep_past_request_lifetime,
                                  MetadataParseErrorFn on_error);
  static ValueType MementoToValue(MementoType content_type) {
    return content_type;
  }

  static StaticSlice Encode(ValueType x);
  static const char* DisplayValue(ValueType content_type);
  static const char* DisplayMemento(ValueType content_type) {
    return DisplayValue(content_type);
  }
};

// scheme metadata trait.
struct HttpSchemeMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  enum ValueType : uint8_t {
    kHttp,
    kHttps,
    kInvalid,
  };
  using MementoType = ValueType;
  using CompressionTraits = HttpSchemeCompressor;
  static absl::string_view key() { return ":scheme"; }
  static MementoType ParseMemento(Slice value, bool,
                                  MetadataParseErrorFn on_error) {
    return Parse(value.as_string_view(), on_error);
  }
  static ValueType Parse(absl::string_view value,
                         MetadataParseErrorFn on_error);
  static ValueType MementoToValue(MementoType content_type) {
    return content_type;
  }
  static StaticSlice Encode(ValueType x);
  static const char* DisplayValue(ValueType content_type);
  static const char* DisplayMemento(MementoType content_type) {
    return DisplayValue(content_type);
  }
};

size_t EncodedSizeOfKey(HttpSchemeMetadata, HttpSchemeMetadata::ValueType x);

// method metadata trait.
struct HttpMethodMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  enum ValueType : uint8_t {
    kPost,
    kGet,
    kPut,
    kInvalid,
  };
  using MementoType = ValueType;
  using CompressionTraits = HttpMethodCompressor;
  static absl::string_view key() { return ":method"; }
  static MementoType ParseMemento(Slice value,
                                  bool will_keep_past_request_lifetime,
                                  MetadataParseErrorFn on_error);
  static ValueType MementoToValue(MementoType content_type) {
    return content_type;
  }
  static StaticSlice Encode(ValueType x);
  static const char* DisplayValue(ValueType content_type);
  static const char* DisplayMemento(MementoType content_type) {
    return DisplayValue(content_type);
  }
};

// Base type for metadata pertaining to a single compression algorithm
// (e.g., "grpc-encoding").
struct CompressionAlgorithmBasedMetadata {
  using ValueType = grpc_compression_algorithm;
  using MementoType = ValueType;
  static MementoType ParseMemento(Slice value,
                                  bool will_keep_past_request_lifetime,
                                  MetadataParseErrorFn on_error);
  static ValueType MementoToValue(MementoType x) { return x; }
  static Slice Encode(ValueType x) {
    CHECK(x != GRPC_COMPRESS_ALGORITHMS_COUNT);
    return Slice::FromStaticString(CompressionAlgorithmAsString(x));
  }
  static const char* DisplayValue(ValueType x) {
    if (const char* p = CompressionAlgorithmAsString(x)) {
      return p;
    } else {
      return "<discarded-invalid-value>";
    }
  }
  static const char* DisplayMemento(MementoType x) { return DisplayValue(x); }
};

// grpc-encoding metadata trait.
struct GrpcEncodingMetadata : public CompressionAlgorithmBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits =
      SmallIntegralValuesCompressor<GRPC_COMPRESS_ALGORITHMS_COUNT>;
  static absl::string_view key() { return "grpc-encoding"; }
};

// grpc-internal-encoding-request metadata trait.
struct GrpcInternalEncodingRequest : public CompressionAlgorithmBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = NoCompressionCompressor;
  static absl::string_view key() { return "grpc-internal-encoding-request"; }
};

// grpc-accept-encoding metadata trait.
struct GrpcAcceptEncodingMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  static absl::string_view key() { return "grpc-accept-encoding"; }
  using ValueType = CompressionAlgorithmSet;
  using MementoType = ValueType;
  using CompressionTraits = StableValueCompressor;
  static MementoType ParseMemento(Slice value, bool, MetadataParseErrorFn) {
    return CompressionAlgorithmSet::FromString(value.as_string_view());
  }
  static ValueType MementoToValue(MementoType x) { return x; }
  static Slice Encode(ValueType x) { return x.ToSlice(); }
  static absl::string_view DisplayValue(ValueType x) { return x.ToString(); }
  static absl::string_view DisplayMemento(MementoType x) {
    return DisplayValue(x);
  }
};

// user-agent metadata trait.
struct UserAgentMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = StableValueCompressor;
  static absl::string_view key() { return "user-agent"; }
};

// grpc-message metadata trait.
struct GrpcMessageMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = NoCompressionCompressor;
  static absl::string_view key() { return "grpc-message"; }
};

// host metadata trait.
struct HostMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = NoCompressionCompressor;
  static absl::string_view key() { return "host"; }
};

// endpoint-load-metrics-bin metadata trait.
struct EndpointLoadMetricsBinMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = NoCompressionCompressor;
  static absl::string_view key() { return "endpoint-load-metrics-bin"; }
};

// grpc-server-stats-bin metadata trait.
struct GrpcServerStatsBinMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = NoCompressionCompressor;
  static absl::string_view key() { return "grpc-server-stats-bin"; }
};

// grpc-trace-bin metadata trait.
struct GrpcTraceBinMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = FrequentKeyWithNoValueCompressionCompressor;
  static absl::string_view key() { return "grpc-trace-bin"; }
};

// grpc-tags-bin metadata trait.
struct GrpcTagsBinMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = FrequentKeyWithNoValueCompressionCompressor;
  static absl::string_view key() { return "grpc-tags-bin"; }
};

// XEnvoyPeerMetadata
struct XEnvoyPeerMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = true;
  using CompressionTraits = StableValueCompressor;
  static absl::string_view key() { return "x-envoy-peer-metadata"; }
};

// :authority metadata trait.
struct HttpAuthorityMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = SmallSetOfValuesCompressor;
  static absl::string_view key() { return ":authority"; }
};

// :path metadata trait.
struct HttpPathMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = SmallSetOfValuesCompressor;
  static absl::string_view key() { return ":path"; }
};

// We separate SimpleIntBasedMetadata into two pieces: one that does not
// depend on the invalid value, and one that does. This allows the compiler to
// easily see the functions that are shared, and helps reduce code bloat here.
template <typename Int>
struct SimpleIntBasedMetadataBase {
  using ValueType = Int;
  using MementoType = Int;
  static ValueType MementoToValue(MementoType value) { return value; }
  static Slice Encode(ValueType x) { return Slice::FromInt64(x); }
  static Int DisplayValue(ValueType x) { return x; }
  static Int DisplayMemento(MementoType x) { return x; }
};

template <typename Int, Int kInvalidValue>
struct SimpleIntBasedMetadata : public SimpleIntBasedMetadataBase<Int> {
  static constexpr Int invalid_value() { return kInvalidValue; }
  static Int ParseMemento(Slice value, bool, MetadataParseErrorFn on_error) {
    Int out;
    if (!absl::SimpleAtoi(value.as_string_view(), &out)) {
      on_error("not an integer", value);
      out = kInvalidValue;
    }
    return out;
  }
};

// grpc-status metadata trait.
struct GrpcStatusMetadata {
  using ValueType = grpc_status_code;
  using MementoType = grpc_status_code;
  static ValueType MementoToValue(MementoType value) { return value; }
  static Slice Encode(ValueType x) { return Slice::FromInt64(x); }
  static std::string DisplayValue(ValueType x) {
    switch (x) {
      case GRPC_STATUS_OK:
        return "OK";
      case GRPC_STATUS_CANCELLED:
        return "CANCELLED";
      case GRPC_STATUS_UNKNOWN:
        return "UNKNOWN";
      case GRPC_STATUS_INVALID_ARGUMENT:
        return "INVALID_ARGUMENT";
      case GRPC_STATUS_DEADLINE_EXCEEDED:
        return "DEADLINE_EXCEEDED";
      case GRPC_STATUS_NOT_FOUND:
        return "NOT_FOUND";
      case GRPC_STATUS_ALREADY_EXISTS:
        return "ALREADY_EXISTS";
      case GRPC_STATUS_PERMISSION_DENIED:
        return "PERMISSION_DENIED";
      case GRPC_STATUS_RESOURCE_EXHAUSTED:
        return "RESOURCE_EXHAUSTED";
      case GRPC_STATUS_FAILED_PRECONDITION:
        return "FAILED_PRECONDITION";
      case GRPC_STATUS_ABORTED:
        return "ABORTED";
      case GRPC_STATUS_OUT_OF_RANGE:
        return "OUT_OF_RANGE";
      case GRPC_STATUS_UNIMPLEMENTED:
        return "UNIMPLEMENTED";
      case GRPC_STATUS_INTERNAL:
        return "INTERNAL";
      case GRPC_STATUS_UNAVAILABLE:
        return "UNAVAILABLE";
      case GRPC_STATUS_DATA_LOSS:
        return "DATA_LOSS";
      case GRPC_STATUS_UNAUTHENTICATED:
        return "UNAUTHENTICATED";
      default:
        return absl::StrCat("UNKNOWN(", static_cast<int>(x), ")");
    }
  }
  static auto DisplayMemento(MementoType x) { return DisplayValue(x); }
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = SmallIntegralValuesCompressor<16>;
  static absl::string_view key() { return "grpc-status"; }
  static grpc_status_code ParseMemento(Slice value, bool,
                                       MetadataParseErrorFn on_error) {
    int64_t wire_value;
    if (!absl::SimpleAtoi(value.as_string_view(), &wire_value)) {
      on_error("not an integer", value);
      return GRPC_STATUS_UNKNOWN;
    }
    if (wire_value < 0) {
      on_error("negative value", value);
      return GRPC_STATUS_UNKNOWN;
    }
    if (wire_value >= GRPC_STATUS__DO_NOT_USE) {
      on_error("out of range", value);
      return GRPC_STATUS_UNKNOWN;
    }
    return static_cast<grpc_status_code>(wire_value);
  }
};

// grpc-previous-rpc-attempts metadata trait.
struct GrpcPreviousRpcAttemptsMetadata
    : public SimpleIntBasedMetadata<uint32_t, 0> {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = NoCompressionCompressor;
  static absl::string_view key() { return "grpc-previous-rpc-attempts"; }
};

// grpc-retry-pushback-ms metadata trait.
struct GrpcRetryPushbackMsMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  static absl::string_view key() { return "grpc-retry-pushback-ms"; }
  using ValueType = Duration;
  using MementoType = Duration;
  using CompressionTraits = NoCompressionCompressor;
  static ValueType MementoToValue(MementoType x) { return x; }
  static Slice Encode(Duration x) { return Slice::FromInt64(x.millis()); }
  static int64_t DisplayValue(Duration x) { return x.millis(); }
  static int64_t DisplayMemento(Duration x) { return DisplayValue(x); }
  static Duration ParseMemento(Slice value,
                               bool will_keep_past_request_lifetime,
                               MetadataParseErrorFn on_error);
};

// :status metadata trait.
// TODO(ctiller): consider moving to uint16_t
struct HttpStatusMetadata : public SimpleIntBasedMetadata<uint32_t, 0> {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = true;
  using CompressionTraits = HttpStatusCompressor;
  static absl::string_view key() { return ":status"; }
};

// "secret" metadata trait used to pass load balancing token between filters.
// This should not be exposed outside of gRPC core.
class GrpcLbClientStats;

struct GrpcLbClientStatsMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  static absl::string_view key() { return "grpclb_client_stats"; }
  using ValueType = GrpcLbClientStats*;
  using MementoType = ValueType;
  using CompressionTraits = NoCompressionCompressor;
  static ValueType MementoToValue(MementoType value) { return value; }
  static Slice Encode(ValueType) { abort(); }
  static const char* DisplayValue(ValueType) { return "<internal-lb-stats>"; }
  static const char* DisplayMemento(MementoType) {
    return "<internal-lb-stats>";
  }
  static MementoType ParseMemento(Slice, bool, MetadataParseErrorFn error) {
    error("not a valid value for grpclb_client_stats", Slice());
    return nullptr;
  }
};

inline size_t EncodedSizeOfKey(GrpcLbClientStatsMetadata,
                               GrpcLbClientStatsMetadata::ValueType) {
  return 0;
}

// lb-token metadata
struct LbTokenMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = NoCompressionCompressor;
  static absl::string_view key() { return "lb-token"; }
};

// lb-cost-bin metadata
struct LbCostBinMetadata {
  static constexpr bool kRepeatable = true;
  static constexpr bool kTransferOnTrailersOnly = false;
  static absl::string_view key() { return "lb-cost-bin"; }
  struct ValueType {
    double cost;
    std::string name;
  };
  using MementoType = ValueType;
  using CompressionTraits = NoCompressionCompressor;
  static ValueType MementoToValue(MementoType value) { return value; }
  static Slice Encode(const ValueType& x);
  static std::string DisplayValue(ValueType x);
  static std::string DisplayMemento(MementoType x) { return DisplayValue(x); }
  static MementoType ParseMemento(Slice value,
                                  bool will_keep_past_request_lifetime,
                                  MetadataParseErrorFn on_error);
};

// traceparent metadata
struct W3CTraceParentMetadata : public SimpleSliceBasedMetadata {
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  using CompressionTraits = NoCompressionCompressor;
  static absl::string_view key() { return "traceparent"; }
};

// Annotation added by a transport to note whether a failed request was never
// placed on the wire, or never seen by a server.
struct GrpcStreamNetworkState {
  static absl::string_view DebugKey() { return "GrpcStreamNetworkState"; }
  static constexpr bool kRepeatable = false;
  static constexpr bool kTransferOnTrailersOnly = false;
  enum ValueType : uint8_t {
    kNotSentOnWire,
    kNotSeenByServer,
  };
  static std::string DisplayValue(ValueType x);
};

// Annotation added by a server transport to note the peer making a request.
struct PeerString {
  static absl::string_view DebugKey() { return "PeerString"; }
  static constexpr bool kRepeatable = false;
  using ValueType = Slice;
  static std::string DisplayValue(const ValueType& x);
};

// Annotation added by various systems to describe the reason for a failure.
struct GrpcStatusContext {
  static absl::string_view DebugKey() { return "GrpcStatusContext"; }
  static constexpr bool kRepeatable = true;
  using ValueType = std::string;
  static const std::string& DisplayValue(const std::string& x);
};

// Annotation added by a transport to note that the status came from the wire.
struct GrpcStatusFromWire {
  static absl::string_view DebugKey() { return "GrpcStatusFromWire"; }
  static constexpr bool kRepeatable = false;
  using ValueType = bool;
  static absl::string_view DisplayValue(bool x) { return x ? "true" : "false"; }
};

// Annotation to denote that this call qualifies for cancelled=1 for the
// RECV_CLOSE_ON_SERVER op
struct GrpcCallWasCancelled {
  static absl::string_view DebugKey() { return "GrpcCallWasCancelled"; }
  static constexpr bool kRepeatable = false;
  using ValueType = bool;
  static absl::string_view DisplayValue(bool x) { return x ? "true" : "false"; }
};

// Annotation added by client surface code to denote wait-for-ready state
struct WaitForReady {
  struct ValueType {
    bool value = false;
    bool explicitly_set = false;
  };
  static absl::string_view DebugKey() { return "WaitForReady"; }
  static constexpr bool kRepeatable = false;
  static std::string DisplayValue(ValueType x);
};

// Annotation added by retry code to indicate a transparent retry.
struct IsTransparentRetry {
  static absl::string_view DebugKey() { return "IsTransparentRetry"; }
  static constexpr bool kRepeatable = false;
  using ValueType = bool;
  static std::string DisplayValue(ValueType x) { return x ? "true" : "false"; }
};

// Annotation added by a transport to note that server trailing metadata
// is a Trailers-Only response.
struct GrpcTrailersOnly {
  static absl::string_view DebugKey() { return "GrpcTrailersOnly"; }
  static constexpr bool kRepeatable = false;
  using ValueType = bool;
  static absl::string_view DisplayValue(bool x) { return x ? "true" : "false"; }
};

// On the client-side, the value is a uintptr_t with a value of 1 if the call
// has a registered/known method, or 0, if it's not known. On the server side,
// the value is a (ChannelRegisteredMethod*).
struct GrpcRegisteredMethod {
  static absl::string_view DebugKey() { return "GrpcRegisteredMethod"; }
  static constexpr bool kRepeatable = false;
  using ValueType = void*;
  static std::string DisplayValue(void* x);
};

// Annotation added by filters to inform the transport to tarpit this
// response: add some random delay to thwart certain kinds of attacks.
struct GrpcTarPit {
  static absl::string_view DebugKey() { return "GrpcTarPit"; }
  static constexpr bool kRepeatable = false;
  using ValueType = Empty;
  static absl::string_view DisplayValue(Empty) { return "tarpit"; }
};

namespace metadata_detail {

// Build a key/value formatted debug string.
// Output looks like 'key1: value1, key2: value2'
// The string is expected to be readable, but not necessarily parsable.
class DebugStringBuilder {
 public:
  // Add one key/value pair to the output if it is allow listed.
  // Redact only the value if it is not allow listed.
  void AddAfterRedaction(absl::string_view key, absl::string_view value);

  // Finalize the output and return the string.
  // Subsequent Add calls are UB.
  std::string TakeOutput() { return std::move(out_); }

 private:
  bool IsAllowListed(absl::string_view key) const;
  void Add(absl::string_view key, absl::string_view value);
  std::string out_;
};

// IsEncodable: Given a trait, determine if that trait is encodable, or is
// just a value attached to a MetadataMap. We use the presence of the key()
// static method to determine if a trait is encodable or not - encodable
// traits have string names, and non-encodable traits do not.
template <typename Trait, typename Ignored = void>
struct IsEncodableTrait {
  static const bool value = false;
};

template <typename Trait>
struct IsEncodableTrait<Trait, absl::void_t<decltype(Trait::key())>> {
  static const bool value = true;
};

template <typename MustBeVoid, typename... Traits>
struct EncodableTraits;

template <typename Trait, typename... Traits>
struct EncodableTraits<absl::enable_if_t<IsEncodableTrait<Trait>::value, void>,
                       Trait, Traits...> {
  using List =
      typename EncodableTraits<void,
                               Traits...>::List::template PushFront<Trait>;
};

template <typename Trait, typename... Traits>
struct EncodableTraits<absl::enable_if_t<!IsEncodableTrait<Trait>::value, void>,
                       Trait, Traits...> {
  using List = typename EncodableTraits<void, Traits...>::List;
};

template <>
struct EncodableTraits<void> {
  using List = Typelist<>;
};

template <typename Trait>
struct EncodableNameLookupKeyComparison {
  bool operator()(absl::string_view key) { return key == Trait::key(); }
};

template <typename Trait, typename Op>
struct EncodableNameLookupOnFound {
  auto operator()(Op* op) { return op->Found(Trait()); }
};

template <typename... Traits>
struct EncodableNameLookup {
  template <typename Op>
  static auto Lookup(absl::string_view key, Op* op) {
    return IfList(
        key, op, [key](Op* op) { return op->NotFound(key); },
        EncodableNameLookupKeyComparison<Traits>()...,
        EncodableNameLookupOnFound<Traits, Op>()...);
  }
};

template <typename... Traits>
using NameLookup = typename EncodableTraits<
    void, Traits...>::List::template Instantiate<EncodableNameLookup>;

// Helper to take a slice to a memento to a value.
// By splitting this part out we can scale code size as the number of
// (memento, value) types, rather than as the number of traits.
template <typename ParseMementoFn, typename MementoToValueFn>
struct ParseValue {
  template <ParseMementoFn parse_memento, MementoToValueFn memento_to_value>
  static GPR_ATTRIBUTE_NOINLINE auto Parse(Slice* value,
                                           MetadataParseErrorFn on_error)
      -> decltype(memento_to_value(parse_memento(std::move(*value), false,
                                                 on_error))) {
    return memento_to_value(parse_memento(std::move(*value), false, on_error));
  }
};

// This is an "Op" type for NameLookup.
// Used for MetadataMap::Parse, its Found/NotFound methods turn a slice into a
// ParsedMetadata object.
template <typename Container>
class ParseHelper {
 public:
  ParseHelper(Slice value, bool will_keep_past_request_lifetime,
              MetadataParseErrorFn on_error, size_t transport_size)
      : value_(std::move(value)),
        will_keep_past_request_lifetime_(will_keep_past_request_lifetime),
        on_error_(on_error),
        transport_size_(transport_size) {}

  template <typename Trait>
  GPR_ATTRIBUTE_NOINLINE ParsedMetadata<Container> Found(Trait trait) {
    return ParsedMetadata<Container>(
        trait,
        ParseValueToMemento<typename Trait::MementoType, Trait::ParseMemento>(),
        static_cast<uint32_t>(transport_size_));
  }

  GPR_ATTRIBUTE_NOINLINE ParsedMetadata<Container> NotFound(
      absl::string_view key) {
    return ParsedMetadata<Container>(
        typename ParsedMetadata<Container>::FromSlicePair{},
        Slice::FromCopiedString(key),
        will_keep_past_request_lifetime_ ? value_.TakeUniquelyOwned()
                                         : std::move(value_),
        transport_size_);
  }

 private:
  template <typename T, T (*parse_memento)(Slice, bool, MetadataParseErrorFn)>
  GPR_ATTRIBUTE_NOINLINE T ParseValueToMemento() {
    return parse_memento(std::move(value_), will_keep_past_request_lifetime_,
                         on_error_);
  }

  Slice value_;
  const bool will_keep_past_request_lifetime_;
  MetadataParseErrorFn on_error_;
  const size_t transport_size_;
};

// This is an "Op" type for NameLookup.
// Used for MetadataMap::Append, its Found/NotFound methods turn a slice into
// a value and add it to a container.
template <typename Container>
class AppendHelper {
 public:
  AppendHelper(Container* container, Slice value, MetadataParseErrorFn on_error)
      : container_(container), value_(std::move(value)), on_error_(on_error) {}

  template <typename Trait>
  GPR_ATTRIBUTE_NOINLINE void Found(Trait trait) {
    container_->Set(
        trait, ParseValue<decltype(Trait::ParseMemento),
                          decltype(Trait::MementoToValue)>::
                   template Parse<Trait::ParseMemento, Trait::MementoToValue>(
                       &value_, on_error_));
  }

  GPR_ATTRIBUTE_NOINLINE void NotFound(absl::string_view key) {
    container_->unknown_.Append(key, std::move(value_));
  }

 private:
  Container* const container_;
  Slice value_;
  MetadataParseErrorFn on_error_;
};

// This is an "Op" type for NameLookup.
// Used for MetadataMap::Remove, its Found/NotFound methods remove a key from
// the container.
template <typename Container>
class RemoveHelper {
 public:
  explicit RemoveHelper(Container* container) : container_(container) {}

  template <typename Trait>
  GPR_ATTRIBUTE_NOINLINE void Found(Trait trait) {
    container_->Remove(trait);
  }

  GPR_ATTRIBUTE_NOINLINE void NotFound(absl::string_view key) {
    container_->unknown_.Remove(key);
  }

 private:
  Container* const container_;
};

// This is an "Op" type for NameLookup.
// Used for MetadataMap::GetStringValue, its Found/NotFound methods generated
// a string value from the container.
template <typename Container>
class GetStringValueHelper {
 public:
  explicit GetStringValueHelper(const Container* container,
                                std::string* backing)
      : container_(container), backing_(backing) {}

  template <typename Trait>
  GPR_ATTRIBUTE_NOINLINE absl::enable_if_t<
      Trait::kRepeatable == false &&
          std::is_same<Slice, typename Trait::ValueType>::value,
      std::optional<absl::string_view>>
  Found(Trait) {
    const auto* value = container_->get_pointer(Trait());
    if (value == nullptr) return std::nullopt;
    return value->as_string_view();
  }

  template <typename Trait>
  GPR_ATTRIBUTE_NOINLINE absl::enable_if_t<
      Trait::kRepeatable == true &&
          !std::is_same<Slice, typename Trait::ValueType>::value,
      std::optional<absl::string_view>>
  Found(Trait) {
    const auto* value = container_->get_pointer(Trait());
    if (value == nullptr) return std::nullopt;
    backing_->clear();
    for (const auto& v : *value) {
      if (!backing_->empty()) backing_->push_back(',');
      auto new_segment = Trait::Encode(v);
      backing_->append(new_segment.begin(), new_segment.end());
    }
    return *backing_;
  }

  template <typename Trait>
  GPR_ATTRIBUTE_NOINLINE absl::enable_if_t<
      Trait::kRepeatable == false &&
          !std::is_same<Slice, typename Trait::ValueType>::value,
      std::optional<absl::string_view>>
  Found(Trait) {
    const auto* value = container_->get_pointer(Trait());
    if (value == nullptr) return std::nullopt;
    *backing_ = std::string(Trait::Encode(*value).as_string_view());
    return *backing_;
  }

  GPR_ATTRIBUTE_NOINLINE std::optional<absl::string_view> NotFound(
      absl::string_view key) {
    return container_->unknown_.GetStringValue(key, backing_);
  }

 private:
  const Container* const container_;
  std::string* backing_;
};

// Sink for key value logs
using LogFn = absl::FunctionRef<void(absl::string_view, absl::string_view)>;

template <typename T>
struct AdaptDisplayValueToLog {
  static std::string ToString(const T& value) { return std::to_string(value); }
};

template <>
struct AdaptDisplayValueToLog<std::string> {
  static std::string ToString(const std::string& value) { return value; }
};

template <>
struct AdaptDisplayValueToLog<const std::string&> {
  static std::string ToString(const std::string& value) { return value; }
};

template <>
struct AdaptDisplayValueToLog<absl::string_view> {
  static std::string ToString(absl::string_view value) {
    return std::string(value);
  }
};

template <>
struct AdaptDisplayValueToLog<Slice> {
  static std::string ToString(Slice value) {
    return std::string(value.as_string_view());
  }
};

template <>
struct AdaptDisplayValueToLog<const char*> {
  static std::string ToString(const char* value) { return std::string(value); }
};

template <>
struct AdaptDisplayValueToLog<StaticSlice> {
  static absl::string_view ToString(StaticSlice value) {
    return value.as_string_view();
  }
};

template <typename T, typename U, typename V>
GPR_ATTRIBUTE_NOINLINE void LogKeyValueTo(absl::string_view key, const T& value,
                                          V (*display_value)(U), LogFn log_fn) {
  log_fn(key, AdaptDisplayValueToLog<V>::ToString(display_value(value)));
}

// Generate a strong type for metadata values per trait.
template <typename Which, typename Ignored = void>
struct Value;

template <typename Which>
struct Value<Which, absl::enable_if_t<Which::kRepeatable == false &&
                                          IsEncodableTrait<Which>::value,
                                      void>> {
  Value() = default;
  explicit Value(const typename Which::ValueType& value) : value(value) {}
  explicit Value(typename Which::ValueType&& value)
      : value(std::forward<typename Which::ValueType>(value)) {}
  Value(const Value&) = delete;
  Value& operator=(const Value&) = delete;
  Value(Value&&) noexcept = default;
  Value& operator=(Value&& other) noexcept {
    value = std::move(other.value);
    return *this;
  }
  template <typename Encoder>
  void EncodeTo(Encoder* encoder) const {
    encoder->Encode(Which(), value);
  }
  template <typename Encoder>
  void VisitWith(Encoder* encoder) const {
    return EncodeTo(encoder);
  }
  void LogTo(LogFn log_fn) const {
    LogKeyValueTo(Which::key(), value, Which::DisplayValue, log_fn);
  }
  using StorageType = typename Which::ValueType;
  GPR_NO_UNIQUE_ADDRESS StorageType value;
};

template <typename Which>
struct Value<Which, absl::enable_if_t<Which::kRepeatable == false &&
                                          !IsEncodableTrait<Which>::value,
                                      void>> {
  Value() = default;
  explicit Value(const typename Which::ValueType& value) : value(value) {}
  explicit Value(typename Which::ValueType&& value)
      : value(std::forward<typename Which::ValueType>(value)) {}
  Value(const Value&) = delete;
  Value& operator=(const Value&) = delete;
  Value(Value&&) noexcept = default;
  Value& operator=(Value&& other) noexcept {
    value = std::move(other.value);
    return *this;
  }
  template <typename Encoder>
  void EncodeTo(Encoder*) const {}
  template <typename Encoder>
  void VisitWith(Encoder* encoder) const {
    encoder->Encode(Which(), value);
  }
  void LogTo(LogFn log_fn) const {
    LogKeyValueTo(Which::DebugKey(), value, Which::DisplayValue, log_fn);
  }
  using StorageType = typename Which::ValueType;
  GPR_NO_UNIQUE_ADDRESS StorageType value;
};

template <typename Which>
struct Value<Which, absl::enable_if_t<Which::kRepeatable == true &&
                                          IsEncodableTrait<Which>::value,
                                      void>> {
  Value() = default;
  explicit Value(const typename Which::ValueType& value) {
    this->value.push_back(value);
  }
  explicit Value(typename Which::ValueType&& value) {
    this->value.emplace_back(std::forward<typename Which::ValueType>(value));
  }
  Value(const Value&) = delete;
  Value& operator=(const Value&) = delete;
  Value(Value&& other) noexcept : value(std::move(other.value)) {}
  Value& operator=(Value&& other) noexcept {
    value = std::move(other.value);
    return *this;
  }
  template <typename Encoder>
  void EncodeTo(Encoder* encoder) const {
    for (const auto& v : value) {
      encoder->Encode(Which(), v);
    }
  }
  template <typename Encoder>
  void VisitWith(Encoder* encoder) const {
    return EncodeTo(encoder);
  }
  void LogTo(LogFn log_fn) const {
    for (const auto& v : value) {
      LogKeyValueTo(Which::key(), v, Which::Encode, log_fn);
    }
  }
  using StorageType = absl::InlinedVector<typename Which::ValueType, 1>;
  StorageType value;
};

template <typename Which>
struct Value<Which, absl::enable_if_t<Which::kRepeatable == true &&
                                          !IsEncodableTrait<Which>::value,
                                      void>> {
  Value() = default;
  explicit Value(const typename Which::ValueType& value) {
    this->value.push_back(value);
  }
  explicit Value(typename Which::ValueType&& value) {
    this->value.emplace_back(std::forward<typename Which::ValueType>(value));
  }
  Value(const Value&) = delete;
  Value& operator=(const Value&) = delete;
  Value(Value&& other) noexcept : value(std::move(other.value)) {}
  Value& operator=(Value&& other) noexcept {
    value = std::move(other.value);
    return *this;
  }
  template <typename Encoder>
  void EncodeTo(Encoder*) const {}
  template <typename Encoder>
  void VisitWith(Encoder* encoder) const {
    for (const auto& v : value) {
      encoder->Encode(Which(), v);
    }
  }
  void LogTo(LogFn log_fn) const {
    for (const auto& v : value) {
      LogKeyValueTo(Which::DebugKey(), v, Which::DisplayValue, log_fn);
    }
  }
  using StorageType = absl::InlinedVector<typename Which::ValueType, 1>;
  StorageType value;
};

// Encoder to copy some metadata
template <typename Output>
class CopySink {
 public:
  explicit CopySink(Output* dst) : dst_(dst) {}

  template <class T, class V>
  void Encode(T trait, V value) {
    dst_->Set(trait, value);
  }

  template <class T>
  void Encode(T trait, const Slice& value) {
    dst_->Set(trait, std::move(value.AsOwned()));
  }

  void Encode(const Slice& key, const Slice& value) {
    dst_->unknown_.Append(key.as_string_view(), value.Ref());
  }

 private:
  Output* dst_;
};

// Callable for the ForEach in Encode() -- for each value, call the
// appropriate encoder method.
template <typename Encoder>
struct EncodeWrapper {
  Encoder* encoder;
  template <typename Which>
  void operator()(const Value<Which>& which) {
    which.EncodeTo(encoder);
  }
};

// Callable for the table ForEach in ForEach() -- for each value, call the
// appropriate visitor method.
template <typename Encoder>
struct ForEachWrapper {
  Encoder* encoder;
  template <typename Which>
  void operator()(const Value<Which>& which) {
    which.VisitWith(encoder);
  }
};

// Callable for the ForEach in Log()
struct LogWrapper {
  LogFn log_fn;
  template <typename Which>
  void operator()(const Value<Which>& which) {
    which.LogTo(log_fn);
  }
};

// Callable for the table FilterIn -- for each value, call the
// appropriate filter method to determine of the value should be kept or
// removed.
template <typename Filterer>
struct FilterWrapper {
  Filterer filter_fn;

  template <typename Which,
            absl::enable_if_t<IsEncodableTrait<Which>::value, bool> = true>
  bool operator()(const Value<Which>& /*which*/) {
    return filter_fn(Which());
  }

  template <typename Which,
            absl::enable_if_t<!IsEncodableTrait<Which>::value, bool> = true>
  bool operator()(const Value<Which>& /*which*/) {
    return true;
  }
};

// Encoder to compute TransportSize
class TransportSizeEncoder {
 public:
  void Encode(const Slice& key, const Slice& value) {
    size_ += key.length() + value.length() + 32;
  }

  template <typename Which>
  void Encode(Which, const typename Which::ValueType& value) {
    Add(Which(), value);
  }

  void Encode(ContentTypeMetadata,
              const typename ContentTypeMetadata::ValueType& value) {
    if (value == ContentTypeMetadata::kInvalid) return;
    Add(ContentTypeMetadata(), value);
  }

  size_t size() const { return size_; }

 private:
  template <typename Which>
  void Add(Which, const typename Which::ValueType& value) {
    size_ += Which::key().length() + Which::Encode(value).length() + 32;
  }

  uint32_t size_ = 0;
};

// Handle unknown (non-trait-based) fields in the metadata map.
class UnknownMap {
 public:
  using BackingType = std::vector<std::pair<Slice, Slice>>;

  void Append(absl::string_view key, Slice value);
  void Remove(absl::string_view key);
  std::optional<absl::string_view> GetStringValue(absl::string_view key,
                                                  std::string* backing) const;

  BackingType::const_iterator begin() const { return unknown_.cbegin(); }
  BackingType::const_iterator end() const { return unknown_.cend(); }

  template <typename Filterer>
  void Filter(Filterer* filter_fn) {
    unknown_.erase(
        std::remove_if(unknown_.begin(), unknown_.end(),
                       [&](auto& pair) {
                         return !(*filter_fn)(pair.first.as_string_view());
                       }),
        unknown_.end());
  }

  bool empty() const { return unknown_.empty(); }
  size_t size() const { return unknown_.size(); }
  void Clear() { unknown_.clear(); }

 private:
  // Backing store for added metadata.
  BackingType unknown_;
};

// Given a factory template Factory, construct a type that derives from
// Factory<MetadataTrait, MetadataTrait::CompressionTraits> for all
// MetadataTraits. Useful for transports in defining the stateful parts of their
// compression algorithm.
template <template <typename, typename> class Factory,
          typename... MetadataTraits>
struct StatefulCompressor;

template <template <typename, typename> class Factory, typename MetadataTrait,
          bool kEncodable = IsEncodableTrait<MetadataTrait>::value>
struct SpecificStatefulCompressor;

template <template <typename, typename> class Factory, typename MetadataTrait>
struct SpecificStatefulCompressor<Factory, MetadataTrait, true>
    : public Factory<MetadataTrait, typename MetadataTrait::CompressionTraits> {
};

template <template <typename, typename> class Factory, typename MetadataTrait>
struct SpecificStatefulCompressor<Factory, MetadataTrait, false> {};

template <template <typename, typename> class Factory, typename MetadataTrait,
          typename... MetadataTraits>
struct StatefulCompressor<Factory, MetadataTrait, MetadataTraits...>
    : public SpecificStatefulCompressor<Factory, MetadataTrait>,
      public StatefulCompressor<Factory, MetadataTraits...> {};

template <template <typename, typename> class Factory>
struct StatefulCompressor<Factory> {};

}  // namespace metadata_detail

// Helper function for encoders
// Given a metadata trait, convert the value to a slice.
template <typename Which>
absl::enable_if_t<std::is_same<typename Which::ValueType, Slice>::value,
                  const Slice&>
MetadataValueAsSlice(const Slice& slice) {
  return slice;
}

template <typename Which>
absl::enable_if_t<!std::is_same<typename Which::ValueType, Slice>::value, Slice>
MetadataValueAsSlice(typename Which::ValueType value) {
  return Slice(Which::Encode(value));
}

// MetadataMap encodes the mapping of metadata keys to metadata values.
//
// MetadataMap takes a derived class and list of traits. Each of these trait
// objects defines one metadata field that is used by core, and so should have
// more specialized handling than just using the generic APIs.
//
// MetadataMap is the backing type for some derived type via the curiously
// recursive template pattern. This is because many types consumed by
// MetadataMap require the container type to operate on, and many of those
// types are instantiated one per trait. A naive implementation without the
// Derived type would, for traits A,B,C, then instantiate for some
// T<Container, Trait>:
//  - T<MetadataMap<A,B,C>, A>,
//  - T<MetadataMap<A,B,C>, B>,
//  - T<MetadataMap<A,B,C>, C>.
// Since these types ultimately need to be recorded in the .dynstr segment
// for dynamic linkers (if gRPC is linked as a static library) this would
// create O(N^2) bytes of symbols even in stripped libraries. To avoid this
// we use the derived type (e.g. grpc_metadata_batch right now) to capture
// the container type, and we would write T<grpc_metadata_batch, A>, etc...
// Note that now the container type uses a number of bytes that is independent
// of the number of traits, and so we return to a linear symbol table growth
// function.
//
// Each trait object has one of two possible signatures, depending on whether
// that traits field is encodable or not.
// Non-encodable traits are carried in a MetadataMap, but are never passed to
// the application nor serialized to wire.
//
// Encodable traits have the following signature:
// // Traits for the "grpc-xyz" metadata field:
// struct GrpcXyzMetadata {
//   // Can this metadata field be repeated?
//   static constexpr bool kRepeatable = ...;
//   // Should this metadata be transferred from server headers to trailers on
//   // Trailers-Only response?
//   static constexpr bool kTransferOnTrailersOnly = ...;
//   // The type that's stored on MetadataBatch
//   using ValueType = ...;
//   // The type that's stored in compression/decompression tables
//   using MementoType = ...;
//   // The string key for this metadata type (for transports that require it)
//   static absl::string_view key() { return "grpc-xyz"; }
//   // Parse a memento from a slice
//   // Takes ownership of value
//   // If will_keep_past_request_lifetime is true, expect that the returned
//   // memento will be kept for a long time, and so try not to keep a ref to
//   // the input slice.
//   // Calls fn in the case of an error that should be reported to the user
//   static MementoType ParseMemento(
//       Slice value,
//       bool will_keep_past_request_lifetime,
//       MementoParseErrorFn fn) {
//   ...
//   }
//   // Convert a memento to a value
//   static ValueType MementoToValue(MementoType memento) { ... }
//   // Convert a value to its canonical text wire format (the format that
//   // ParseMemento will accept!)
//   static Slice Encode(const ValueType& value);
//   // Convert a value to something that can be passed to StrCat and
//   displayed
//   // for debugging
//   static SomeStrCatableType DisplayValue(ValueType value) { ... }
//   static SomeStrCatableType DisplayMemento(MementoType value) { ... }
// };
//
// Non-encodable traits are determined by missing the key() method, and have
// the following signature (and by convention omit the Metadata part of the
// type name):
// // Traits for the GrpcXyz field:
// struct GrpcXyz {
//   // The string key that should be used for debug dumps - should not be a
//   // valid http2 key (ie all lower case)
//   static absl::string_view DebugKey() { return "GRPC_XYZ"; }
//   // Can this metadata field be repeated?
//   static constexpr bool kRepeatable = ...;
//   // The type that's stored on MetadataBatch
//   using ValueType = ...;
//   // Convert a value to something that can be passed to StrCat and
//   displayed
//   // for debugging
//   static SomeStrCatableType DisplayValue(ValueType value) { ... }
// };
//
// About parsing and mementos:
//
// Many gRPC transports exchange metadata as key/value strings, but also allow
// for a more efficient representation as a single integer. We can use this
// integer representation to avoid reparsing too, by storing the parsed value
// in the compression table. This is what mementos are used for.
//
// A trait offers the capability to turn a slice into a memento via
// ParseMemento. This is exposed to users of MetadataMap via the Parse()
// method, that returns a ParsedMetadata object. That ParsedMetadata object
// can in turn be used to set the same value on many different MetadataMaps
// without having to reparse.
//
// Implementation wise, ParsedMetadata is a type erased wrapper around
// MementoType. When we set a value on MetadataMap, we first turn that memento
// into a value. For most types, this is going to be a no-op, but for example
// for grpc-timeout we make the memento the timeout expressed on the wire, but
// we make the value the timestamp of when the timeout will expire (i.e. the
// deadline).
template <class Derived, typename... Traits>
class MetadataMap {
 public:
  MetadataMap() = default;
  ~MetadataMap();

  // Given a compressor factory - template taking <MetadataTrait,
  // CompressionTrait>, StatefulCompressor<Factory> provides a type
  // derived from all Encodable traits in this MetadataMap.
  // This can be used by transports to delegate compression to the appropriate
  // compression algorithm.
  template <template <typename, typename> class Factory>
  using StatefulCompressor =
      metadata_detail::StatefulCompressor<Factory, Traits...>;

  MetadataMap(const MetadataMap&) = delete;
  MetadataMap& operator=(const MetadataMap&) = delete;
  MetadataMap(MetadataMap&&) noexcept;
  // We never create MetadataMap directly, instead we create Derived, but we
  // want to be able to move it without redeclaring this.
  // NOLINTNEXTLINE(misc-unconventional-assign-operator)
  Derived& operator=(MetadataMap&&) noexcept;

  // Encode this metadata map into some encoder.
  // For each field that is set in the MetadataMap, call
  // encoder->Encode.
  //
  // For fields for which we have traits, this will be a method with
  // the signature:
  //    void Encode(TraitsType, typename TraitsType::ValueType value);
  // For fields for which we do not have traits, this will be a method
  // with the signature:
  //    void Encode(string_view key, Slice value);
  template <typename Encoder>
  void Encode(Encoder* encoder) const {
    table_.template ForEachIn<metadata_detail::EncodeWrapper<Encoder>,
                              Value<Traits>...>(
        metadata_detail::EncodeWrapper<Encoder>{encoder});
    for (const auto& unk : unknown_) {
      encoder->Encode(unk.first, unk.second);
    }
  }

  // Like Encode, but also visit the non-encodable fields.
  template <typename Encoder>
  void ForEach(Encoder* encoder) const {
    table_.ForEach(metadata_detail::ForEachWrapper<Encoder>{encoder});
    for (const auto& unk : unknown_) {
      encoder->Encode(unk.first, unk.second);
    }
  }

  // Similar to Encode, but targeted at logging: for each metadatum,
  // call f(key, value) as absl::string_views.
  void Log(metadata_detail::LogFn log_fn) const {
    table_.ForEach(metadata_detail::LogWrapper{log_fn});
    for (const auto& unk : unknown_) {
      log_fn(unk.first.as_string_view(), unk.second.as_string_view());
    }
  }

  // Filter the metadata map.
  // Iterates over all encodable and unknown headers and calls the filter_fn
  // for each of them. If the function returns true, the header is kept.
  template <typename Filterer>
  void Filter(Filterer filter_fn) {
    table_.template FilterIn<metadata_detail::FilterWrapper<Filterer>,
                             Value<Traits>...>(
        metadata_detail::FilterWrapper<Filterer>{filter_fn});
    unknown_.Filter<Filterer>(&filter_fn);
  }

  std::string DebugString() const {
    metadata_detail::DebugStringBuilder builder;
    Log([&builder](absl::string_view key, absl::string_view value) {
      builder.AddAfterRedaction(key, value);
    });
    return builder.TakeOutput();
  }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const MetadataMap& map) {
    sink.Append(map.DebugString());
  }

  // Get the pointer to the value of some known metadata.
  // Returns nullptr if the metadata is not present.
  // Causes a compilation error if Which is not an element of Traits.
  template <typename Which>
  const typename metadata_detail::Value<Which>::StorageType* get_pointer(
      Which) const {
    if (auto* p = table_.template get<Value<Which>>()) return &p->value;
    return nullptr;
  }

  // Get the pointer to the value of some known metadata.
  // Returns nullptr if the metadata is not present.
  // Causes a compilation error if Which is not an element of Traits.
  template <typename Which>
  typename metadata_detail::Value<Which>::StorageType* get_pointer(Which) {
    if (auto* p = table_.template get<Value<Which>>()) return &p->value;
    return nullptr;
  }

  // Get the pointer to the value of some known metadata.
  // Adds the default value for the metadata is not present.
  // Causes a compilation error if Which is not an element of Traits.
  template <typename Which>
  typename metadata_detail::Value<Which>::StorageType* GetOrCreatePointer(
      Which) {
    return &table_.template get_or_create<Value<Which>>()->value;
  }

  // Get the value of some known metadata.
  // Returns nullopt if the metadata is not present.
  // Causes a compilation error if Which is not an element of Traits.
  template <typename Which>
  std::optional<typename Which::ValueType> get(Which) const {
    if (auto* p = table_.template get<Value<Which>>()) return p->value;
    return std::nullopt;
  }

  // Set the value of some known metadata.
  // Returns a pointer to the new value.
  template <typename Which, typename... Args>
  absl::enable_if_t<Which::kRepeatable == false, void> Set(Which,
                                                           Args&&... args) {
    table_.template set<Value<Which>>(std::forward<Args>(args)...);
  }
  template <typename Which, typename... Args>
  absl::enable_if_t<Which::kRepeatable == true, void> Set(Which,
                                                          Args&&... args) {
    GetOrCreatePointer(Which())->emplace_back(std::forward<Args>(args)...);
  }

  // Remove a specific piece of known metadata.
  template <typename Which>
  void Remove(Which) {
    table_.template clear<Value<Which>>();
  }

  // Remove some metadata by name
  void Remove(absl::string_view key) {
    metadata_detail::RemoveHelper<Derived> helper(static_cast<Derived*>(this));
    metadata_detail::NameLookup<Traits...>::Lookup(key, &helper);
  }

  void Remove(const char* key) { Remove(absl::string_view(key)); }

  // Retrieve some metadata by name
  std::optional<absl::string_view> GetStringValue(absl::string_view name,
                                                  std::string* buffer) const {
    metadata_detail::GetStringValueHelper<Derived> helper(
        static_cast<const Derived*>(this), buffer);
    return metadata_detail::NameLookup<Traits...>::Lookup(name, &helper);
  }

  // Extract a piece of known metadata.
  // Returns nullopt if the metadata was not present, or the value if it was.
  // The same as:
  //  auto value = m.get(T());
  //  m.Remove(T());
  template <typename Which>
  absl::enable_if_t<Which::kRepeatable == false,
                    std::optional<typename Which::ValueType>>
  Take(Which which) {
    if (auto* p = get_pointer(which)) {
      std::optional<typename Which::ValueType> value(std::move(*p));
      Remove(which);
      return value;
    }
    return {};
  }

  // Extract repeated known metadata.
  // Returns an empty vector if the metadata was not present.
  template <typename Which>
  absl::enable_if_t<Which::kRepeatable == true,
                    typename metadata_detail::Value<Which>::StorageType>
  Take(Which which) {
    if (auto* p = get_pointer(which)) {
      typename Value<Which>::StorageType value = std::move(*p);
      Remove(which);
      return value;
    }
    return {};
  }

  // Parse metadata from a key/value pair, and return an object representing
  // that result.
  static ParsedMetadata<Derived> Parse(absl::string_view key, Slice value,
                                       bool will_keep_past_request_lifetime,
                                       uint32_t transport_size,
                                       MetadataParseErrorFn on_error) {
    metadata_detail::ParseHelper<Derived> helper(
        value.TakeOwned(), will_keep_past_request_lifetime, on_error,
        transport_size);
    return metadata_detail::NameLookup<Traits...>::Lookup(key, &helper);
  }

  // Set a value from a parsed metadata object.
  void Set(const ParsedMetadata<Derived>& m) {
    m.SetOnContainer(static_cast<Derived*>(this));
  }

  // Append a key/value pair - takes ownership of value
  void Append(absl::string_view key, Slice value,
              MetadataParseErrorFn on_error) {
    metadata_detail::AppendHelper<Derived> helper(static_cast<Derived*>(this),
                                                  value.TakeOwned(), on_error);
    metadata_detail::NameLookup<Traits...>::Lookup(key, &helper);
  }

  void Clear();
  size_t TransportSize() const;
  Derived Copy() const;
  bool empty() const { return table_.empty() && unknown_.empty(); }
  size_t count() const { return table_.count() + unknown_.size(); }

 private:
  friend class metadata_detail::AppendHelper<Derived>;
  friend class metadata_detail::GetStringValueHelper<Derived>;
  friend class metadata_detail::RemoveHelper<Derived>;
  friend class metadata_detail::CopySink<Derived>;
  friend class ParsedMetadata<Derived>;

  template <typename Which>
  using Value = metadata_detail::Value<Which>;

  // Table of known metadata types.
  PackedTable<Value<Traits>...> table_;
  metadata_detail::UnknownMap unknown_;
};

// Ok/not-ok check for metadata maps that contain GrpcStatusMetadata, so that
// they can be used as result types for TrySeq.
template <typename Derived, typename... Args>
inline bool IsStatusOk(const MetadataMap<Derived, Args...>& m) {
  return m.get(GrpcStatusMetadata()).value_or(GRPC_STATUS_UNKNOWN) ==
         GRPC_STATUS_OK;
}

template <typename Derived, typename... Traits>
MetadataMap<Derived, Traits...>::MetadataMap(MetadataMap&& other) noexcept
    : table_(std::move(other.table_)), unknown_(std::move(other.unknown_)) {}

// We never create MetadataMap directly, instead we create Derived, but we
// want to be able to move it without redeclaring this.
// NOLINTNEXTLINE(misc-unconventional-assign-operator)
template <typename Derived, typename... Traits>
Derived& MetadataMap<Derived, Traits...>::operator=(
    MetadataMap&& other) noexcept {
  table_ = std::move(other.table_);
  unknown_ = std::move(other.unknown_);
  return static_cast<Derived&>(*this);
}

template <typename Derived, typename... Traits>
MetadataMap<Derived, Traits...>::~MetadataMap() = default;

template <typename Derived, typename... Traits>
void MetadataMap<Derived, Traits...>::Clear() {
  table_.ClearAll();
  unknown_.Clear();
}

template <typename Derived, typename... Traits>
size_t MetadataMap<Derived, Traits...>::TransportSize() const {
  metadata_detail::TransportSizeEncoder enc;
  Encode(&enc);
  return enc.size();
}

template <typename Derived, typename... Traits>
Derived MetadataMap<Derived, Traits...>::Copy() const {
  Derived out;
  metadata_detail::CopySink<Derived> sink(&out);
  ForEach(&sink);
  return out;
}

}  // namespace grpc_core

struct grpc_metadata_batch;

using grpc_metadata_batch_base = grpc_core::MetadataMap<
    grpc_metadata_batch,
    // Colon prefixed headers first
    grpc_core::HttpPathMetadata, grpc_core::HttpAuthorityMetadata,
    grpc_core::HttpMethodMetadata, grpc_core::HttpStatusMetadata,
    grpc_core::HttpSchemeMetadata,
    // Non-colon prefixed headers begin here
    grpc_core::ContentTypeMetadata, grpc_core::TeMetadata,
    grpc_core::GrpcEncodingMetadata, grpc_core::GrpcInternalEncodingRequest,
    grpc_core::GrpcAcceptEncodingMetadata, grpc_core::GrpcStatusMetadata,
    grpc_core::GrpcTimeoutMetadata, grpc_core::GrpcPreviousRpcAttemptsMetadata,
    grpc_core::GrpcRetryPushbackMsMetadata, grpc_core::UserAgentMetadata,
    grpc_core::GrpcMessageMetadata, grpc_core::HostMetadata,
    grpc_core::EndpointLoadMetricsBinMetadata,
    grpc_core::GrpcServerStatsBinMetadata, grpc_core::GrpcTraceBinMetadata,
    grpc_core::GrpcTagsBinMetadata, grpc_core::GrpcLbClientStatsMetadata,
    grpc_core::LbCostBinMetadata, grpc_core::LbTokenMetadata,
    grpc_core::XEnvoyPeerMetadata, grpc_core::W3CTraceParentMetadata,
    // Non-encodable things
    grpc_core::GrpcStreamNetworkState, grpc_core::PeerString,
    grpc_core::GrpcStatusContext, grpc_core::GrpcStatusFromWire,
    grpc_core::GrpcCallWasCancelled, grpc_core::WaitForReady,
    grpc_core::IsTransparentRetry, grpc_core::GrpcTrailersOnly,
    grpc_core::GrpcTarPit,
    grpc_core::GrpcRegisteredMethod GRPC_CUSTOM_CLIENT_METADATA
        GRPC_CUSTOM_SERVER_METADATA>;

struct grpc_metadata_batch : public grpc_metadata_batch_base {
  using grpc_metadata_batch_base::grpc_metadata_batch_base;
};

#endif  // GRPC_SRC_CORE_CALL_METADATA_BATCH_H
