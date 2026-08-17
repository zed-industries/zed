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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_FAKE_FAKE_SECURITY_CONNECTOR_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_FAKE_FAKE_SECURITY_CONNECTOR_H
#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include "src/core/credentials/transport/security_connector.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/ref_counted_ptr.h"

// Creates a fake connector that emulates real channel security.
grpc_core::RefCountedPtr<grpc_channel_security_connector>
grpc_fake_channel_security_connector_create(
    grpc_core::RefCountedPtr<grpc_channel_credentials> channel_creds,
    grpc_core::RefCountedPtr<grpc_call_credentials> request_metadata_creds,
    const char* target, const grpc_core::ChannelArgs& args);

// Creates a fake connector that emulates real server security.
grpc_core::RefCountedPtr<grpc_server_security_connector>
grpc_fake_server_security_connector_create(
    grpc_core::RefCountedPtr<grpc_server_credentials> server_creds);

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_FAKE_FAKE_SECURITY_CONNECTOR_H
