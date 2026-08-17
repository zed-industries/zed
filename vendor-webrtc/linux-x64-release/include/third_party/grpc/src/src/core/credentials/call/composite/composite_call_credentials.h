//
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
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_CALL_COMPOSITE_COMPOSITE_CALL_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_CALL_COMPOSITE_COMPOSITE_CALL_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>

#include <algorithm>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/statusor.h"
#include "src/core/credentials/call/call_credentials.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

class grpc_composite_call_credentials : public grpc_call_credentials {
 public:
  using CallCredentialsList =
      std::vector<grpc_core::RefCountedPtr<grpc_call_credentials>>;

  grpc_composite_call_credentials(
      grpc_core::RefCountedPtr<grpc_call_credentials> creds1,
      grpc_core::RefCountedPtr<grpc_call_credentials> creds2);
  ~grpc_composite_call_credentials() override = default;

  void Orphaned() override { inner_.clear(); }

  grpc_core::ArenaPromise<absl::StatusOr<grpc_core::ClientMetadataHandle>>
  GetRequestMetadata(grpc_core::ClientMetadataHandle initial_metadata,
                     const GetRequestMetadataArgs* args) override;

  grpc_security_level min_security_level() const override {
    return min_security_level_;
  }

  const CallCredentialsList& inner() const { return inner_; }
  std::string debug_string() override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

 private:
  int cmp_impl(const grpc_call_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return grpc_core::QsortCompare(
        static_cast<const grpc_call_credentials*>(this), other);
  }

  void push_to_inner(grpc_core::RefCountedPtr<grpc_call_credentials> creds,
                     bool is_composite);
  grpc_security_level min_security_level_;
  CallCredentialsList inner_;
};

#endif  // GRPC_SRC_CORE_CREDENTIALS_CALL_COMPOSITE_COMPOSITE_CALL_CREDENTIALS_H
