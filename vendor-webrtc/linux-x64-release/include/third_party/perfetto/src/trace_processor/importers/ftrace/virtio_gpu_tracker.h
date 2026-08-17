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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_VIRTIO_GPU_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_VIRTIO_GPU_TRACKER_H_

#include <cstdint>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/protozero/field.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class VirtioGpuTracker {
 public:
  explicit VirtioGpuTracker(TraceProcessorContext*);

  void ParseVirtioGpu(int64_t timestamp,
                      uint32_t field_id,
                      uint32_t pid,
                      protozero::ConstBytes blob);

 private:
  class VirtioGpuQueue {
   public:
    VirtioGpuQueue(TraceProcessorContext* context, const char* name);

    void HandleNumFree(int64_t timestamp, uint32_t num_free);
    void HandleCmdQueue(int64_t timestamp,
                        uint32_t seqno,
                        uint32_t type,
                        uint64_t fence_id);
    void HandleCmdResponse(int64_t timestamp, uint32_t seqno);

   private:
    TraceProcessorContext* context_;
    base::StringView name_;

    // Maps a seqno to the timestamp of a VirtioGpuCmdQueue.  The events
    // come in pairs of VirtioGpuCmdQueue plus VirtioGpuCmdResponse and
    // can be matched up via their seqno field.  To calculate the slice
    // duration we need to lookup the timestamp of the matching CmdQueue
    // event when we get the CmdResponse event.
    base::FlatHashMap<uint32_t, int64_t> start_timestamps_;
  };

  VirtioGpuQueue virtgpu_control_queue_;
  VirtioGpuQueue virtgpu_cursor_queue_;

  void ParseVirtioGpuCmdQueue(int64_t timestamp,
                              uint32_t pid,
                              protozero::ConstBytes);
  void ParseVirtioGpuCmdResponse(int64_t timestamp,
                                 uint32_t pid,
                                 protozero::ConstBytes blob);
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_VIRTIO_GPU_TRACKER_H_
