/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_TOKENIZER_H_

#include <optional>
#include <vector>

#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/clock_tracker.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

#include "protos/perfetto/trace/ftrace/ftrace_event_bundle.pbzero.h"

namespace perfetto {
namespace trace_processor {

class FtraceTokenizer {
 public:
  explicit FtraceTokenizer(TraceProcessorContext* context)
      : context_(context) {}

  base::Status TokenizeFtraceBundle(TraceBlobView bundle,
                                    RefPtr<PacketSequenceStateGeneration>,
                                    uint32_t packet_sequence_id);

 private:
  void TokenizeFtraceEvent(uint32_t cpu,
                           ClockTracker::ClockId,
                           TraceBlobView event,
                           RefPtr<PacketSequenceStateGeneration> state);
  void TokenizeFtraceCompactSched(uint32_t cpu,
                                  ClockTracker::ClockId,
                                  protozero::ConstBytes);
  void TokenizeFtraceCompactSchedSwitch(
      uint32_t cpu,
      ClockTracker::ClockId,
      const protos::pbzero::FtraceEventBundle::CompactSched::Decoder& compact,
      const std::vector<StringId>& string_table);
  void TokenizeFtraceCompactSchedWaking(
      uint32_t cpu,
      ClockTracker::ClockId,
      const protos::pbzero::FtraceEventBundle::CompactSched::Decoder& compact,
      const std::vector<StringId>& string_table);

  void HandleFtraceClockSnapshot(int64_t ftrace_ts,
                                 int64_t boot_ts,
                                 uint32_t packet_sequence_id);
  void TokenizeFtraceGpuWorkPeriod(uint32_t cpu,
                                   TraceBlobView event,
                                   RefPtr<PacketSequenceStateGeneration> state);
  void TokenizeFtraceThermalExynosAcpmBulk(
      uint32_t cpu,
      TraceBlobView event,
      RefPtr<PacketSequenceStateGeneration> state);
  void TokenizeFtraceParamSetValueCpm(
      uint32_t cpu,
      TraceBlobView event,
      RefPtr<PacketSequenceStateGeneration> state);
  std::optional<protozero::Field> GetFtraceEventField(
      uint32_t event_id,
      const TraceBlobView& event);

  void DlogWithLimit(const base::Status& status) {
    static std::atomic<uint32_t> dlog_count(0);
    if (dlog_count++ < 10)
      PERFETTO_DLOG("%s", status.c_message());
  }

  int64_t latest_ftrace_clock_snapshot_ts_ = 0;
  std::vector<bool> per_cpu_seen_first_bundle_;
  TraceProcessorContext* context_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_TOKENIZER_H_
