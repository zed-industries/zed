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

#ifndef GRPC_SRC_CORE_TSI_LOCAL_TRANSPORT_SECURITY_H
#define GRPC_SRC_CORE_TSI_LOCAL_TRANSPORT_SECURITY_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include "src/core/tsi/transport_security.h"
#include "src/core/tsi/transport_security_interface.h"

#define TSI_LOCAL_NUM_OF_PEER_PROPERTIES 1
#define TSI_LOCAL_PROCESS_ID_PEER_PROPERTY "process_id"

///
/// This method creates a local TSI handshaker instance.
///
///- self: address of local TSI handshaker instance to be returned from the
///  method.
///
/// It returns TSI_OK on success and an error status code on failure.
///
/// This handshaker is also being used as a minimalist handshaker for insecure
/// security connector. If this handshaker ever needs to do anything more that
/// does not fit with an insecure connector, we would need to add a separate
/// handshaker for insecure connectors.
///
tsi_result tsi_local_handshaker_create(tsi_handshaker** self);

#endif  // GRPC_SRC_CORE_TSI_LOCAL_TRANSPORT_SECURITY_H
