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

#ifndef GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_HANDSHAKER_PRIVATE_H
#define GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_HANDSHAKER_PRIVATE_H

#include <grpc/support/port_platform.h>

#include "src/core/tsi/alts/handshaker/alts_handshaker_client.h"

namespace grpc_core {
namespace internal {

///
/// Unsafe, use for testing only.

alts_handshaker_client* alts_tsi_handshaker_get_client_for_testing(
    alts_tsi_handshaker* handshaker);

bool alts_tsi_handshaker_get_has_sent_start_message_for_testing(
    alts_tsi_handshaker* handshaker);

void alts_tsi_handshaker_set_client_vtable_for_testing(
    alts_tsi_handshaker* handshaker, alts_handshaker_client_vtable* vtable);

bool alts_tsi_handshaker_get_is_client_for_testing(
    alts_tsi_handshaker* handshaker);

void alts_handshaker_client_set_grpc_caller_for_testing(
    alts_handshaker_client* client, alts_grpc_caller caller);

grpc_byte_buffer* alts_handshaker_client_get_send_buffer_for_testing(
    alts_handshaker_client* client);

grpc_byte_buffer** alts_handshaker_client_get_recv_buffer_addr_for_testing(
    alts_handshaker_client* client);

grpc_metadata_array* alts_handshaker_client_get_initial_metadata_for_testing(
    alts_handshaker_client* client);

void alts_handshaker_client_set_recv_bytes_for_testing(
    alts_handshaker_client* client, grpc_slice* recv_bytes);

void alts_handshaker_client_check_fields_for_testing(
    alts_handshaker_client* client, tsi_handshaker_on_next_done_cb cb,
    void* user_data, bool has_sent_start_message, grpc_slice* recv_bytes);

void alts_handshaker_client_set_fields_for_testing(
    alts_handshaker_client* client, alts_tsi_handshaker* handshaker,
    tsi_handshaker_on_next_done_cb cb, void* user_data,
    grpc_byte_buffer* recv_buffer, bool inject_read_failure);

void alts_handshaker_client_set_vtable_for_testing(
    alts_handshaker_client* client, alts_handshaker_client_vtable* vtable);

alts_tsi_handshaker* alts_handshaker_client_get_handshaker_for_testing(
    alts_handshaker_client* client);

void alts_handshaker_client_set_cb_for_testing(
    alts_handshaker_client* client, tsi_handshaker_on_next_done_cb cb);

grpc_closure* alts_handshaker_client_get_closure_for_testing(
    alts_handshaker_client* client);

void alts_handshaker_client_on_status_received_for_testing(
    alts_handshaker_client* client, grpc_status_code status,
    grpc_error_handle error);

void alts_handshaker_client_ref_for_testing(alts_handshaker_client* c);

}  // namespace internal
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_TSI_ALTS_HANDSHAKER_ALTS_TSI_HANDSHAKER_PRIVATE_H
