//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CHANNELZ_CHANNELZ_REGISTRY_H
#define GRPC_SRC_CORE_CHANNELZ_CHANNELZ_REGISTRY_H

#include <grpc/support/port_platform.h>

#include <cstdint>
#include <map>
#include <string>

#include "absl/base/thread_annotations.h"
#include "src/core/channelz/channelz.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {
namespace channelz {

// singleton registry object to track all objects that are needed to support
// channelz bookkeeping. All objects share globally distributed uuids.
class ChannelzRegistry final {
 public:
  static void Register(BaseNode* node) {
    return Default()->InternalRegister(node);
  }
  static void Unregister(intptr_t uuid) { Default()->InternalUnregister(uuid); }
  static RefCountedPtr<BaseNode> Get(intptr_t uuid) {
    return Default()->InternalGet(uuid);
  }

  // Returns the allocated JSON string that represents the proto
  // GetTopChannelsResponse as per channelz.proto.
  static std::string GetTopChannels(intptr_t start_channel_id) {
    return Default()->InternalGetTopChannels(start_channel_id);
  }

  // Returns the allocated JSON string that represents the proto
  // GetServersResponse as per channelz.proto.
  static std::string GetServers(intptr_t start_server_id) {
    return Default()->InternalGetServers(start_server_id);
  }

  // Test only helper function to dump the JSON representation to std out.
  // This can aid in debugging channelz code.
  static void LogAllEntities() { Default()->InternalLogAllEntities(); }

  // Test only helper function to reset to initial state.
  static void TestOnlyReset() {
    auto* p = Default();
    MutexLock lock(&p->mu_);
    p->node_map_.clear();
    p->uuid_generator_ = 0;
  }

 private:
  // Returned the singleton instance of ChannelzRegistry;
  static ChannelzRegistry* Default();

  // globally registers an Entry. Returns its unique uuid
  void InternalRegister(BaseNode* node);

  // globally unregisters the object that is associated to uuid. Also does
  // sanity check that an object doesn't try to unregister the wrong type.
  void InternalUnregister(intptr_t uuid);

  // if object with uuid has previously been registered as the correct type,
  // returns the void* associated with that uuid. Else returns nullptr.
  RefCountedPtr<BaseNode> InternalGet(intptr_t uuid);

  std::string InternalGetTopChannels(intptr_t start_channel_id);
  std::string InternalGetServers(intptr_t start_server_id);

  void InternalLogAllEntities();

  // protects members
  Mutex mu_;
  std::map<intptr_t, BaseNode*> node_map_ ABSL_GUARDED_BY(mu_);
  intptr_t uuid_generator_ ABSL_GUARDED_BY(mu_) = 0;
};

}  // namespace channelz
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CHANNELZ_CHANNELZ_REGISTRY_H
