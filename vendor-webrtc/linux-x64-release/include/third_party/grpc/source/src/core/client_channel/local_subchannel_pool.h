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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_LOCAL_SUBCHANNEL_POOL_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_LOCAL_SUBCHANNEL_POOL_H

#include <grpc/support/port_platform.h>

#include <map>

#include "src/core/client_channel/subchannel_pool_interface.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

// The local subchannel pool that is owned by a single channel. It doesn't
// support subchannel sharing with other channels by nature. Nor does it support
// subchannel retention when a subchannel is not used. The only real purpose of
// using this subchannel pool is to allow subchannel reuse within the channel
// when an incoming resolver update contains some addresses for which the
// channel has already created subchannels.
// Thread-unsafe.
class LocalSubchannelPool final : public SubchannelPoolInterface {
 public:
  LocalSubchannelPool() {}
  ~LocalSubchannelPool() override {}

  // Implements interface methods.
  // Thread-unsafe. Intended to be invoked within the client_channel work
  // serializer.
  RefCountedPtr<Subchannel> RegisterSubchannel(
      const SubchannelKey& key, RefCountedPtr<Subchannel> constructed) override;
  void UnregisterSubchannel(const SubchannelKey& key,
                            Subchannel* subchannel) override;
  RefCountedPtr<Subchannel> FindSubchannel(const SubchannelKey& key) override;

 private:
  // A map from subchannel key to subchannel.
  std::map<SubchannelKey, Subchannel*> subchannel_map_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_LOCAL_SUBCHANNEL_POOL_H
