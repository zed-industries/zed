/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_SORTER_TRACE_TOKEN_BUFFER_H_
#define SRC_TRACE_PROCESSOR_SORTER_TRACE_TOKEN_BUFFER_H_

#include <cstdint>
#include <limits>
#include <optional>
#include <utility>
#include <vector>

#include "perfetto/base/compiler.h"
#include "perfetto/ext/base/circular_queue.h"
#include "perfetto/ext/base/utils.h"
#include "perfetto/trace_processor/trace_blob.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/util/bump_allocator.h"

namespace perfetto {
namespace trace_processor {

// Helper class which stores tokenized objects while the corresponding events
// are being sorted by TraceSorter.
//
// This class intrusively compresses the tokenized objects as much as possible
// to reduce their memory footprint. This is important to reduce the peak memory
// usage of TraceProcessor which is always hit at some point during sorting.
// The tokenized objects make up the vast majority of this peak so we trade the
// complexity in this class for big reductions in the peak use.
//
// go/perfetto-tp-memory-use gives an overview of trace processor memory usage.
class TraceTokenBuffer {
 public:
  // Identifier returned when appending items to this buffer. This id can
  // later by passed to |Extract| to retrieve the event.
  struct Id {
    // The allocation id of the object in the buffer.
    BumpAllocator::AllocId alloc_id;
  };

  // Appends an object of type |T| to the token buffer. Returns an id for
  // looking up the object later using |Extract|.
  template <typename T>
  PERFETTO_WARN_UNUSED_RESULT Id Append(T object) {
    static_assert(sizeof(T) % 8 == 0, "Size must be a multiple of 8");
    static_assert(alignof(T) == 8, "Alignment must be 8");
    BumpAllocator::AllocId id = AllocAndResizeInternedVectors(sizeof(T));
    new (allocator_.GetPointer(id)) T(std::move(object));
    return Id{id};
  }
  PERFETTO_WARN_UNUSED_RESULT Id Append(TrackEventData);
  PERFETTO_WARN_UNUSED_RESULT Id Append(TracePacketData data) {
    // While in theory we could add a special case for TracePacketData, the
    // judgement call we make is that the code complexity does not justify the
    // micro-performance gain you might hope to see by avoiding the few if
    // conditions in the |TracePacketData| path.
    return Append(TrackEventData(std::move(data)));
  }

  // Gets a pointer an object of type |T| from the token buffer using an id
  // previously returned by |Append|. This type *must* match the type added
  // using Append. Mismatching types will caused undefined behaviour.
  template <typename T>
  PERFETTO_WARN_UNUSED_RESULT T* Get(Id id) {
    return static_cast<T*>(allocator_.GetPointer(id.alloc_id));
  }

  // Extracts an object of type |T| from the token buffer using an id previously
  // returned by |Append|. This type *must* match the type added using Append.
  // Mismatching types will caused undefined behaviour.
  template <typename T>
  PERFETTO_WARN_UNUSED_RESULT T Extract(Id id) {
    T* typed_ptr = static_cast<T*>(allocator_.GetPointer(id.alloc_id));
    T object(std::move(*typed_ptr));
    typed_ptr->~T();
    allocator_.Free(id.alloc_id);
    return object;
  }

  // Returns the "past-the-end" id from the underlying allocator.
  // The main use of this function is to provide an id which is greater than
  // all ids previously returned by |Append|.
  //
  // This is similar to the |end()| function in standard library vector classes.
  BumpAllocator::AllocId PastTheEndAllocId() {
    return allocator_.PastTheEndId();
  }

  // Attempts to free any memory retained by this buffer and the underlying
  // allocator. The amount of memory free is implementation defined.
  void FreeMemory();

 private:
  struct BlobWithOffset {
    TraceBlob* blob;
    size_t offset_in_blob;
  };
  using InternedIndex = size_t;
  using BlobWithOffsets = std::vector<BlobWithOffset>;
  using SequenceStates = std::vector<PacketSequenceStateGeneration*>;

  // Functions to intern TraceBlob and PacketSequenceStateGeneration: as these
  // are often shared between packets, we can significantly reduce memory use
  // by only storing them once.
  uint32_t InternTraceBlob(InternedIndex, const TraceBlobView&);
  uint16_t InternSeqState(InternedIndex, RefPtr<PacketSequenceStateGeneration>);
  uint32_t AddTraceBlob(InternedIndex, const TraceBlobView&);

  BumpAllocator::AllocId AllocAndResizeInternedVectors(uint32_t size);
  InternedIndex GetInternedIndex(BumpAllocator::AllocId);

  BumpAllocator allocator_;
  base::CircularQueue<BlobWithOffsets> interned_blobs_;
  base::CircularQueue<SequenceStates> interned_seqs_;
};

// GCC7 does not like us declaring these inside the class so define these
// out-of-line.
template <>
PERFETTO_WARN_UNUSED_RESULT TrackEventData
    TraceTokenBuffer::Extract<TrackEventData>(Id);
template <>
PERFETTO_WARN_UNUSED_RESULT inline TracePacketData
TraceTokenBuffer::Extract<TracePacketData>(Id id) {
  // See the comment in Append(TracePacketData) for why we do this.
  return Extract<TrackEventData>(id).trace_packet_data;
}

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_SORTER_TRACE_TOKEN_BUFFER_H_
