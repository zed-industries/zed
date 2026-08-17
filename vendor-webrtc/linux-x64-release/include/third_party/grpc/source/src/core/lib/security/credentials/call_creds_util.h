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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_CALL_CREDS_UTIL_H
#define GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_CALL_CREDS_UTIL_H

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <string>

#include "src/core/lib/security/credentials/credentials.h"
#include "src/core/lib/transport/transport.h"

namespace grpc_core {

// Helper function to construct service URL for jwt call creds.
std::string MakeJwtServiceUrl(
    const ClientMetadataHandle& initial_metadata,
    const grpc_call_credentials::GetRequestMetadataArgs* args);

// Helper function to construct context for plugin call creds.
grpc_auth_metadata_context MakePluginAuthMetadataContext(
    const ClientMetadataHandle& initial_metadata,
    const grpc_call_credentials::GetRequestMetadataArgs* args);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_CALL_CREDS_UTIL_H
