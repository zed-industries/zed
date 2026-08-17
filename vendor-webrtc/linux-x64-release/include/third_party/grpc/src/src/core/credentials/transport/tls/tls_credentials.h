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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_TLS_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_TLS_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include "src/core/credentials/transport/security_connector.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"

class TlsCredentials final : public grpc_channel_credentials {
 public:
  explicit TlsCredentials(
      grpc_core::RefCountedPtr<grpc_tls_credentials_options> options);
  ~TlsCredentials() override;

  grpc_core::RefCountedPtr<grpc_channel_security_connector>
  create_security_connector(
      grpc_core::RefCountedPtr<grpc_call_credentials> call_creds,
      const char* target_name, grpc_core::ChannelArgs* args) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

  grpc_tls_credentials_options* options() const { return options_.get(); }

 private:
  int cmp_impl(const grpc_channel_credentials* other) const override;

  grpc_core::RefCountedPtr<grpc_tls_credentials_options> options_;
};

class TlsServerCredentials final : public grpc_server_credentials {
 public:
  explicit TlsServerCredentials(
      grpc_core::RefCountedPtr<grpc_tls_credentials_options> options);
  ~TlsServerCredentials() override;

  grpc_core::RefCountedPtr<grpc_server_security_connector>
  create_security_connector(const grpc_core::ChannelArgs& /* args */) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

  grpc_tls_credentials_options* options() const { return options_.get(); }

 private:
  grpc_core::RefCountedPtr<grpc_tls_credentials_options> options_;
};

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_TLS_CREDENTIALS_H
