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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_SYSCALLS_SYSCALL_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_SYSCALLS_SYSCALL_TRACKER_H_

#include <limits>
#include <tuple>

#include "perfetto/ext/base/string_view.h"
#include "src/kernel_utils/syscall_table.h"
#include "src/trace_processor/containers/bit_vector.h"
#include "src/trace_processor/importers/common/event_tracker.h"
#include "src/trace_processor/importers/common/slice_tracker.h"
#include "src/trace_processor/importers/common/track_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

class SyscallTracker : public Destructible {
 public:
  SyscallTracker(const SyscallTracker&) = delete;
  SyscallTracker& operator=(const SyscallTracker&) = delete;
  ~SyscallTracker() override;
  static SyscallTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->syscall_tracker) {
      context->syscall_tracker.reset(new SyscallTracker(context));
    }
    return static_cast<SyscallTracker*>(context->syscall_tracker.get());
  }

  void SetArchitecture(Architecture architecture);

  void Enter(int64_t ts,
             UniqueTid utid,
             uint32_t syscall_num,
             EventTracker::SetArgsCallback args_callback =
                 EventTracker::SetArgsCallback()) {
    StringId name = SyscallNumberToStringId(syscall_num);
    if (name.is_null())
      return;

    TrackId track_id = context_->track_tracker->InternThreadTrack(utid);

    // sys_rt_sigreturn does not return so should be inserted as an instant
    // event. See https://github.com/google/perfetto/issues/733 for details.
    if (name == sys_rt_sigreturn_string_id_) {
      context_->slice_tracker->Scoped(ts, track_id, kNullStringId, name, 0,
                                      args_callback);
    } else {
      context_->slice_tracker->Begin(ts, track_id, kNullStringId /* cat */,
                                     name, args_callback);
    }

    if (name == sys_write_string_id_) {
      if (utid >= in_sys_write_.size())
        in_sys_write_.Resize(utid + 1);

      in_sys_write_.Set(utid);
    }
  }

  void Exit(int64_t ts,
            UniqueTid utid,
            uint32_t syscall_num,
            EventTracker::SetArgsCallback args_callback =
                EventTracker::SetArgsCallback()) {
    StringId name = SyscallNumberToStringId(syscall_num);
    if (name.is_null())
      return;

    if (name == sys_write_string_id_) {
      if (utid >= in_sys_write_.size())
        in_sys_write_.Resize(utid + 1);
      // Either seeing an exit event without the corresponding entry at the
      // start of the trace, or the slice was closed by
      // MaybeTruncateOngoingWriteSlice.
      if (!in_sys_write_.IsSet(utid))
        return;
      in_sys_write_.Clear(utid);
    }

    TrackId track_id = context_->track_tracker->InternThreadTrack(utid);
    context_->slice_tracker->End(ts, track_id, kNullStringId /* cat */, name,
                                 args_callback);
  }

  // Resolves slice nesting issues when the sys_write is for an atrace slice on
  // android. See callsite for details.
  void MaybeTruncateOngoingWriteSlice(int64_t ts, UniqueTid utid) {
    if (utid >= in_sys_write_.size())
      in_sys_write_.Resize(utid + 1);

    if (!in_sys_write_.IsSet(utid))
      return;
    in_sys_write_.Clear(utid);
    context_->storage->IncrementStats(stats::truncated_sys_write_duration);

    TrackId track_id = context_->track_tracker->InternThreadTrack(utid);
    context_->slice_tracker->End(ts, track_id, kNullStringId /* cat */,
                                 sys_write_string_id_);
  }

 private:
  explicit SyscallTracker(TraceProcessorContext*);

  TraceProcessorContext* const context_;

  inline StringId SyscallNumberToStringId(uint32_t syscall_num) {
    if (syscall_num > kMaxSyscalls)
      return kNullStringId;
    return arch_syscall_to_string_id_[syscall_num];
  }

  // This is table from platform specific syscall number directly to
  // the relevant StringId (this avoids having to always do two conversions).
  std::array<StringId, kMaxSyscalls> arch_syscall_to_string_id_{};
  StringId sys_write_string_id_ = std::numeric_limits<StringId>::max();
  StringId sys_rt_sigreturn_string_id_ = std::numeric_limits<StringId>::max();
  // UniqueTids currently in a sys_write syscall.
  BitVector in_sys_write_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_SYSCALLS_SYSCALL_TRACKER_H_
