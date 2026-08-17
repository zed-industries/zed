//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_CHANNEL_ARGS_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_CHANNEL_ARGS_H

// Specifies channel args for the xDS client.
// Used only when GRPC_ARG_TEST_ONLY_DO_NOT_USE_IN_PROD_XDS_BOOTSTRAP_CONFIG
// is set.
#define GRPC_ARG_TEST_ONLY_DO_NOT_USE_IN_PROD_XDS_CLIENT_CHANNEL_ARGS \
  "grpc.xds_client_channel_args"

// Timeout in milliseconds to wait for a resource to be returned from
// the xds server before assuming that it does not exist.
// The default is 15 seconds.
#define GRPC_ARG_XDS_RESOURCE_DOES_NOT_EXIST_TIMEOUT_MS \
  "grpc.xds_resource_does_not_exist_timeout_ms"

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_CHANNEL_ARGS_H
