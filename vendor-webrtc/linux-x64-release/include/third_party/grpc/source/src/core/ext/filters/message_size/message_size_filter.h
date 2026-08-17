//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_FILTERS_MESSAGE_SIZE_MESSAGE_SIZE_FILTER_H
#define GRPC_SRC_CORE_EXT_FILTERS_MESSAGE_SIZE_MESSAGE_SIZE_FILTER_H

#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/config/core_configuration.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/service_config/service_config_parser.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/validation_errors.h"

namespace grpc_core {

class MessageSizeParsedConfig : public ServiceConfigParser::ParsedConfig {
 public:
  std::optional<uint32_t> max_send_size() const { return max_send_size_; }
  std::optional<uint32_t> max_recv_size() const { return max_recv_size_; }

  MessageSizeParsedConfig() = default;

  MessageSizeParsedConfig(std::optional<uint32_t> max_send_size,
                          std::optional<uint32_t> max_recv_size)
      : max_send_size_(max_send_size), max_recv_size_(max_recv_size) {}

  static const MessageSizeParsedConfig* GetFromCallContext(
      Arena* arena, size_t service_config_parser_index);

  static MessageSizeParsedConfig GetFromChannelArgs(const ChannelArgs& args);

  static const JsonLoaderInterface* JsonLoader(const JsonArgs&);

 private:
  std::optional<uint32_t> max_send_size_;
  std::optional<uint32_t> max_recv_size_;
};

class MessageSizeParser : public ServiceConfigParser::Parser {
 public:
  absl::string_view name() const override { return parser_name(); }

  std::unique_ptr<ServiceConfigParser::ParsedConfig> ParsePerMethodParams(
      const ChannelArgs& /*args*/, const Json& json,
      ValidationErrors* errors) override;

  static void Register(CoreConfiguration::Builder* builder);

  static size_t ParserIndex();

 private:
  static absl::string_view parser_name() { return "message_size"; }
};

std::optional<uint32_t> GetMaxRecvSizeFromChannelArgs(const ChannelArgs& args);
std::optional<uint32_t> GetMaxSendSizeFromChannelArgs(const ChannelArgs& args);

class ServerMessageSizeFilter final
    : public ImplementChannelFilter<ServerMessageSizeFilter> {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "message_size"; }

  static absl::StatusOr<std::unique_ptr<ServerMessageSizeFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args filter_args);

  explicit ServerMessageSizeFilter(const ChannelArgs& args)
      : parsed_config_(MessageSizeParsedConfig::GetFromChannelArgs(args)) {}

  class Call {
   public:
    static inline const NoInterceptor OnClientInitialMetadata;
    static inline const NoInterceptor OnServerInitialMetadata;
    static inline const NoInterceptor OnServerTrailingMetadata;
    static inline const NoInterceptor OnFinalize;
    ServerMetadataHandle OnClientToServerMessage(
        const Message& message, ServerMessageSizeFilter* filter);
    static inline const NoInterceptor OnClientToServerHalfClose;
    ServerMetadataHandle OnServerToClientMessage(
        const Message& message, ServerMessageSizeFilter* filter);
  };

 private:
  const MessageSizeParsedConfig parsed_config_;
};

class ClientMessageSizeFilter final
    : public ImplementChannelFilter<ClientMessageSizeFilter> {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "message_size"; }

  static absl::StatusOr<std::unique_ptr<ClientMessageSizeFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args filter_args);

  explicit ClientMessageSizeFilter(const ChannelArgs& args)
      : parsed_config_(MessageSizeParsedConfig::GetFromChannelArgs(args)) {}

  class Call {
   public:
    void OnClientInitialMetadata(ClientMetadata&,
                                 ClientMessageSizeFilter* filter);
    static inline const NoInterceptor OnServerInitialMetadata;
    static inline const NoInterceptor OnServerTrailingMetadata;
    static inline const NoInterceptor OnFinalize;
    ServerMetadataHandle OnClientToServerMessage(const Message& message);
    static inline const NoInterceptor OnClientToServerHalfClose;
    ServerMetadataHandle OnServerToClientMessage(const Message& message);

   private:
    MessageSizeParsedConfig limits_;
  };

 private:
  const size_t service_config_parser_index_{MessageSizeParser::ParserIndex()};
  const MessageSizeParsedConfig parsed_config_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_FILTERS_MESSAGE_SIZE_MESSAGE_SIZE_FILTER_H
