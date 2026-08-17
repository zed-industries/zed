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

#ifndef GRPC_SRC_CPP_EXT_FILTERS_CENSUS_CLIENT_FILTER_H
#define GRPC_SRC_CPP_EXT_FILTERS_CENSUS_CLIENT_FILTER_H

#include <grpc/support/port_platform.h>
#include <grpcpp/support/client_interceptor.h>
#include <grpcpp/support/interceptor.h>

#include "absl/status/statusor.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/transport.h"

namespace grpc {
namespace internal {

class OpenCensusClientFilter : public grpc_core::ChannelFilter {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "opencensus_client"; }

  static absl::StatusOr<std::unique_ptr<OpenCensusClientFilter>> Create(
      const grpc_core::ChannelArgs& args, ChannelFilter::Args /*filter_args*/);

  explicit OpenCensusClientFilter(bool tracing_enabled)
      : tracing_enabled_(tracing_enabled) {}

  grpc_core::ArenaPromise<grpc_core::ServerMetadataHandle> MakeCallPromise(
      grpc_core::CallArgs call_args,
      grpc_core::NextPromiseFactory next_promise_factory) override;

 private:
  bool tracing_enabled_ = true;
};

class OpenCensusClientInterceptorFactory
    : public grpc::experimental::ClientInterceptorFactoryInterface {
 public:
  grpc::experimental::Interceptor* CreateClientInterceptor(
      grpc::experimental::ClientRpcInfo* info) override;
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_FILTERS_CENSUS_CLIENT_FILTER_H
