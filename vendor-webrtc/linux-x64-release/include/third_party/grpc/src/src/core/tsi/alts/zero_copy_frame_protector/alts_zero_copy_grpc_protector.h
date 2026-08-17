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

#ifndef GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_ZERO_COPY_GRPC_PROTECTOR_H
#define GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_ZERO_COPY_GRPC_PROTECTOR_H

#include <grpc/support/port_platform.h>
#include <stdbool.h>

#include "src/core/tsi/alts/crypt/gsec.h"
#include "src/core/tsi/transport_security_interface.h"

//
// This method creates an ALTS zero-copy grpc protector.
//
//- key_factory: a key factory that creates keys to seal/unseal frames.
//  it self-contains the information such as key length and whether rekey is
//  supported.
//- is_client: a flag indicating if the protector will be used at client or
//  server side.
//- is_integrity_only: a flag indicating if the protector instance will be
//  used for integrity-only or privacy-integrity mode.
//- enable_extra_copy: a flag indicating if the protector instance does one
//  extra memory copy during the protect operation for integrity_only mode.
//  For the unprotect operation, it is still zero-copy. If application intends
//  to modify the data buffer after the protect operation, we can turn on this
//  mode to avoid integrity check failure.
//- max_protected_frame_size: an in/out parameter indicating max frame size
//  to be used by the protector. If it is nullptr, the default frame size will
//  be used. Otherwise, the provided frame size will be adjusted (if not
//  falling into a valid frame range) and used.
//- protector: a pointer to the zero-copy protector returned from the method.
//
// This method returns TSI_OK on success or a specific error code otherwise.
//
tsi_result alts_zero_copy_grpc_protector_create(
    const grpc_core::GsecKeyFactoryInterface& key_factory, bool is_client,
    bool is_integrity_only, bool enable_extra_copy,
    size_t* max_protected_frame_size, tsi_zero_copy_grpc_protector** protector);

#endif  // GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_ZERO_COPY_GRPC_PROTECTOR_H
