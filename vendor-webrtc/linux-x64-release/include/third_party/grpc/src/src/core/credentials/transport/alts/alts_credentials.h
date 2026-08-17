//
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
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_ALTS_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_ALTS_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include "src/core/credentials/transport/security_connector.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

// Main struct for grpc ALTS channel credential.
class grpc_alts_credentials final : public grpc_channel_credentials {
 public:
  grpc_alts_credentials(const grpc_alts_credentials_options* options,
                        const char* handshaker_service_url);
  ~grpc_alts_credentials() override;

  grpc_core::RefCountedPtr<grpc_channel_security_connector>
  create_security_connector(
      grpc_core::RefCountedPtr<grpc_call_credentials> call_creds,
      const char* target_name, grpc_core::ChannelArgs* args) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

  const grpc_alts_credentials_options* options() const { return options_; }
  grpc_alts_credentials_options* mutable_options() { return options_; }
  const char* handshaker_service_url() const { return handshaker_service_url_; }

 private:
  int cmp_impl(const grpc_channel_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return grpc_core::QsortCompare(
        static_cast<const grpc_channel_credentials*>(this), other);
  }

  grpc_alts_credentials_options* options_;
  char* handshaker_service_url_;
};

// Main struct for grpc ALTS server credential.
class grpc_alts_server_credentials final : public grpc_server_credentials {
 public:
  grpc_alts_server_credentials(const grpc_alts_credentials_options* options,
                               const char* handshaker_service_url);
  ~grpc_alts_server_credentials() override;

  grpc_core::RefCountedPtr<grpc_server_security_connector>
  create_security_connector(const grpc_core::ChannelArgs& /* args */) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

  const grpc_alts_credentials_options* options() const { return options_; }
  grpc_alts_credentials_options* mutable_options() { return options_; }
  const char* handshaker_service_url() const { return handshaker_service_url_; }

 private:
  grpc_alts_credentials_options* options_;
  char* handshaker_service_url_;
};

///
/// This method creates an ALTS channel credential object with customized
/// information provided by caller.
///
///- options: grpc ALTS credentials options instance for client.
///- handshaker_service_url: address of ALTS handshaker service in the format of
///  "host:port". If it's nullptr, the address of default metadata server will
///  be used.
///- enable_untrusted_alts: a boolean flag used to enable ALTS in untrusted
///  mode. This mode can be enabled when we are sure ALTS is running on GCP or
/// for testing purpose.
///
/// It returns nullptr if the flag is disabled AND ALTS is not running on GCP.
/// Otherwise, it returns the created credential object.
///

grpc_channel_credentials* grpc_alts_credentials_create_customized(
    const grpc_alts_credentials_options* options,
    const char* handshaker_service_url, bool enable_untrusted_alts);

///
/// This method creates an ALTS server credential object with customized
/// information provided by caller.
///
///- options: grpc ALTS credentials options instance for server.
///- handshaker_service_url: address of ALTS handshaker service in the format of
///  "host:port". If it's nullptr, the address of default metadata server will
///  be used.
///- enable_untrusted_alts: a boolean flag used to enable ALTS in untrusted
///  mode. This mode can be enabled when we are sure ALTS is running on GCP or
/// for testing purpose.
///
/// It returns nullptr if the flag is disabled and ALTS is not running on GCP.
/// Otherwise, it returns the created credential object.
///
grpc_server_credentials* grpc_alts_server_credentials_create_customized(
    const grpc_alts_credentials_options* options,
    const char* handshaker_service_url, bool enable_untrusted_alts);

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_ALTS_ALTS_CREDENTIALS_H
