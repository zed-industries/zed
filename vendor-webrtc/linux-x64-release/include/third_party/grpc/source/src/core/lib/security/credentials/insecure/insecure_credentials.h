//
//
// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_INSECURE_INSECURE_CREDENTIALS_H
#define GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_INSECURE_INSECURE_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/security/credentials/credentials.h"
#include "src/core/lib/security/security_connector/security_connector.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"

namespace grpc_core {

class InsecureCredentials final : public grpc_channel_credentials {
 public:
  RefCountedPtr<grpc_channel_security_connector> create_security_connector(
      RefCountedPtr<grpc_call_credentials> request_metadata_creds,
      const char* /* target_name */, ChannelArgs* /* args */) override;

  static UniqueTypeName Type();

  UniqueTypeName type() const override { return Type(); }

 private:
  int cmp_impl(const grpc_channel_credentials* other) const override;
};

class InsecureServerCredentials final : public grpc_server_credentials {
 public:
  RefCountedPtr<grpc_server_security_connector> create_security_connector(
      const ChannelArgs& /* args */) override;

  static UniqueTypeName Type();

  UniqueTypeName type() const override { return Type(); }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_INSECURE_INSECURE_CREDENTIALS_H
