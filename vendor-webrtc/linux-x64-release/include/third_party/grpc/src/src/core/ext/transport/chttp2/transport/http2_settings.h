//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_SETTINGS_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_SETTINGS_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <cstdint>
#include <optional>

#include "absl/functional/function_ref.h"
#include "absl/strings/string_view.h"
#include "src/core/ext/transport/chttp2/transport/frame.h"
#include "src/core/ext/transport/chttp2/transport/http2_status.h"
#include "src/core/util/json/json.h"
#include "src/core/util/useful.h"

namespace grpc_core {

class Http2Settings {
 public:
  enum : uint16_t {
    kHeaderTableSizeWireId = 1,
    kEnablePushWireId = 2,
    kMaxConcurrentStreamsWireId = 3,
    kInitialWindowSizeWireId = 4,
    kMaxFrameSizeWireId = 5,
    kMaxHeaderListSizeWireId = 6,
    kGrpcAllowTrueBinaryMetadataWireId = 65027,
    kGrpcPreferredReceiveCryptoFrameSizeWireId = 65028,
    kGrpcAllowSecurityFrameWireId = 65029,
  };

  void Diff(bool is_first_send, const Http2Settings& old,
            absl::FunctionRef<void(uint16_t key, uint32_t value)> cb) const;
  GRPC_MUST_USE_RESULT http2::Http2ErrorCode Apply(uint16_t key,
                                                   uint32_t value);
  uint32_t header_table_size() const { return header_table_size_; }
  uint32_t max_concurrent_streams() const { return max_concurrent_streams_; }
  uint32_t initial_window_size() const { return initial_window_size_; }
  uint32_t max_frame_size() const { return max_frame_size_; }
  uint32_t max_header_list_size() const { return max_header_list_size_; }
  uint32_t preferred_receive_crypto_message_size() const {
    return preferred_receive_crypto_message_size_;
  }
  bool enable_push() const { return enable_push_; }
  bool allow_true_binary_metadata() const {
    return allow_true_binary_metadata_;
  }
  bool allow_security_frame() const { return allow_security_frame_; }

  void SetHeaderTableSize(uint32_t x) { header_table_size_ = x; }
  void SetMaxConcurrentStreams(uint32_t x) { max_concurrent_streams_ = x; }
  void SetInitialWindowSize(uint32_t x) {
    initial_window_size_ = std::min(x, max_initial_window_size());
  }
  void SetEnablePush(bool x) { enable_push_ = x; }
  void SetMaxHeaderListSize(uint32_t x) {
    max_header_list_size_ = std::min(x, 16777216u);
  }
  void SetAllowTrueBinaryMetadata(bool x) { allow_true_binary_metadata_ = x; }
  void SetMaxFrameSize(uint32_t x) {
    max_frame_size_ = Clamp(x, min_max_frame_size(), max_max_frame_size());
  }
  void SetPreferredReceiveCryptoMessageSize(uint32_t x) {
    preferred_receive_crypto_message_size_ =
        Clamp(x, min_preferred_receive_crypto_message_size(),
              max_preferred_receive_crypto_message_size());
  }
  void SetAllowSecurityFrame(bool x) { allow_security_frame_ = x; }

  static absl::string_view header_table_size_name() {
    return "HEADER_TABLE_SIZE";
  }
  static absl::string_view max_concurrent_streams_name() {
    return "MAX_CONCURRENT_STREAMS";
  }
  static absl::string_view initial_window_size_name() {
    return "INITIAL_WINDOW_SIZE";
  }
  static absl::string_view max_frame_size_name() { return "MAX_FRAME_SIZE"; }
  static absl::string_view max_header_list_size_name() {
    return "MAX_HEADER_LIST_SIZE";
  }
  static absl::string_view enable_push_name() { return "ENABLE_PUSH"; }
  static absl::string_view allow_true_binary_metadata_name() {
    return "GRPC_ALLOW_TRUE_BINARY_METADATA";
  }
  static absl::string_view preferred_receive_crypto_message_size_name() {
    return "GRPC_PREFERRED_RECEIVE_MESSAGE_SIZE";
  }
  static absl::string_view allow_security_frame_name() {
    return "GRPC_ALLOW_SECURITY_FRAME";
  }

