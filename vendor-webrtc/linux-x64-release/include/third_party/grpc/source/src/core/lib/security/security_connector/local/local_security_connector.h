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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_LOCAL_LOCAL_SECURITY_CONNECTOR_H
#define GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_LOCAL_LOCAL_SECURITY_CONNECTOR_H
#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/security/security_connector/security_connector.h"
#include "src/core/util/ref_counted_ptr.h"

///
/// This method creates a local channel security connector.
///
///- channel_creds: channel credential instance.
///- request_metadata_creds: credential object which will be sent with each
///  request. This parameter can be nullptr.
///- target_name: the name of the endpoint that the channel is connecting to.
///- args: channel args passed from the caller.
///- sc: address of local channel security connector instance to be returned
///  from the method.
///
/// It returns nullptr on failure.
///
grpc_core::RefCountedPtr<grpc_channel_security_connector>
grpc_local_channel_security_connector_create(
    grpc_core::RefCountedPtr<grpc_channel_credentials> channel_creds,
    grpc_core::RefCountedPtr<grpc_call_credentials> request_metadata_creds,
    const grpc_core::ChannelArgs& args, const char* target_name);

///
/// This method creates a local server security connector.
///
///- server_creds: server credential instance.
///- sc: address of local server security connector instance to be returned from
///  the method.
///
/// It returns nullptr on failure.
///
grpc_core::RefCountedPtr<grpc_server_security_connector>
grpc_local_server_security_connector_create(
    grpc_core::RefCountedPtr<grpc_server_credentials> server_creds);

#endif  // GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_LOCAL_LOCAL_SECURITY_CONNECTOR_H
