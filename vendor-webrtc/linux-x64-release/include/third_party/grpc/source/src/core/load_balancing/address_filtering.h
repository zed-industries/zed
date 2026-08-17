//
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_ADDRESS_FILTERING_H
#define GRPC_SRC_CORE_LOAD_BALANCING_ADDRESS_FILTERING_H

#include <grpc/support/port_platform.h>

#include <map>
#include <memory>
#include <utility>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/resolver/endpoint_addresses.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_string.h"

// The resolver returns a flat list of addresses.  When a hierarchy of
// LB policies is in use, each leaf of the hierarchy will need a
// different subset of those addresses.  This library provides a
// mechanism for determining which address is passed to which leaf
// policy.
//
// Each address will have an associated path that indicates which child
// it should be sent to at each level of the hierarchy to wind up at the
// right leaf policy.  Each LB policy will look at the first element of
// the path of each address to determine which child to send the address
// to.  It will then remove that first element when passing the address
// down to its child.
//
// For example, consider the following LB policy hierarchy:
//
// - priority
//   - child0 (weighted_target)
//     - localityA (round_robin)
//     - localityB (round_robin)
//   - child1 (weighted_target)
//     - localityC (round_robin)
//     - localityD (round_robin)
//
// Now consider the following addresses:
// - 10.0.0.1:80 path=["child0", "localityA"]
// - 10.0.0.2:80 path=["child0", "localityB"]
// - 10.0.0.3:80 path=["child1", "localityC"]
// - 10.0.0.4:80 path=["child1", "localityD"]
//
// The priority policy will split this up into two lists, one for each
// of its children:
// - child0:
//   - 10.0.0.1:80 path=["localityA"]
//   - 10.0.0.2:80 path=["localityB"]
// - child1:
//   - 10.0.0.3:80 path=["localityC"]
//   - 10.0.0.4:80 path=["localityD"]
//
// The weighted_target policy for child0 will split its list up into two
// lists, one for each of its children:
// - localityA:
//   - 10.0.0.1:80 path=[]
// - localityB:
//   - 10.0.0.2:80 path=[]
//
// Similarly, the weighted_target policy for child1 will split its list
// up into two lists, one for each of its children:
// - localityC:
//   - 10.0.0.3:80 path=[]
// - localityD:
//   - 10.0.0.4:80 path=[]

namespace grpc_core {

// An address channel arg containing the hierarchical path
// to be associated with the address.
class HierarchicalPathArg final : public RefCounted<HierarchicalPathArg> {
 public:
  explicit HierarchicalPathArg(std::vector<RefCountedStringValue> path)
      : path_(std::move(path)) {}

  // Channel arg traits methods.
  static absl::string_view ChannelArgName();
  static int ChannelArgsCompare(const HierarchicalPathArg* a,
                                const HierarchicalPathArg* b);

  const std::vector<RefCountedStringValue>& path() const { return path_; }

 private:
  std::vector<RefCountedStringValue> path_;
};

// A map from the next path element to the endpoint addresses that fall
// under that path element.
using HierarchicalAddressMap =
    std::map<RefCountedStringValue, std::shared_ptr<EndpointAddressesIterator>,
             RefCountedStringValueLessThan>;

// Splits up the addresses into a separate list for each child.
absl::StatusOr<HierarchicalAddressMap> MakeHierarchicalAddressMap(
    absl::StatusOr<std::shared_ptr<EndpointAddressesIterator>> addresses);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_ADDRESS_FILTERING_H
