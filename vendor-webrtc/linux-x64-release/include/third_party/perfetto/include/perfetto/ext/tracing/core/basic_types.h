/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_TRACING_CORE_BASIC_TYPES_H_
#define INCLUDE_PERFETTO_EXT_TRACING_CORE_BASIC_TYPES_H_

#include <stddef.h>
#include <stdint.h>

#include "perfetto/ext/base/sys_types.h"

namespace perfetto {

// Unique within the scope of the tracing service.
using TracingSessionID = uint64_t;

// Unique within the scope of the tracing service.
using ProducerID = uint16_t;

// Unique within the scope of the tracing service.
using DataSourceInstanceID = uint64_t;

// Unique within the scope of a Producer.
using WriterID = uint16_t;

// Unique within the scope of the tracing service.
using FlushRequestID = uint64_t;

// Combines Producer and Writer ID in one word which can be used as key for
// hashtables and other data structures.
using ProducerAndWriterID = uint32_t;

inline ProducerAndWriterID MkProducerAndWriterID(ProducerID p, WriterID w) {
  static_assert(
      sizeof(ProducerID) + sizeof(WriterID) == sizeof(ProducerAndWriterID),
      "MkProducerAndWriterID() and GetProducerAndWriterID() need updating");
  return (static_cast<ProducerAndWriterID>(p) << (sizeof(WriterID) * 8)) | w;
}

inline void GetProducerAndWriterID(ProducerAndWriterID x,
                                   ProducerID* p,
                                   WriterID* w) {
  static constexpr auto mask = (1ull << (sizeof(WriterID) * 8)) - 1;
  *w = static_cast<WriterID>(x & mask);
  *p = static_cast<ProducerID>(x >> (sizeof(WriterID) * 8));
}

// We need one FD per producer and we are not going to be able to keep > 64k FDs
// open in the service.
static constexpr ProducerID kMaxProducerID = static_cast<ProducerID>(-1);

// 1024 Writers per producer seems a resonable bound. This reduces the ability
// to memory-DoS the service by having to keep track of too many writer IDs.
static constexpr WriterID kMaxWriterID = static_cast<WriterID>((1 << 10) - 1);

// Unique within the scope of a {ProducerID, WriterID} tuple.
using ChunkID = uint32_t;
static constexpr ChunkID kMaxChunkID = static_cast<ChunkID>(-1);

// Unique within the scope of the tracing service.
using BufferID = uint16_t;

// Target buffer ID for SharedMemoryArbiter. Values up to max uint16_t are
// equivalent to a bound BufferID. Values above max uint16_t are reservation IDs
// for the target buffer of a startup trace writer. Reservation IDs will be
// translated to actual BufferIDs after they are bound by
// SharedMemoryArbiter::BindStartupTargetBuffer().
// TODO(mohitms): Delete this type and use `struct {uint16 ; uint16;}` instead.
using MaybeUnboundBufferID = uint32_t;

// Keep this in sync with SharedMemoryABI::PageHeader::target_buffer.
static constexpr BufferID kMaxTraceBufferID = static_cast<BufferID>(-1);

// Unique within the scope of a tracing session.
using PacketSequenceID = uint32_t;
// Used for extra packets emitted by the service, such as statistics.
static constexpr PacketSequenceID kServicePacketSequenceID = 1;
static constexpr PacketSequenceID kMaxPacketSequenceID =
    static_cast<PacketSequenceID>(-1);

constexpr uint32_t kDefaultFlushTimeoutMs = 5000;

// The special id 0xffff..ffff represents the tracing session with the highest
// bugreport score. This is used for CloneSession(kBugreportSessionId).
constexpr TracingSessionID kBugreportSessionId =
    static_cast<TracingSessionID>(-1);

// The ID of a machine in a multi-machine tracing session.
using MachineID = base::MachineID;
constexpr MachineID kDefaultMachineID = base::kDefaultMachineID;

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACING_CORE_BASIC_TYPES_H_
