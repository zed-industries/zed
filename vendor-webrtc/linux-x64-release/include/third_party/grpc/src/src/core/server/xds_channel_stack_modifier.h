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

#ifndef GRPC_SRC_CORE_SERVER_XDS_CHANNEL_STACK_MODIFIER_H
#define GRPC_SRC_CORE_SERVER_XDS_CHANNEL_STACK_MODIFIER_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack_builder.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/useful.h"

namespace grpc_core {

// XdsChannelStackModifier allows for inserting xDS HTTP filters into the
// channel stack. It is registered to mutate the `ChannelStackBuilder` object
// via ChannelInit::Builder::RegisterPostProcessor.
class XdsChannelStackModifier final
    : public RefCounted<XdsChannelStackModifier> {
 public:
  explicit XdsChannelStackModifier(
      std::vector<const grpc_channel_filter*> filters)
      : filters_(std::move(filters)) {}
  void ModifyChannelStack(ChannelStackBuilder& builder);
  grpc_arg MakeChannelArg() const;
  static RefCountedPtr<XdsChannelStackModifier> GetFromChannelArgs(
      const grpc_channel_args& args);
  static absl::string_view ChannelArgName();
  static int ChannelArgsCompare(const XdsChannelStackModifier* a,
                                const XdsChannelStackModifier* b) {
    return QsortCompare(a, b);
  }

 private:
  std::vector<const grpc_channel_filter*> filters_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_SERVER_XDS_CHANNEL_STACK_MODIFIER_H