  static uint32_t max_initial_window_size() { return 2147483647u; }
  static uint32_t max_max_frame_size() { return 16777215u; }
  static uint32_t min_max_frame_size() { return 16384u; }
  static uint32_t min_preferred_receive_crypto_message_size() { return 16384u; }
  static uint32_t max_preferred_receive_crypto_message_size() {
    return 2147483647u;
  }

  static std::string WireIdToName(uint16_t wire_id);

  bool operator==(const Http2Settings& rhs) const {
    return header_table_size_ == rhs.header_table_size_ &&
           max_concurrent_streams_ == rhs.max_concurrent_streams_ &&
           initial_window_size_ == rhs.initial_window_size_ &&
           max_frame_size_ == rhs.max_frame_size_ &&
           max_header_list_size_ == rhs.max_header_list_size_ &&
           preferred_receive_crypto_message_size_ ==
               rhs.preferred_receive_crypto_message_size_ &&
           enable_push_ == rhs.enable_push_ &&
           allow_true_binary_metadata_ == rhs.allow_true_binary_metadata_ &&
           allow_security_frame_ == rhs.allow_security_frame_;
  }

  bool operator!=(const Http2Settings& rhs) const { return !operator==(rhs); }

  Json::Object ToJsonObject() const {
    Json::Object object;
    object["headerTableSize"] = Json::FromNumber(header_table_size());
    object["maxConcurrentStreams"] = Json::FromNumber(max_concurrent_streams());
    object["initialWindowSize"] = Json::FromNumber(initial_window_size());
    object["maxFrameSize"] = Json::FromNumber(max_frame_size());
    object["maxHeaderListSize"] = Json::FromNumber(max_header_list_size());
    object["preferredReceiveCryptoMessageSize"] =
        Json::FromNumber(preferred_receive_crypto_message_size());
    object["enablePush"] = Json::FromBool(enable_push());
    object["allowTrueBinaryMetadata"] =
        Json::FromBool(allow_true_binary_metadata());
    object["allowSecurityFrame"] = Json::FromBool(allow_security_frame());
    return object;
  }

 private:
  uint32_t header_table_size_ = 4096;
  uint32_t max_concurrent_streams_ = 4294967295u;
  uint32_t initial_window_size_ = 65535u;
  uint32_t max_frame_size_ = 16384u;
  uint32_t max_header_list_size_ = 16777216u;
  uint32_t preferred_receive_crypto_message_size_ = 0u;
  bool enable_push_ = true;
  bool allow_true_binary_metadata_ = false;
  bool allow_security_frame_ = false;
};

class Http2SettingsManager {
 public:
  Http2Settings& mutable_local() { return local_; }
  const Http2Settings& local() const { return local_; }
  const Http2Settings& acked() const { return acked_; }
  Http2Settings& mutable_peer() { return peer_; }
  const Http2Settings& peer() const { return peer_; }

  Json::Object ToJsonObject() const {
    Json::Object object;
    object["local"] = Json::FromObject(local_.ToJsonObject());
    object["sent"] = Json::FromObject(sent_.ToJsonObject());
    object["peer"] = Json::FromObject(peer_.ToJsonObject());
    object["acked"] = Json::FromObject(acked_.ToJsonObject());
    return object;
  }

  std::optional<Http2SettingsFrame> MaybeSendUpdate();
  GRPC_MUST_USE_RESULT bool AckLastSend();

 private:
  enum class UpdateState : uint8_t {
    kFirst,
    kSending,
    kIdle,
  };
  UpdateState update_state_ = UpdateState::kFirst;
  Http2Settings local_;
  Http2Settings sent_;
  Http2Settings peer_;
  Http2Settings acked_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_SETTINGS_H
