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

#ifndef SRC_TRACING_CORE_TRACE_WRITER_IMPL_H_
#define SRC_TRACING_CORE_TRACE_WRITER_IMPL_H_

#include <cstdint>
#include <functional>
#include <memory>

#include "perfetto/base/proc_utils.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/shared_memory_abi.h"
#include "perfetto/ext/tracing/core/shared_memory_arbiter.h"
#include "perfetto/ext/tracing/core/trace_writer.h"
#include "perfetto/protozero/contiguous_memory_range.h"
#include "perfetto/protozero/message_handle.h"
#include "perfetto/protozero/root_message.h"
#include "perfetto/protozero/scattered_stream_writer.h"
#include "perfetto/tracing/buffer_exhausted_policy.h"
#include "src/tracing/core/patch_list.h"

namespace perfetto {

class SharedMemoryArbiterImpl;

// See //include/perfetto/ext/tracing/core/trace_writer.h for docs.
//
// Locking will happen only when a chunk is exhausted and a new one is
// acquired from the arbiter.
//
// TODO: TraceWriter needs to keep the shared memory buffer alive (refcount?).
// Otherwise if the shared memory buffer goes away (e.g. the Service crashes)
// the TraceWriter will keep writing into unmapped memory.
//
class TraceWriterImpl : public TraceWriter,
                        public protozero::MessageFinalizationListener,
                        public protozero::ScatteredStreamWriter::Delegate {
 public:
  // TracePacketHandle is defined in trace_writer.h
  TraceWriterImpl(SharedMemoryArbiterImpl*,
                  WriterID,
                  MaybeUnboundBufferID buffer_id,
                  BufferExhaustedPolicy);
  ~TraceWriterImpl() override;

  // TraceWriter implementation. See documentation in trace_writer.h.
  TracePacketHandle NewTracePacket() override;
  void FinishTracePacket() override;
  // Commits the data pending for the current chunk into the shared memory
  // buffer and sends a CommitDataRequest() to the service.
  // TODO(primiano): right now the |callback| will be called on the IPC thread.
  // This is fine in the current single-thread scenario, but long-term
  // trace_writer_impl.cc should be smarter and post it on the right thread.
  void Flush(std::function<void()> callback = {}) override;
  WriterID writer_id() const override;
  uint64_t written() const override {
    return protobuf_stream_writer_.written();
  }
  uint64_t drop_count() const override { return drop_count_; }

  bool drop_packets_for_testing() const { return drop_packets_; }

 private:
  TraceWriterImpl(const TraceWriterImpl&) = delete;
  TraceWriterImpl& operator=(const TraceWriterImpl&) = delete;

  // ScatteredStreamWriter::Delegate implementation.
  protozero::ContiguousMemoryRange GetNewBuffer() override;
  uint8_t* AnnotatePatch(uint8_t*) override;

  // MessageFinalizationListener implementation.
  void OnMessageFinalized(protozero::Message*) override;

  // Writes the size of the current fragment into the chunk.
  //
  // The size of nested messages inside TracePacket is written by
  // by the user, but the size of the TracePacket fragments is written by
  // TraceWriterImpl.
  void FinalizeFragmentIfRequired();

  // Returns |cur_chunk_| (for which is_valid() must be true) to the
  // |shmem_arbiter|.
  void ReturnCompletedChunk();

  // The per-producer arbiter that coordinates access to the shared memory
  // buffer from several threads.
  SharedMemoryArbiterImpl* const shmem_arbiter_;

  // ID of the current writer.
  const WriterID id_;

  // This is copied into the commit request by SharedMemoryArbiter. See comments
  // in data_source_config.proto for |target_buffer|. If this is a reservation
  // for a buffer ID in case of a startup trace writer, SharedMemoryArbiterImpl
  // will also translate the reservation ID to the actual buffer ID.
  const MaybeUnboundBufferID target_buffer_;

  // Whether GetNewChunk() should stall or return an invalid chunk if the SMB is
  // exhausted.
  const BufferExhaustedPolicy buffer_exhausted_policy_;

  // Monotonic (% wrapping) sequence id of the chunk. Together with the WriterID
  // this allows the Service to reconstruct the linear sequence of packets.
  ChunkID next_chunk_id_ = 0;

  // The chunk we are holding onto (if any).
  SharedMemoryABI::Chunk cur_chunk_;

  // Passed to protozero message to write directly into |cur_chunk_|. It
  // keeps track of the write pointer. It calls us back (GetNewBuffer()) when
  // |cur_chunk_| is filled.
  protozero::ScatteredStreamWriter protobuf_stream_writer_;

  // The packet returned via NewTracePacket(). Its owned by this class,
  // TracePacketHandle has just a pointer to it.
  //
  // The caller of NewTracePacket can use TakeStreamWriter() and use the stream
  // writer directly: in that case:
  // * cur_packet_->size() is not up to date. Only the stream writer has the
  //   correct information.
  // * cur_packet_->nested_message() is always nullptr.
  // * cur_packet_->size_field() is still used to track the start of the current
  //   fragment.
  std::unique_ptr<protozero::RootMessage<protos::pbzero::TracePacket>>
      cur_packet_;

  // The start address of |cur_packet_| within |cur_chunk_|. Used to figure out
  // fragments sizes when a TracePacket write is interrupted by GetNewBuffer().
  uint8_t* cur_fragment_start_ = nullptr;

  // true if we received a call to GetNewBuffer() after NewTracePacket(),
  // false if GetNewBuffer() happened during NewTracePacket() prologue, while
  // starting the TracePacket header.
  bool fragmenting_packet_ = false;

  // Set to |true| when the current chunk contains the maximum number of packets
  // a chunk can contain. When this is |true|, the next packet requires starting
  // a new chunk.
  bool reached_max_packets_per_chunk_ = false;

  // If we fail to acquire a new chunk when the arbiter operates in
  // SharedMemory::BufferExhaustedPolicy::kDrop mode, the trace writer enters a
  // mode in which data is written to a local garbage chunk and dropped.
  bool drop_packets_ = false;

  // Whether the trace writer should try to acquire a new chunk from the SMB
  // when the next TracePacket is started because it filled the garbage chunk at
  // least once since the last attempt.
  bool retry_new_chunk_after_packet_ = false;

  // Set to true if `cur_chunk_` has a packet counter that's inflated by one.
  // The count may be inflated to convince the tracing service scraping logic
  // that the last packet has been completed. When this is true, cur_chunk_
  // should have at least `kExtraRoomForInflatedPacket` bytes free.
  bool cur_chunk_packet_count_inflated_ = false;

  // Points to the size field of the still open fragment we're writing to the
  // current chunk. If the chunk was already returned, this is reset to
  // |nullptr|. If the fragment is finalized, this is reset to |nullptr|.
  //
  // Note: for nested messages the field is tracked somewhere else
  // (protozero::Message::size_field_ or PerfettoPbMsg::size_field). For the
  // root message, protozero::Message::size_field_ is nullptr and this is used
  // instead. This is because at the root level we deal with fragments, not
  // logical messages.
  uint8_t* cur_fragment_size_field_ = nullptr;

  // When a packet is fragmented across different chunks, the |size_field| of
  // the outstanding nested protobuf messages is redirected onto Patch entries
  // in this list at the time the Chunk is returned (because at that point we
  // have to release the ownership of the current Chunk). This list will be
  // later sent out-of-band to the tracing service, who will patch the required
  // chunks, if they are still around.
  PatchList patch_list_;

  // PID of the process that created the trace writer. Used for a DCHECK that
  // aims to detect unsupported process forks while tracing.
  const base::PlatformProcessId process_id_;

  // True for the first packet on sequence. See the comment for
  // TracePacket.first_packet_on_sequence for more details.
  bool first_packet_on_sequence_ = true;

  // Number of times the trace writter entered a
  // SharedMemory::BufferExhaustedPolicy::kDrop mode (i.e. as indicated by the
  // `drop_packets_` variable). Note that this does *not* necessarily equal the
  // number of trace packets dropped as multiple packets could have been dropped
  // in one entry into kDrop mode (i.e. this variable will be a *lower bound*
  // but *not* an upper bound).
  uint64_t drop_count_ = 0;
};

}  // namespace perfetto

#endif  // SRC_TRACING_CORE_TRACE_WRITER_IMPL_H_
