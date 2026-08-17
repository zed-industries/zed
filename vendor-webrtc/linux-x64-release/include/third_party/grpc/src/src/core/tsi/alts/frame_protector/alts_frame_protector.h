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

#ifndef GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_FRAME_PROTECTOR_H
#define GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_FRAME_PROTECTOR_H

#include <grpc/support/port_platform.h>
#include <stdbool.h>

#include "src/core/tsi/transport_security_interface.h"

typedef struct alts_frame_protector alts_frame_protector;

///
/// TODO: Add a parameter to the interface to support the use of
/// different record protocols within a frame protector.
///
/// This method creates a frame protector.
///
///- key: a symmetric key used to seal/unseal frames.
///- key_size: the size of symmetric key.
///- is_client: a flag indicating if the frame protector will be used at client
///  (is_client = true) or server (is_client = false) side.
///- is_rekey: a flag indicating if the frame protector will use an AEAD with
///  rekeying.
///- max_protected_frame_size: an in/out parameter indicating max frame size
///  to be used by the frame protector. If it is nullptr, the default frame
///  size will be used. Otherwise, the provided frame size will be adjusted (if
///  not falling into a valid frame range) and used.
///- self: a pointer to the frame protector returned from the method.
///
/// This method returns TSI_OK on success and TSI_INTERNAL_ERROR otherwise.
///
tsi_result alts_create_frame_protector(const uint8_t* key, size_t key_size,
                                       bool is_client, bool is_rekey,
                                       size_t* max_protected_frame_size,
                                       tsi_frame_protector** self);

#endif  // GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_FRAME_PROTECTOR_H
