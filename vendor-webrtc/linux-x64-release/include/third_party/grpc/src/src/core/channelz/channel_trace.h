//
//
// Copyright 2017 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_CHANNELZ_CHANNEL_TRACE_H
#define GRPC_SRC_CORE_CHANNELZ_CHANNEL_TRACE_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#include "absl/base/thread_annotations.h"
#include "src/core/util/json/json.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {
namespace channelz {

namespace testing {
size_t GetSizeofTraceEvent(void);
}

class BaseNode;

// Object used to hold live data for a channel. This data is exposed via the
// channelz service:
// https://github.com/grpc/proposal/blob/master/A14-channelz.md
class ChannelTrace {
 public:
  explicit ChannelTrace(size_t max_event_memory);
  ~ChannelTrace();

  enum Severity {
    Unset = 0,  // never to be used
    Info,       // we start at 1 to avoid using proto default values
    Warning,
    Error
  };

  static const char* SeverityString(ChannelTrace::Severity severity) {
    switch (severity) {
      case ChannelTrace::Severity::Info:
        return "CT_INFO";
      case ChannelTrace::Severity::Warning:
        return "CT_WARNING";
      case ChannelTrace::Severity::Error:
        return "CT_ERROR";
      default:
        GPR_UNREACHABLE_CODE(return "CT_UNKNOWN");
    }
  }

  // Adds a new trace event to the tracing object
  //
  // NOTE: each ChannelTrace tracks the memory used by its list of trace
  // events, so adding an event with a large amount of data could cause other
  // trace event to be evicted. If a single trace is larger than the limit, it
  // will cause all events to be evicted. The limit is set with the arg:
  // GRPC_ARG_MAX_CHANNEL_TRACE_EVENT_MEMORY_PER_NODE.
  //
  // TODO(ncteisen): as this call is used more and more throughout the gRPC
  // stack, determine if it makes more sense to accept a char* instead of a
  // slice.
  void AddTraceEvent(Severity severity, const grpc_slice& data);

  // Adds a new trace event to the tracing object. This trace event refers to a
  // an event that concerns a different channelz entity. For example, if this
  // channel has created a new subchannel, then it would record that with
  // a TraceEvent referencing the new subchannel.
  //
  // NOTE: see the note in the method above.
  //
  // TODO(ncteisen): see the todo in the method above.
  void AddTraceEventWithReference(Severity severity, const grpc_slice& data,
                                  RefCountedPtr<BaseNode> referenced_entity);

  // Creates and returns the raw Json object, so a parent channelz
  // object may incorporate the json before rendering.
  Json RenderJson() const;

  void ForEachTraceEvent(
      absl::FunctionRef<void(gpr_timespec, Severity, std::string,
                             RefCountedPtr<BaseNode>)>
          callback) const {
    MutexLock lock(&mu_);
    ForEachTraceEventLocked(callback);
  }
  std::string creation_timestamp() const;
  uint64_t num_events_logged() const {
    MutexLock lock(&mu_);
    return num_events_logged_;
  }

 private:
  friend size_t testing::GetSizeofTraceEvent(void);

  // Private class to encapsulate all the data and bookkeeping needed for a
  // a trace event.
  class TraceEvent {
   public:
    // Constructor for a TraceEvent that references a channel.
    TraceEvent(Severity severity, const grpc_slice& data,
               RefCountedPtr<BaseNode> referenced_entity_);

    // Constructor for a TraceEvent that does not reverence a different
    // channel.
    TraceEvent(Severity severity, const grpc_slice& data);

    ~TraceEvent();

    // set and get for the next_ pointer.
    TraceEvent* next() const { return next_; }
    void set_next(TraceEvent* next) { next_ = next; }

    size_t memory_usage() const { return memory_usage_; }
    gpr_timespec timestamp() const { return timestamp_; }
    Severity severity() const { return severity_; }
    std::string description() const;
    RefCountedPtr<BaseNode> referenced_entity() const;

   private:
    const gpr_timespec timestamp_;
    const Severity severity_;
    const grpc_slice data_;
    const size_t memory_usage_;
    // the tracer object for the (sub)channel that this trace event refers to.
    const RefCountedPtr<BaseNode> referenced_entity_;
    TraceEvent* next_ = nullptr;
  };  // TraceEvent

  // Internal helper to add and link in a trace event
  void AddTraceEventHelper(TraceEvent* new_trace_event);
  void ForEachTraceEventLocked(
      absl::FunctionRef<void(gpr_timespec, Severity, std::string,
                             RefCountedPtr<BaseNode>)>) const
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  const size_t max_event_memory_;
  const gpr_timespec time_created_;

  mutable Mutex mu_;
  uint64_t num_events_logged_ ABSL_GUARDED_BY(mu_) = 0;
  size_t event_list_memory_usage_ ABSL_GUARDED_BY(mu_) = 0;
  TraceEvent* head_trace_ ABSL_GUARDED_BY(mu_) = nullptr;
  TraceEvent* tail_trace_ ABSL_GUARDED_BY(mu_) = nullptr;
};

}  // namespace channelz
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CHANNELZ_CHANNEL_TRACE_H
