//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_TEST_CPP_INTEROP_XDS_STATS_WATCHER_H
#define GRPC_TEST_CPP_INTEROP_XDS_STATS_WATCHER_H

#include <grpcpp/grpcpp.h>

#include <atomic>
#include <chrono>
#include <condition_variable>
#include <deque>
#include <map>
#include <mutex>
#include <set>
#include <sstream>
#include <string>
#include <thread>
#include <unordered_set>
#include <vector>

#include "absl/status/status.h"
#include "absl/types/span.h"
#include "src/proto/grpc/testing/empty.pb.h"
#include "src/proto/grpc/testing/messages.pb.h"

namespace grpc {
namespace testing {

class XdsStatsWatcher;

struct AsyncClientCallResult {
  Empty empty_response;
  SimpleResponse simple_response;
  Status status;
  int saved_request_id;
  ClientConfigureRequest::RpcType rpc_type;
};

struct StatsWatchers {
  // Unique ID for each outgoing RPC
  int global_request_id = 0;
  // Unique ID for each outgoing RPC by RPC method type
  std::map<int, int> global_request_id_by_type;
  // Stores a set of watchers that should be notified upon outgoing RPC
  // completion
  std::set<XdsStatsWatcher*> watchers;
  // Global watcher for accumululated stats.
  XdsStatsWatcher* global_watcher;
  // Mutex for global_request_id and watchers
  std::mutex mu;
};

/// Records the remote peer distribution for a given range of RPCs.
class XdsStatsWatcher {
 public:
  XdsStatsWatcher(int start_id, int end_id,
                  absl::Span<const std::string> metadata_keys);

  // Upon the completion of an RPC, we will look at the request_id, the
  // rpc_type, and the peer the RPC was sent to in order to count
  // this RPC into the right stats bin.
  void RpcCompleted(
      const AsyncClientCallResult& call, const std::string& peer,
      const std::multimap<grpc::string_ref, grpc::string_ref>& initial_metadata,
      const std::multimap<grpc::string_ref, grpc::string_ref>&
          trailing_metadata);

  LoadBalancerStatsResponse WaitForRpcStatsResponse(int timeout_sec);

  void GetCurrentRpcStats(LoadBalancerAccumulatedStatsResponse* response,
                          StatsWatchers* stats_watchers);

 private:
  int start_id_;
  int end_id_;
  int rpcs_needed_;
  int no_remote_peer_ = 0;
  std::map<int, int> no_remote_peer_by_type_;
  // A map of stats keyed by peer name.
  std::map<std::string, int> rpcs_by_peer_;
  // A two-level map of stats keyed at top level by RPC method and second level
  // by peer name.
  std::map<int, std::map<std::string, int>> rpcs_by_type_;
  // Storing accumulated stats in the response proto format.
  LoadBalancerAccumulatedStatsResponse accumulated_stats_;
  std::mutex m_;
  std::condition_variable cv_;
  std::unordered_set<std::string> metadata_keys_;
  bool include_all_metadata_ = false;
  std::map<std::string, LoadBalancerStatsResponse::MetadataByPeer>
      metadata_by_peer_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_INTEROP_XDS_STATS_WATCHER_H
