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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_V4L2_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_V4L2_TRACKER_H_

#include <stdint.h>
#include <cstdint>
#include <optional>

#include "perfetto/ext/base/flat_hash_map.h"

#include "perfetto/protozero/field.h"
#include "src/trace_processor/importers/common/args_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;

class V4l2Tracker : public Destructible {
 public:
  // Declared public for testing only.
  explicit V4l2Tracker(TraceProcessorContext*);
  V4l2Tracker(const V4l2Tracker&) = delete;
  V4l2Tracker& operator=(const V4l2Tracker&) = delete;
  ~V4l2Tracker() override;

  static V4l2Tracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->v4l2_tracker) {
      context->v4l2_tracker.reset(new V4l2Tracker(context));
    }
    return static_cast<V4l2Tracker*>(context->v4l2_tracker.get());
  }

  void ParseV4l2Event(uint64_t fld_id,
                      int64_t timestamp,
                      uint32_t pid,
                      const protozero::ConstBytes&);

 private:
  struct BufferEvent {
   public:
    int32_t device_minor;
    std::optional<uint32_t> index;
    std::optional<uint32_t> type;
    std::optional<uint32_t> bytesused;
    uint32_t flags;
    uint32_t field;
    int64_t timestamp;
    uint32_t sequence;
    uint32_t timecode_flags;
    uint32_t timecode_frames;
    uint32_t timecode_hours;
    uint32_t timecode_minutes;
    uint32_t timecode_seconds;
    uint32_t timecode_type;
    uint32_t timecode_userbits0;
    uint32_t timecode_userbits1;
    uint32_t timecode_userbits2;
    uint32_t timecode_userbits3;
  };

  struct BufferEventStringIds {
    explicit BufferEventStringIds(TraceStorage& storage);

    const StringId v4l2;
    const StringId v4l2_qbuf;
    const StringId v4l2_dqbuf;
    const StringId device_minor;
    const StringId index;
    const StringId type;
    const StringId bytesused;
    const StringId flags;
    const StringId field;
    const StringId timestamp;
    const StringId timecode_type;
    const StringId timecode_flags;
    const StringId timecode_frames;
    const StringId timecode_seconds;
    const StringId timecode_minutes;
    const StringId timecode_hours;
    const StringId timecode_userbits0;
    const StringId timecode_userbits1;
    const StringId timecode_userbits2;
    const StringId timecode_userbits3;
    const StringId sequence;
  };

  struct BufferTypeStringIds {
    explicit BufferTypeStringIds(TraceStorage& storage);

    StringId Map(uint32_t type);

    const StringId video_capture;
    const StringId video_output;
    const StringId video_overlay;
    const StringId vbi_capture;
    const StringId vbi_output;
    const StringId sliced_vbi_capture;
    const StringId sliced_vbi_output;
    const StringId video_output_overlay;
    const StringId video_capture_mplane;
    const StringId video_output_mplane;
    const StringId sdr_capture;
    const StringId sdr_output;
    const StringId meta_capture;
    const StringId meta_output;
    const StringId priv;
  };

  struct BufferFieldStringIds {
    explicit BufferFieldStringIds(TraceStorage& storage);

    StringId Map(uint32_t field);

    const StringId any;
    const StringId none;
    const StringId top;
    const StringId bottom;
    const StringId interlaced;
    const StringId seq_tb;
    const StringId seq_bt;
    const StringId alternate;
    const StringId interlaced_tb;
    const StringId interlaced_bt;
  };

  struct TimecodeTypeStringIds {
    explicit TimecodeTypeStringIds(TraceStorage& storage);

    StringId Map(uint32_t field);

    const StringId type_24fps;
    const StringId type_25fps;
    const StringId type_30fps;
    const StringId type_50fps;
    const StringId type_60fps;
  };

  struct QueuedBuffer {
    std::optional<SliceId> queue_slice_id;
  };

  std::optional<SliceId> AddSlice(StringId buf_name_id,
                                  int64_t timestamp,
                                  uint32_t pid,
                                  const BufferEvent& evt);

  void AddArgs(const BufferEvent& evt, ArgsTracker::BoundInserter* inserter);

  StringId InternBufFlags(uint32_t flags);
  StringId InternTcFlags(uint32_t flags);

  TraceProcessorContext* const context_;
  base::FlatHashMap<uint64_t, QueuedBuffer> queued_buffers_;

  BufferEventStringIds buf_event_ids_;
  BufferTypeStringIds buf_type_ids_;
  BufferFieldStringIds buf_field_ids_;
  TimecodeTypeStringIds tc_type_ids_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_V4L2_TRACKER_H_
