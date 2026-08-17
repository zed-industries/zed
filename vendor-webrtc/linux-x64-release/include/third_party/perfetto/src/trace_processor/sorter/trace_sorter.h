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

#ifndef SRC_TRACE_PROCESSOR_SORTER_TRACE_SORTER_H_
#define SRC_TRACE_PROCESSOR_SORTER_TRACE_SORTER_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <memory>
#include <optional>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/ext/base/circular_queue.h"
#include "perfetto/public/compiler.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/android_bugreport/android_dumpstate_event.h"
#include "src/trace_processor/importers/android_bugreport/android_log_event.h"
#include "src/trace_processor/importers/art_method/art_method_event.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/fuchsia/fuchsia_record.h"
#include "src/trace_processor/importers/gecko/gecko_event.h"
#include "src/trace_processor/importers/instruments/row.h"
#include "src/trace_processor/importers/perf/record.h"
#include "src/trace_processor/importers/perf_text/perf_text_event.h"
#include "src/trace_processor/importers/systrace/systrace_line.h"
#include "src/trace_processor/sorter/trace_token_buffer.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/bump_allocator.h"

namespace perfetto::trace_processor {

// This class takes care of sorting events parsed from the trace stream in
// arbitrary order and pushing them to the next pipeline stages (parsing) in
// order. In order to support streaming use-cases, sorting happens within a
// window.
//
// Events are held in the TraceSorter staging area (events_) until either:
// 1. We can determine that it's safe to extract events by observing
//  TracingServiceEvent Flush and ReadBuffer events
// 2. The trace EOF is reached
//
// Incremental extraction
//
// Incremental extraction happens by using a combination of flush and read
// buffer events from the tracing service. Note that incremental extraction
// is only applicable for write_into_file traces; ring-buffer traces will
// be sorted fully in-memory implicitly because there is only a single read
// buffer call at the end.
//
// The algorithm for incremental extraction is explained in detail at
// go/trace-sorting-is-complicated.
//
// Sorting algorithm
//
// The sorting algorithm is designed around the assumption that:
// - Most events come from ftrace.
// - Ftrace events are sorted within each cpu most of the times.
//
// Due to this, this class is oprerates as a streaming merge-sort of N+1 queues
// (N = num cpus + 1 for non-ftrace events). Each queue in turn gets sorted (if
// necessary) before proceeding with the global merge-sort-extract.
//
// When an event is pushed through, it is just appended to the end of one of
// the N queues. While appending, we keep track of the fact that the queue
// is still ordered or just lost ordering. When an out-of-order event is
// detected on a queue we keep track of: (1) the offset within the queue where
// the chaos begun, (2) the timestamp that broke the ordering.
//
// When we decide to extract events from the queues into the next stages of
// the trace processor, we re-sort the events in the queue. Rather than
// re-sorting everything all the times, we use the above knowledge to restrict
// sorting to the (hopefully smaller) tail of the |events_| staging area.
// At any time, the first partition of |events_| [0 .. sort_start_idx_) is
// ordered, and the second partition [sort_start_idx_.. end] is not.
// We use a logarithmic bound search operation to figure out what is the index
// within the first partition where sorting should start, and sort all events
// from there to the end.
class TraceSorter {
 public:
  enum class SortingMode {
    kDefault,
    kFullSort,
  };
  enum class EventHandling {
    // Indicates that events should be sorted and pushed to the parsing stage.
    kSortAndPush,

    // Indicates that events should be sorted but then dropped before pushing
    // to the parsing stage.
    // Used for performance analysis of the sorter.
    kSortAndDrop,

    // Indicates that the events should be dropped as soon as they enter the
    // sorter.
    // Used in cases where we only want to perform tokenization: dropping data
    // when it hits the sorter is much cleaner than trying to handle this
    // at every different tokenizer.
    kDrop,
  };

  TraceSorter(TraceProcessorContext*,
              SortingMode,
              EventHandling = EventHandling::kSortAndPush);

  ~TraceSorter();

  SortingMode sorting_mode() const { return sorting_mode_; }
  bool SetSortingMode(SortingMode sorting_mode);

  void AddMachineContext(TraceProcessorContext* context) {
    sorter_data_by_machine_.emplace_back(context);
  }

  void PushAndroidDumpstateEvent(
      int64_t timestamp,
      const AndroidDumpstateEvent& event,
      std::optional<MachineId> machine_id = std::nullopt) {
    AppendNonFtraceEvent(timestamp,
                         TimestampedEvent::Type::kAndroidDumpstateEvent, event,
                         machine_id);
  }

