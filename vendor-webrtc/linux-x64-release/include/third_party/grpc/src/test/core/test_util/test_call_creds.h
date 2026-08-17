//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TEST_UTIL_TEST_CALL_CREDS_H
#define GRPC_TEST_CORE_TEST_UTIL_TEST_CALL_CREDS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>

#include <string>

#include "absl/status/statusor.h"
#include "src/core/credentials/call/call_credentials.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

class grpc_md_only_test_credentials : public grpc_call_credentials {
 public:
  grpc_md_only_test_credentials(const char* md_key, const char* md_value)
      : grpc_call_credentials(GRPC_SECURITY_NONE),
        key_(grpc_core::Slice::FromCopiedString(md_key)),
        value_(grpc_core::Slice::FromCopiedString(md_value)) {}

  void Orphaned() override {}

  grpc_core::ArenaPromise<absl::StatusOr<grpc_core::ClientMetadataHandle>>
  GetRequestMetadata(grpc_core::ClientMetadataHandle initial_metadata,
                     const GetRequestMetadataArgs* args) override;

  std::string debug_string() override { return "MD only Test Credentials"; }

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

 private:
  int cmp_impl(const grpc_call_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return grpc_core::QsortCompare(
        static_cast<const grpc_call_credentials*>(this), other);
  }

  grpc_core::Slice key_;
  grpc_core::Slice value_;
};

// Metadata-only credentials with the specified key and value where
// asynchronicity can be simulated for testing.
grpc_call_credentials* grpc_md_only_test_credentials_create(
    const char* md_key, const char* md_value);

#endif  // GRPC_TEST_CORE_TEST_UTIL_TEST_CALL_CREDS_H
