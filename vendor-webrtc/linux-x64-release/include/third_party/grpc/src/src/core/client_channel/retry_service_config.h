//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_SERVICE_CONFIG_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_SERVICE_CONFIG_H

#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>

#include "absl/strings/string_view.h"
#include "src/core/call/status_util.h"
#include "src/core/config/core_configuration.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/service_config/service_config_parser.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/time.h"
#include "src/core/util/validation_errors.h"

namespace grpc_core {
namespace internal {

class RetryGlobalConfig final : public ServiceConfigParser::ParsedConfig {
 public:
  uintptr_t max_milli_tokens() const { return max_milli_tokens_; }
  uintptr_t milli_token_ratio() const { return milli_token_ratio_; }

  static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
  void JsonPostLoad(const Json& json, const JsonArgs& args,
                    ValidationErrors* errors);

 private:
  uintptr_t max_milli_tokens_ = 0;
  uintptr_t milli_token_ratio_ = 0;
};

class RetryMethodConfig final : public ServiceConfigParser::ParsedConfig {
 public:
  int max_attempts() const { return max_attempts_; }
  Duration initial_backoff() const { return initial_backoff_; }
  Duration max_backoff() const { return max_backoff_; }
  float backoff_multiplier() const { return backoff_multiplier_; }
  StatusCodeSet retryable_status_codes() const {
    return retryable_status_codes_;
  }
  std::optional<Duration> per_attempt_recv_timeout() const {
    return per_attempt_recv_timeout_;
  }

  static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
  void JsonPostLoad(const Json& json, const JsonArgs& args,
                    ValidationErrors* errors);

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const RetryMethodConfig& config) {
    sink.Append(absl::StrCat(
        "max_attempts:", config.max_attempts_, " initial_backoff:",
        config.initial_backoff_, " max_backoff:", config.max_backoff_,
        " backoff_multiplier:", config.backoff_multiplier_,
        " retryable_status_codes:", config.retryable_status_codes_.ToString(),
        " per_attempt_recv_timeout:",
        config.per_attempt_recv_timeout_.has_value()
            ? absl::StrCat(*config.per_attempt_recv_timeout_)
            : "none"));
  }

 private:
  int max_attempts_ = 0;
  Duration initial_backoff_;
  Duration max_backoff_;
  float backoff_multiplier_ = 0;
  StatusCodeSet retryable_status_codes_;
  std::optional<Duration> per_attempt_recv_timeout_;
};

class RetryServiceConfigParser final : public ServiceConfigParser::Parser {
 public:
  absl::string_view name() const override { return parser_name(); }

  std::unique_ptr<ServiceConfigParser::ParsedConfig> ParseGlobalParams(
      const ChannelArgs& /*args*/, const Json& json,
      ValidationErrors* errors) override;

  std::unique_ptr<ServiceConfigParser::ParsedConfig> ParsePerMethodParams(
      const ChannelArgs& args, const Json& json,
      ValidationErrors* errors) override;

  static size_t ParserIndex();
  static void Register(CoreConfiguration::Builder* builder);

 private:
  static absl::string_view parser_name() { return "retry"; }
};

}  // namespace internal
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_SERVICE_CONFIG_H