  void PushAndroidLogEvent(int64_t timestamp,
                           const AndroidLogEvent& event,
                           std::optional<MachineId> machine_id = std::nullopt) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kAndroidLogEvent,
                         event, machine_id);
  }

  void PushPerfRecord(int64_t timestamp,
                      perf_importer::Record record,
                      std::optional<MachineId> machine_id = std::nullopt) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kPerfRecord,
                         std::move(record), machine_id);
  }

  void PushSpeRecord(int64_t timestamp,
                     TraceBlobView record,
                     std::optional<MachineId> machine_id = std::nullopt) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kSpeRecord,
                         std::move(record), machine_id);
  }

  void PushInstrumentsRow(int64_t timestamp,
                          const instruments_importer::Row& row,
                          std::optional<MachineId> machine_id = std::nullopt) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kInstrumentsRow,
                         row, machine_id);
  }

  void PushTracePacket(int64_t timestamp,
                       TracePacketData data,
                       std::optional<MachineId> machine_id = std::nullopt) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kTracePacket,
                         std::move(data), machine_id);
  }

  void PushTracePacket(int64_t timestamp,
                       RefPtr<PacketSequenceStateGeneration> state,
                       TraceBlobView tbv,
                       std::optional<MachineId> machine_id = std::nullopt) {
    PushTracePacket(timestamp,
                    TracePacketData{std::move(tbv), std::move(state)},
                    machine_id);
  }

  void PushJsonValue(int64_t timestamp,
                     std::string json_value,
                     const JsonEvent::Type& type) {
    if (const auto* scoped = std::get_if<JsonEvent::Scoped>(&type); scoped) {
      // We need to account for slices with duration by sorting them specially:
      // this requires us to use the slower comparator which takes this into
      // account.
      use_slow_sorting_ = true;
    }
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kJsonValue,
                         JsonEvent{std::move(json_value), type});
  }

  void PushFuchsiaRecord(int64_t timestamp, FuchsiaRecord fuchsia_record) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kFuchsiaRecord,
                         std::move(fuchsia_record));
  }

  void PushSystraceLine(SystraceLine systrace_line) {
    AppendNonFtraceEvent(systrace_line.ts,
                         TimestampedEvent::Type::kSystraceLine,
                         std::move(systrace_line));
  }

  void PushTrackEventPacket(
      int64_t timestamp,
      TrackEventData track_event,
      std::optional<MachineId> machine_id = std::nullopt) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kTrackEvent,
                         std::move(track_event), machine_id);
  }

  void PushLegacyV8CpuProfileEvent(int64_t timestamp,
                                   uint64_t session_id,
                                   uint32_t pid,
                                   uint32_t tid,
                                   uint32_t callsite_id) {
    AppendNonFtraceEvent(
        timestamp, TimestampedEvent::Type::kLegacyV8CpuProfileEvent,
        LegacyV8CpuProfileEvent{session_id, pid, tid, callsite_id});
  }

  void PushGeckoEvent(int64_t timestamp,
                      const gecko_importer::GeckoEvent& event) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kGeckoEvent, event);
  }

  void PushArtMethodEvent(int64_t timestamp,
                          const art_method::ArtMethodEvent& event) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kArtMethodEvent,
                         event);
  }

  void PushPerfTextEvent(int64_t timestamp,
                         const perf_text_importer::PerfTextEvent& event) {
    AppendNonFtraceEvent(timestamp, TimestampedEvent::Type::kPerfTextEvent,
                         event);
  }

  void PushEtwEvent(uint32_t cpu,
                    int64_t timestamp,
                    TraceBlobView tbv,
                    RefPtr<PacketSequenceStateGeneration> state,
                    std::optional<MachineId> machine_id = std::nullopt) {
    if (PERFETTO_UNLIKELY(event_handling_ == EventHandling::kDrop)) {
      return;
    }
    TraceTokenBuffer::Id id =
        token_buffer_.Append(TracePacketData{std::move(tbv), std::move(state)});
    auto* queue = GetQueue(cpu + 1, machine_id);
    queue->Append(timestamp, TimestampedEvent::Type::kEtwEvent, id,
                  use_slow_sorting_);
    UpdateAppendMaxTs(queue);
  }

  void PushFtraceEvent(uint32_t cpu,
                       int64_t timestamp,
                       TraceBlobView tbv,
                       RefPtr<PacketSequenceStateGeneration> state,
                       std::optional<MachineId> machine_id = std::nullopt) {
    if (PERFETTO_UNLIKELY(event_handling_ == EventHandling::kDrop)) {
      return;
    }
    TraceTokenBuffer::Id id =
        token_buffer_.Append(TracePacketData{std::move(tbv), std::move(state)});
    auto* queue = GetQueue(cpu + 1, machine_id);
    queue->Append(timestamp, TimestampedEvent::Type::kFtraceEvent, id,
                  use_slow_sorting_);
    UpdateAppendMaxTs(queue);
  }

  void PushInlineFtraceEvent(
      uint32_t cpu,
      int64_t timestamp,
      const InlineSchedSwitch& inline_sched_switch,
      std::optional<MachineId> machine_id = std::nullopt) {
    if (PERFETTO_UNLIKELY(event_handling_ == EventHandling::kDrop)) {
      return;
    }
    // TODO(rsavitski): if a trace has a mix of normal & "compact" events
    // (being pushed through this function), the ftrace batches will no longer
    // be fully sorted by timestamp. In such situations, we will have to sort
    // at the end of the batch. We can do better as both sub-sequences are
    // sorted however. Consider adding extra queues, or pushing them in a
    // merge-sort fashion
    // // instead.
    TraceTokenBuffer::Id id = token_buffer_.Append(inline_sched_switch);
    auto* queue = GetQueue(cpu + 1, machine_id);
    queue->Append(timestamp, TimestampedEvent::Type::kInlineSchedSwitch, id,
                  use_slow_sorting_);
    UpdateAppendMaxTs(queue);
  }

  void PushInlineFtraceEvent(
      uint32_t cpu,
      int64_t timestamp,
      const InlineSchedWaking& inline_sched_waking,
      std::optional<MachineId> machine_id = std::nullopt) {
    if (PERFETTO_UNLIKELY(event_handling_ == EventHandling::kDrop)) {
      return;
    }
    TraceTokenBuffer::Id id = token_buffer_.Append(inline_sched_waking);
    auto* queue = GetQueue(cpu + 1, machine_id);
    queue->Append(timestamp, TimestampedEvent::Type::kInlineSchedWaking, id,
                  use_slow_sorting_);
    UpdateAppendMaxTs(queue);
  }

  void ExtractEventsForced() {
    BumpAllocator::AllocId end_id = token_buffer_.PastTheEndAllocId();
    SortAndExtractEventsUntilAllocId(end_id);
    for (auto& sorter_data : sorter_data_by_machine_) {
      for (const auto& queue : sorter_data.queues) {
        PERFETTO_CHECK(queue.events_.empty());
      }
      sorter_data.queues.clear();
    }

    alloc_id_for_extraction_ = end_id;
    flushes_since_extraction_ = 0;
  }

  void NotifyFlushEvent() { flushes_since_extraction_++; }

  void NotifyReadBufferEvent() {
    if (sorting_mode_ == SortingMode::kFullSort ||
        flushes_since_extraction_ < 2) {
      return;
    }

    SortAndExtractEventsUntilAllocId(alloc_id_for_extraction_);
    alloc_id_for_extraction_ = token_buffer_.PastTheEndAllocId();
    flushes_since_extraction_ = 0;
  }

  int64_t max_timestamp() const { return append_max_ts_; }

 private:
  struct TimestampedEvent {
    enum class Type : uint8_t {
      kAndroidDumpstateEvent,
      kAndroidLogEvent,
      kEtwEvent,
      kFtraceEvent,
      kFuchsiaRecord,
      kInlineSchedSwitch,
      kInlineSchedWaking,
      kInstrumentsRow,
      kJsonValue,
      kLegacyV8CpuProfileEvent,
      kPerfRecord,
      kSpeRecord,
      kSystraceLine,
      kTracePacket,
      kTrackEvent,
      kGeckoEvent,
      kArtMethodEvent,
      kPerfTextEvent,
      kMax = kPerfTextEvent,
    };

    // Number of bits required to store the max element in |Type|.
    static constexpr uint32_t kMaxTypeBits = 6;
    static_assert(static_cast<uint8_t>(Type::kMax) <= (1 << kMaxTypeBits),
                  "Max type does not fit inside storage");

    // The timestamp of this event.
    int64_t ts;

    // The fields inside BumpAllocator::AllocId of this tokenized object
    // corresponding to this event.
    uint64_t chunk_index : BumpAllocator::kChunkIndexAllocIdBits;
    uint64_t chunk_offset : BumpAllocator::kChunkOffsetAllocIdBits;

    // The type of this event. GCC7 does not like bit-field enums (see
    // https://stackoverflow.com/questions/36005063/gcc-suppress-warning-too-small-to-hold-all-values-of)
    // so use an uint64_t instead and cast to the enum type.
    uint64_t event_type : kMaxTypeBits;

    BumpAllocator::AllocId alloc_id() const {
      return BumpAllocator::AllocId{chunk_index, chunk_offset};
    }

    Type type() const { return static_cast<Type>(event_type); }

    // For std::lower_bound().
    static bool Compare(const TimestampedEvent& x, int64_t ts) {
      return x.ts < ts;
    }

    // For std::sort().
    bool operator<(const TimestampedEvent& evt) const {
      return std::tie(ts, chunk_index, chunk_offset) <
             std::tie(evt.ts, evt.chunk_index, evt.chunk_offset);
    }

    struct SlowOperatorLess {
      // For std::sort() in slow mode.
      bool operator()(const TimestampedEvent& a,
                      const TimestampedEvent& b) const {
        int64_t a_key =
            KeyForType(buffer.Get<JsonEvent>(GetTokenBufferId(a))->type);
        int64_t b_key =
            KeyForType(buffer.Get<JsonEvent>(GetTokenBufferId(b))->type);
        return std::tie(a.ts, a_key, a.chunk_index, a.chunk_offset) <
               std::tie(b.ts, b_key, b.chunk_index, b.chunk_offset);
      }

      static int64_t KeyForType(const JsonEvent::Type& type) {
        switch (type.index()) {
          case base::variant_index<JsonEvent::Type, JsonEvent::End>():
            return std::numeric_limits<int64_t>::min();
          case base::variant_index<JsonEvent::Type, JsonEvent::Scoped>():
            return std::numeric_limits<int64_t>::max() -
                   std::get<JsonEvent::Scoped>(type).dur;
          default:
            return std::numeric_limits<int64_t>::max();
        }
        PERFETTO_FATAL("For GCC");
      }

      TraceTokenBuffer& buffer;
    };
  };

  static_assert(sizeof(TimestampedEvent) == 16,
                "TimestampedEvent must be equal to 16 bytes");
  static_assert(std::is_trivially_copyable_v<TimestampedEvent>,
                "TimestampedEvent must be trivially copyable");
  static_assert(std::is_trivially_move_assignable_v<TimestampedEvent>,
                "TimestampedEvent must be trivially move assignable");
  static_assert(std::is_trivially_move_constructible_v<TimestampedEvent>,
                "TimestampedEvent must be trivially move constructible");
  static_assert(std::is_nothrow_swappable_v<TimestampedEvent>,
                "TimestampedEvent must be trivially swappable");

  struct Queue {
    void Append(int64_t ts,
                TimestampedEvent::Type type,
                TraceTokenBuffer::Id id,
                bool use_slow_sorting) {
      {
        TimestampedEvent event;
        event.ts = ts;
        event.chunk_index = id.alloc_id.chunk_index;
        event.chunk_offset = id.alloc_id.chunk_offset;
        event.event_type = static_cast<uint8_t>(type);
        events_.emplace_back(std::move(event));
      }

      // Events are often seen in order.
      if (PERFETTO_LIKELY(ts > max_ts_ ||
                          (!use_slow_sorting && ts == max_ts_))) {
        max_ts_ = ts;
      } else {
        // The event is breaking ordering. The first time it happens, keep
        // track of which index we are at. We know that everything before that
        // is sorted (because events were pushed monotonically). Everything
        // after that index, instead, will need a sorting pass before moving
        // events to the next pipeline stage.
        if (sort_start_idx_ == 0) {
          sort_start_idx_ = events_.size() - 1;
          sort_min_ts_ = ts;
        } else {
          sort_min_ts_ = std::min(sort_min_ts_, ts);
        }
      }

      min_ts_ = std::min(min_ts_, ts);
      PERFETTO_DCHECK(min_ts_ <= max_ts_);
    }

    bool needs_sorting() const { return sort_start_idx_ != 0; }
    void Sort(TraceTokenBuffer&, bool use_slow_sorting);

    base::CircularQueue<TimestampedEvent> events_;
    int64_t min_ts_ = std::numeric_limits<int64_t>::max();
    int64_t max_ts_ = 0;
    size_t sort_start_idx_ = 0;
    int64_t sort_min_ts_ = std::numeric_limits<int64_t>::max();
  };

  void SortAndExtractEventsUntilAllocId(BumpAllocator::AllocId alloc_id);

  Queue* GetQueue(size_t index,
                  std::optional<MachineId> machine_id = std::nullopt) {
    // sorter_data_by_machine_[0] corresponds to the default machine.
    PERFETTO_DCHECK(sorter_data_by_machine_[0].machine_id == std::nullopt);
    auto* queues = &sorter_data_by_machine_[0].queues;

    // Find the TraceSorterData instance when |machine_id| is not nullopt.
    if (PERFETTO_UNLIKELY(machine_id.has_value())) {
      auto it = std::find_if(sorter_data_by_machine_.begin() + 1,
                             sorter_data_by_machine_.end(),
                             [machine_id](const TraceSorterData& item) {
                               return item.machine_id == machine_id;
                             });
      PERFETTO_DCHECK(it != sorter_data_by_machine_.end());
      queues = &it->queues;
    }

    if (PERFETTO_UNLIKELY(index >= queues->size()))
      queues->resize(index + 1);
    return &queues->at(index);
  }

  template <typename E>
  void AppendNonFtraceEvent(
      int64_t ts,
      TimestampedEvent::Type event_type,
      E&& evt,
      std::optional<MachineId> machine_id = std::nullopt) {
    if (PERFETTO_UNLIKELY(event_handling_ == EventHandling::kDrop)) {
      return;
    }
    TraceTokenBuffer::Id id = token_buffer_.Append(std::forward<E>(evt));
    AppendNonFtraceEventWithId(ts, event_type, id, machine_id);
  }

  void AppendNonFtraceEventWithId(int64_t ts,
                                  TimestampedEvent::Type event_type,
                                  TraceTokenBuffer::Id id,
                                  std::optional<MachineId> machine_id) {
    Queue* queue = GetQueue(0, machine_id);
    queue->Append(ts, event_type, id, use_slow_sorting_);
    UpdateAppendMaxTs(queue);
  }

  void UpdateAppendMaxTs(Queue* queue) {
    append_max_ts_ = std::max(append_max_ts_, queue->max_ts_);
  }

  void ParseTracePacket(TraceProcessorContext& context,
                        const TimestampedEvent&);
  void ParseFtracePacket(TraceProcessorContext& context,
                         uint32_t cpu,
                         const TimestampedEvent&);
  void ParseEtwPacket(TraceProcessorContext& context,
                      uint32_t cpu,
                      const TimestampedEvent&);

  void MaybeExtractEvent(size_t machine_idx,
                         size_t queue_idx,
                         const TimestampedEvent&);
  void ExtractAndDiscardTokenizedObject(const TimestampedEvent& event);

  static TraceTokenBuffer::Id GetTokenBufferId(const TimestampedEvent& event) {
    return TraceTokenBuffer::Id{event.alloc_id()};
  }

  struct TraceSorterData {
    explicit TraceSorterData(TraceProcessorContext* _machine_context)
        : machine_id(_machine_context->machine_id()),
          machine_context(_machine_context) {}
    std::optional<MachineId> machine_id;
    // queues_[0] is the general (non-ftrace) queue.
    // queues_[1] is the ftrace queue for CPU(0).
    // queues_[x] is the ftrace queue for CPU(x - 1).
    TraceProcessorContext* machine_context;
    std::vector<Queue> queues;
  };
  std::vector<TraceSorterData> sorter_data_by_machine_;

  // Whether we should ignore incremental extraction and just wait for
  // forced extractionn at the end of the trace.
  SortingMode sorting_mode_ = SortingMode::kDefault;

  std::shared_ptr<TraceStorage> storage_;

  // Buffer for storing tokenized objects while the corresponding events are
  // being sorted.
  TraceTokenBuffer token_buffer_;

  // The AllocId until which events should be extracted. Set based
  // on the AllocId in |OnReadBuffer|.
  BumpAllocator::AllocId alloc_id_for_extraction_ =
      token_buffer_.PastTheEndAllocId();

  // The number of flushes which have happened since the last incremental
  // extraction.
  uint32_t flushes_since_extraction_ = 0;

  // max(e.ts for e appended to the sorter)
  int64_t append_max_ts_ = 0;

  // How events pushed into the sorter should be handled.
  EventHandling event_handling_ = EventHandling::kSortAndPush;

  // max(e.ts for e pushed to next stage)
  int64_t latest_pushed_event_ts_ = std::numeric_limits<int64_t>::min();

  // Whether when std::sorting the queues, we should use the slow
  // sorting algorithm
  bool use_slow_sorting_ = false;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_SORTER_TRACE_SORTER_H_
