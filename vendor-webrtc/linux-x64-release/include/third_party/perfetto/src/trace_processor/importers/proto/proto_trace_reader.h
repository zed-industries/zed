/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_TRACE_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_TRACE_READER_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/importers/proto/multi_machine_trace_manager.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_builder.h"
#include "src/trace_processor/importers/proto/proto_trace_tokenizer.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace protozero {
struct ConstBytes;
}

namespace perfetto {

namespace protos::pbzero {
class TracePacket_Decoder;
class TraceConfig_Decoder;
}  // namespace protos::pbzero

namespace trace_processor {

class PacketSequenceState;
class TraceProcessorContext;
class TraceSorter;
class TraceStorage;

// Implementation of ChunkedTraceReader for proto traces. Tokenizes a proto
// trace into packets, handles parsing of any packets which need to be
// handled in trace-order and passes the remainder to TraceSorter to sort
// into timestamp order.
class ProtoTraceReader : public ChunkedTraceReader {
 public:
  // |reader| is the abstract method of getting chunks of size |chunk_size_b|
  // from a trace file with these chunks parsed into |trace|.
  explicit ProtoTraceReader(TraceProcessorContext*);
  ~ProtoTraceReader() override;

  // ChunkedTraceReader implementation.
  base::Status Parse(TraceBlobView) override;
  base::Status NotifyEndOfFile() override;

  using SyncClockSnapshots = base::FlatHashMap<
      int64_t,
      std::pair</*host ts*/ uint64_t, /*client ts*/ uint64_t>>;
  static base::FlatHashMap<int64_t /*Clock Id*/, int64_t /*Offset*/>
  CalculateClockOffsetsForTesting(
      std::vector<SyncClockSnapshots>& sync_clock_snapshots) {
    return CalculateClockOffsets(sync_clock_snapshots);
  }

  std::optional<StringId> GetBuiltinClockNameOrNull(int64_t clock_id);

 private:
  struct SequenceScopedState {
    std::optional<PacketSequenceStateBuilder> sequence_state_builder;
    uint32_t previous_packet_dropped_count = 0;
    uint32_t needs_incremental_state_total = 0;
    uint32_t needs_incremental_state_skipped = 0;
  };

  using ConstBytes = protozero::ConstBytes;
  base::Status ParsePacket(TraceBlobView);
  base::Status TimestampTokenizeAndPushToSorter(TraceBlobView);
  base::Status ParseServiceEvent(int64_t ts, ConstBytes);
  base::Status ParseClockSnapshot(ConstBytes blob, uint32_t seq_id);
  base::Status ParseRemoteClockSync(ConstBytes blob);
  void HandleIncrementalStateCleared(
      const protos::pbzero::TracePacket_Decoder&);
  void HandleFirstPacketOnSequence(uint32_t packet_sequence_id);
  void HandlePreviousPacketDropped(const protos::pbzero::TracePacket_Decoder&);
  void ParseTracePacketDefaults(const protos::pbzero::TracePacket_Decoder&,
                                TraceBlobView trace_packet_defaults);
  void ParseInternedData(const protos::pbzero::TracePacket_Decoder&,
                         TraceBlobView interned_data);
  void ParseTraceConfig(ConstBytes);
  void ParseTraceStats(ConstBytes);

  static base::FlatHashMap<int64_t /*Clock Id*/, int64_t /*Offset*/>
  CalculateClockOffsets(std::vector<SyncClockSnapshots>&);

  PacketSequenceStateBuilder* GetIncrementalStateForPacketSequence(
      uint32_t sequence_id) {
    auto& builder = sequence_state_.Find(sequence_id)->sequence_state_builder;
    if (!builder) {
      builder = PacketSequenceStateBuilder(context_);
    }
    return &*builder;
  }
  base::Status ParseExtensionDescriptor(ConstBytes descriptor);

  TraceProcessorContext* context_;

  ProtoTraceTokenizer tokenizer_;

  // Temporary. Currently trace packets do not have a timestamp, so the
  // timestamp given is latest_timestamp_.
  int64_t latest_timestamp_ = 0;

  base::FlatHashMap<uint32_t, SequenceScopedState> sequence_state_;
  StringId skipped_packet_key_id_;
  StringId invalid_incremental_state_key_id_;

  std::vector<TraceBlobView> eof_deferred_packets_;
  bool received_eof_ = false;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROTO_TRACE_READER_H_
