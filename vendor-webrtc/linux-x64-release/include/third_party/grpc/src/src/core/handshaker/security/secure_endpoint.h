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

#ifndef GRPC_SRC_CORE_HANDSHAKER_SECURITY_SECURE_ENDPOINT_H
#define GRPC_SRC_CORE_HANDSHAKER_SECURITY_SECURE_ENDPOINT_H

#include <grpc/grpc.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/util/orphanable.h"

// Integer. The size of a read at which the secure endpoint will offload
// decryption to an event engine thread.
#define GRPC_ARG_DECRYPTION_OFFLOAD_THRESHOLD \
  "grpc.secure_endpoint.decryption_offload_threshold"
#define GRPC_ARG_ENCRYPTION_OFFLOAD_THRESHOLD \
  "grpc.secure_endpoint.encryption_offload_threshold"
#define GRPC_ARG_ENCRYPTION_OFFLOAD_MAX_BUFFERED_WRITES \
  "grpc.secure_endpoint.encryption_offload_max_buffered_writes"

// Takes ownership of protector, zero_copy_protector, and to_wrap, and refs
// leftover_slices. If zero_copy_protector is not NULL, protector will never be
// used.
grpc_core::OrphanablePtr<grpc_endpoint> grpc_secure_endpoint_create(
    struct tsi_frame_protector* protector,
    struct tsi_zero_copy_grpc_protector* zero_copy_protector,
    grpc_core::OrphanablePtr<grpc_endpoint> to_wrap,
    grpc_slice* leftover_slices, size_t leftover_nslices,
    const grpc_core::ChannelArgs& channel_args);

grpc_core::OrphanablePtr<grpc_endpoint> grpc_legacy_secure_endpoint_create(
    struct tsi_frame_protector* protector,
    struct tsi_zero_copy_grpc_protector* zero_copy_protector,
    grpc_core::OrphanablePtr<grpc_endpoint> to_wrap,
    grpc_slice* leftover_slices, const grpc_channel_args* channel_args,
    size_t leftover_nslices);

#endif  // GRPC_SRC_CORE_HANDSHAKER_SECURITY_SECURE_ENDPOINT_H
