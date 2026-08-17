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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_POOL_INTERFACE_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_POOL_INTERFACE_H

#include <grpc/support/port_platform.h>

#include <string>

#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/useful.h"

namespace grpc_core {

class Subchannel;

// A key that can uniquely identify a subchannel.
class SubchannelKey final {
 public:
  SubchannelKey(const grpc_resolved_address& address, const ChannelArgs& args);

  SubchannelKey(const SubchannelKey& other) = default;
  SubchannelKey& operator=(const SubchannelKey& other) = default;
  SubchannelKey(SubchannelKey&& other) noexcept = default;
  SubchannelKey& operator=(SubchannelKey&& other) noexcept = default;

  bool operator<(const SubchannelKey& other) const {
    return Compare(other) < 0;
  }
  bool operator>(const SubchannelKey& other) const {
    return Compare(other) > 0;
  }
  bool operator==(const SubchannelKey& other) const {
    return Compare(other) == 0;
  }

  int Compare(const SubchannelKey& other) const;

  const grpc_resolved_address& address() const { return address_; }
  const ChannelArgs& args() const { return args_; }

  // Human-readable string suitable for logging.
  std::string ToString() const;

 private:
  grpc_resolved_address address_;
  ChannelArgs args_;
};

// Interface for subchannel pool.
// TODO(juanlishen): This refcounting mechanism may lead to memory leak.
// To solve that, we should force polling to flush any pending callbacks, then
// shut down safely. See https://github.com/grpc/grpc/issues/12560.
class SubchannelPoolInterface : public RefCounted<SubchannelPoolInterface> {
 public:
  SubchannelPoolInterface()
      : RefCounted(GRPC_TRACE_FLAG_ENABLED(subchannel_pool)
                       ? "SubchannelPoolInterface"
                       : nullptr) {}
  ~SubchannelPoolInterface() override {}

  static absl::string_view ChannelArgName();
  static int ChannelArgsCompare(const SubchannelPoolInterface* a,
                                const SubchannelPoolInterface* b) {
    return QsortCompare(a, b);
  }

  // Registers a subchannel against a key. Returns the subchannel registered
  // with \a key, which may be different from \a constructed because we reuse
  // (instead of update) any existing subchannel already registered with \a key.
  virtual RefCountedPtr<Subchannel> RegisterSubchannel(
      const SubchannelKey& key, RefCountedPtr<Subchannel> constructed) = 0;

  // Removes the registered subchannel found by \a key.
  virtual void UnregisterSubchannel(const SubchannelKey& key,
                                    Subchannel* subchannel) = 0;

  // Finds the subchannel registered for the given subchannel key. Returns NULL
  // if no such channel exists. Thread-safe.
  virtual RefCountedPtr<Subchannel> FindSubchannel(
      const SubchannelKey& key) = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_POOL_INTERFACE_H
