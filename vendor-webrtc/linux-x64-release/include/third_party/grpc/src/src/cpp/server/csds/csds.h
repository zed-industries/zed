//
//
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
//
//

#ifndef GRPC_SRC_CPP_SERVER_CSDS_CSDS_H
#define GRPC_SRC_CPP_SERVER_CSDS_CSDS_H

#include <grpc/support/port_platform.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/support/status.h>
#include <grpcpp/support/sync_stream.h>

#include "src/proto/grpc/testing/xds/v3/csds.grpc.pb.h"

namespace grpc {
namespace xds {
namespace experimental {

// The implementation of
// envoy::service::status::v3::ClientStatusDiscoveryService
class ClientStatusDiscoveryService final
    : public envoy::service::status::v3::ClientStatusDiscoveryService::Service {
 public:
  // A streaming call that responds client status for each request.
  Status StreamClientStatus(
      ServerContext* /*context*/,
      ServerReaderWriter<envoy::service::status::v3::ClientStatusResponse,
                         envoy::service::status::v3::ClientStatusRequest>*
          stream) override;

  // An unary call to fetch client status.
  Status FetchClientStatus(
      ServerContext* /*unused*/,
      const envoy::service::status::v3::ClientStatusRequest* /*request*/,
      envoy::service::status::v3::ClientStatusResponse* response) override;
};

}  // namespace experimental
}  // namespace xds
}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_CSDS_CSDS_H
