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

#ifndef GRPC_SRC_CORE_HANDSHAKER_SECURITY_SECURITY_HANDSHAKER_H
#define GRPC_SRC_CORE_HANDSHAKER_SECURITY_SECURITY_HANDSHAKER_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include "absl/status/statusor.h"
#include "src/core/config/core_configuration.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/handshaker/handshaker.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/tsi/transport_security_interface.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

/// Creates a security handshaker using \a handshaker.
RefCountedPtr<Handshaker> SecurityHandshakerCreate(
    absl::StatusOr<tsi_handshaker*> handshaker,
    grpc_security_connector* connector, const ChannelArgs& args);

/// Registers security handshaker factories.
void SecurityRegisterHandshakerFactories(CoreConfiguration::Builder*);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_SECURITY_SECURITY_HANDSHAKER_H
