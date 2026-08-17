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

#ifndef GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_GRPC_PRIVACY_INTEGRITY_RECORD_PROTOCOL_H
#define GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_GRPC_PRIVACY_INTEGRITY_RECORD_PROTOCOL_H

#include <grpc/support/port_platform.h>
#include <stdbool.h>

#include "src/core/tsi/alts/crypt/gsec.h"
#include "src/core/tsi/alts/zero_copy_frame_protector/alts_grpc_record_protocol.h"

///
/// This method creates a privacy-integrity alts_grpc_record_protocol instance,
/// given a gsec_aead_crypter instance and a flag indicating if the created
/// instance will be used at the client or server side. The ownership of
/// gsec_aead_crypter instance is transferred to this new object.
///
///- crypter: a gsec_aead_crypter instance used to perform AEAD decryption.
///- is_client: a flag indicating if the alts_grpc_record_protocol instance will
///  be used at the client or server side.
///- rp: an alts_grpc_record_protocol instance to be returned from
///  the method.
///
/// This method returns TSI_OK in case of success or a specific error code in
/// case of failure.
///
tsi_result alts_grpc_privacy_integrity_record_protocol_create(
    gsec_aead_crypter* crypter, size_t overflow_size, bool is_client,
    bool is_protect, alts_grpc_record_protocol** rp);

#endif  // GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_GRPC_PRIVACY_INTEGRITY_RECORD_PROTOCOL_H
