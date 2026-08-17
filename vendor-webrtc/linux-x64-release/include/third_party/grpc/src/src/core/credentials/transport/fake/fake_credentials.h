//
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
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_FAKE_FAKE_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_FAKE_FAKE_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/support/port_platform.h>

#include <string>

#include "absl/status/statusor.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

#define GRPC_ARG_FAKE_SECURITY_EXPECTED_TARGETS \
  "grpc.fake_security.expected_targets"

// -- Fake transport security credentials. --

class grpc_fake_channel_credentials final : public grpc_channel_credentials {
 public:
  grpc_core::RefCountedPtr<grpc_channel_security_connector>
  create_security_connector(
      grpc_core::RefCountedPtr<grpc_call_credentials> call_creds,
      const char* target, grpc_core::ChannelArgs* args) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

 private:
  int cmp_impl(const grpc_channel_credentials* other) const override;
};

class grpc_fake_server_credentials final : public grpc_server_credentials {
 public:
  grpc_core::RefCountedPtr<grpc_server_security_connector>
  create_security_connector(const grpc_core::ChannelArgs& /*args*/) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }
};

// Creates a fake transport security credentials object for testing.
grpc_channel_credentials* grpc_fake_transport_security_credentials_create(void);

// Creates a fake server transport security credentials object for testing.
grpc_server_credentials* grpc_fake_transport_security_server_credentials_create(
    void);

// Used to verify the target names given to the fake transport security
// connector.
//
// The syntax of \a expected_targets by example:
// For LB channels:
//     "backend_target_1,backend_target_2,...;lb_target_1,lb_target_2,..."
// For regular channels:
//     "backend_target_1,backend_target_2,..."
//
// That is to say, LB channels have a heading list of LB targets separated from
// the list of backend targets by a semicolon. For non-LB channels, only the
// latter is present.
grpc_arg grpc_fake_transport_expected_targets_arg(char* expected_targets);

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_FAKE_FAKE_CREDENTIALS_H
