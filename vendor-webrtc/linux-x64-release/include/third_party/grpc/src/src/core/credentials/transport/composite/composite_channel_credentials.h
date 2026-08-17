//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_COMPOSITE_COMPOSITE_CHANNEL_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_COMPOSITE_COMPOSITE_CHANNEL_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/support/port_platform.h>

#include <algorithm>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/statusor.h"
#include "src/core/credentials/call/call_credentials.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

class grpc_composite_channel_credentials : public grpc_channel_credentials {
 public:
  grpc_composite_channel_credentials(
      grpc_core::RefCountedPtr<grpc_channel_credentials> channel_creds,
      grpc_core::RefCountedPtr<grpc_call_credentials> call_creds)
      : inner_creds_(std::move(channel_creds)),
        call_creds_(std::move(call_creds)) {}

  ~grpc_composite_channel_credentials() override = default;

  grpc_core::RefCountedPtr<grpc_channel_credentials>
  duplicate_without_call_credentials() override {
    return inner_creds_->duplicate_without_call_credentials();
  }

  grpc_core::RefCountedPtr<grpc_channel_security_connector>
  create_security_connector(
      grpc_core::RefCountedPtr<grpc_call_credentials> call_creds,
      const char* target, grpc_core::ChannelArgs* args) override;

  grpc_core::ChannelArgs update_arguments(
      grpc_core::ChannelArgs args) override {
    return inner_creds_->update_arguments(std::move(args));
  }

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

  const grpc_channel_credentials* inner_creds() const {
    return inner_creds_.get();
  }
  const grpc_call_credentials* call_creds() const { return call_creds_.get(); }
  grpc_call_credentials* mutable_call_creds() { return call_creds_.get(); }

 private:
  int cmp_impl(const grpc_channel_credentials* other) const override {
    auto* o = static_cast<const grpc_composite_channel_credentials*>(other);
    int r = inner_creds_->cmp(o->inner_creds_.get());
    if (r != 0) return r;
    return call_creds_->cmp(o->call_creds_.get());
  }

  grpc_core::RefCountedPtr<grpc_channel_credentials> inner_creds_;
  grpc_core::RefCountedPtr<grpc_call_credentials> call_creds_;
};

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_COMPOSITE_COMPOSITE_CHANNEL_CREDENTIALS_H
