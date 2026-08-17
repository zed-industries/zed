/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_DRM_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_DRM_TRACKER_H_

#include <cstdint>
#include <deque>
#include <memory>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/protozero/field.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class DrmTracker {
 public:
  explicit DrmTracker(TraceProcessorContext*);

  void ParseDrm(int64_t timestamp,
                uint32_t field_id,
                uint32_t pid,
                protozero::ConstBytes blob);

 private:
  void DrmVblankEvent(int64_t timestamp, int32_t crtc, uint32_t seqno);
  void DrmVblankEventDelivered(int64_t timestamp, int32_t crtc, uint32_t seqno);

  struct SchedRing {
    TrackId track_id;
    std::deque<uint64_t> running_jobs;

    base::FlatHashMap<uint64_t, SliceId> out_slice_ids;
  };
  SchedRing& GetSchedRingByName(base::StringView name);
  void BeginSchedRingSlice(int64_t timestamp, SchedRing& ring);

  void DrmSchedJob(int64_t timestamp,
                   uint32_t pid,
                   base::StringView name,
                   uint64_t job_id);
  void DrmRunJob(int64_t timestamp,
                 base::StringView name,
                 uint64_t job_id,
                 uint64_t fence_id);
  void DrmSchedProcessJob(int64_t timestamp, uint64_t fence_id);

  struct FenceTimeline {
    TrackId track_id;
    bool has_dma_fence_emit;
    std::deque<uint32_t> pending_fences;
  };
  FenceTimeline& GetFenceTimelineByContext(uint32_t context,
                                           base::StringView name);
  void BeginFenceTimelineSlice(int64_t timestamp,
                               const FenceTimeline& timeline);

  void DmaFenceInit(int64_t timestamp,
                    base::StringView name,
                    uint32_t context,
                    uint32_t seqno);
  void DmaFenceEmit(int64_t timestamp,
                    base::StringView name,
                    uint32_t context,
                    uint32_t seqno);
  void DmaFenceSignaled(int64_t timestamp,
                        base::StringView name,
                        uint32_t context,
                        uint32_t seqno);
  void DmaFenceWaitStart(int64_t timestamp,
                         uint32_t pid,
                         uint32_t context,
                         uint32_t seqno);
  void DmaFenceWaitEnd(int64_t timestamp, uint32_t pid);

  TraceProcessorContext* const context_;

  const StringId vblank_slice_signal_id_;
  const StringId vblank_slice_deliver_id_;
  const StringId vblank_arg_seqno_id_;
  const StringId sched_slice_schedule_id_;
  const StringId sched_slice_job_id_;
  const StringId sched_arg_ring_id_;
  const StringId sched_arg_job_id_;
  const StringId fence_slice_fence_id_;
  const StringId fence_slice_wait_id_;
  const StringId fence_arg_context_id_;
  const StringId fence_arg_seqno_id_;

  base::FlatHashMap<base::StringView, std::unique_ptr<SchedRing>> sched_rings_;
  base::FlatHashMap<uint64_t, SchedRing*> sched_pending_fences_;

  base::FlatHashMap<uint32_t, std::unique_ptr<FenceTimeline>> fence_timelines_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_DRM_TRACKER_H_
