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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_EVALUATE_ARGS_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_EVALUATE_ARGS_H

#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/resolved_address.h"

namespace grpc_core {

class EvaluateArgs final {
 public:
  // Caller is responsible for ensuring auth_context outlives PerChannelArgs
  // struct.
  struct PerChannelArgs {
    struct Address {
      // The address in sockaddr form.
      grpc_resolved_address address;
      // The same address with only the host part.
      std::string address_str;
      int port = 0;
    };

    PerChannelArgs(grpc_auth_context* auth_context, const ChannelArgs& args);

    absl::string_view transport_security_type;
    absl::string_view spiffe_id;
    std::vector<absl::string_view> uri_sans;
    std::vector<absl::string_view> dns_sans;
    absl::string_view common_name;
    absl::string_view subject;
    Address local_address;
    Address peer_address;
  };

  EvaluateArgs(grpc_metadata_batch* metadata, PerChannelArgs* channel_args)
      : metadata_(metadata), channel_args_(channel_args) {}

  absl::string_view GetPath() const;
  absl::string_view GetAuthority() const;
  absl::string_view GetMethod() const;
  // Returns metadata value(s) for the specified key.
  // If the key is not present in the batch, returns std::nullopt.
  // If the key is present exactly once in the batch, returns a string_view of
  // that value.
  // If the key is present more than once in the batch, constructs a
  // comma-concatenated string of all values in concatenated_value and returns a
  // string_view of that string.
  std::optional<absl::string_view> GetHeaderValue(
      absl::string_view key, std::string* concatenated_value) const;

  grpc_resolved_address GetLocalAddress() const;
  absl::string_view GetLocalAddressString() const;
  int GetLocalPort() const;
  grpc_resolved_address GetPeerAddress() const;
  absl::string_view GetPeerAddressString() const;
  int GetPeerPort() const;
  absl::string_view GetTransportSecurityType() const;
  absl::string_view GetSpiffeId() const;
  std::vector<absl::string_view> GetUriSans() const;
  std::vector<absl::string_view> GetDnsSans() const;
  absl::string_view GetCommonName() const;
  absl::string_view GetSubject() const;

 private:
  grpc_metadata_batch* metadata_;
  PerChannelArgs* channel_args_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_EVALUATE_ARGS_H
