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

#ifndef GRPC_SRC_CPP_EXT_FILTERS_CENSUS_MEASURES_H
#define GRPC_SRC_CPP_EXT_FILTERS_CENSUS_MEASURES_H

#include <grpc/support/port_platform.h>

#include "opencensus/stats/stats.h"

namespace grpc {

::opencensus::stats::MeasureInt64 RpcClientSentMessagesPerRpc();
::opencensus::stats::MeasureDouble RpcClientSentBytesPerRpc();
::opencensus::stats::MeasureInt64 RpcClientReceivedMessagesPerRpc();
::opencensus::stats::MeasureDouble RpcClientReceivedBytesPerRpc();
::opencensus::stats::MeasureDouble RpcClientRoundtripLatency();
::opencensus::stats::MeasureDouble RpcClientServerLatency();
::opencensus::stats::MeasureInt64 RpcClientStartedRpcs();
::opencensus::stats::MeasureInt64 RpcClientCompletedRpcs();
::opencensus::stats::MeasureInt64 RpcClientRetriesPerCall();
::opencensus::stats::MeasureInt64 RpcClientTransparentRetriesPerCall();
::opencensus::stats::MeasureDouble RpcClientRetryDelayPerCall();
::opencensus::stats::MeasureDouble RpcClientTransportLatency();

::opencensus::stats::MeasureInt64 RpcServerSentMessagesPerRpc();
::opencensus::stats::MeasureDouble RpcServerSentBytesPerRpc();
::opencensus::stats::MeasureInt64 RpcServerReceivedMessagesPerRpc();
::opencensus::stats::MeasureDouble RpcServerReceivedBytesPerRpc();
::opencensus::stats::MeasureDouble RpcServerServerLatency();
::opencensus::stats::MeasureInt64 RpcServerStartedRpcs();
::opencensus::stats::MeasureInt64 RpcServerCompletedRpcs();

namespace internal {
::opencensus::stats::MeasureDouble RpcClientApiLatency();
}

}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_FILTERS_CENSUS_MEASURES_H
