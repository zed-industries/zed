/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_REDACTION_PROCESS_THREAD_TIMELINE_H_
#define SRC_TRACE_REDACTION_PROCESS_THREAD_TIMELINE_H_

#include <cstdint>
#include <limits>
#include <vector>

namespace perfetto::trace_redaction {

class ProcessThreadTimeline {
 public:
  // Opened and closed events are used to mark the start and end of lifespans.
  struct Event {
   public:
    static constexpr auto kUnknownPid = std::numeric_limits<int32_t>::max();
    static constexpr auto kUnknownUid = std::numeric_limits<uint64_t>::max();

    enum class Type { kInvalid, kOpen, kClose };

    bool operator==(const Event& o) const {
      switch (type) {
        case Type::kOpen:
          return o.type == Type::kOpen && ts == o.ts && pid == o.pid &&
                 ppid == o.ppid && uid == o.uid;

        case Type::kClose:
          return o.type == Type::kClose && ts == o.ts && pid == o.pid;

        case Type::kInvalid:
          return o.type == Type::kInvalid;
      }

      return false;
    }

    bool operator!=(const Event& o) const { return !(*this == o); }

    bool valid() const { return type != Type::kInvalid; }

    static Event Open(uint64_t ts, int32_t pid, int32_t ppid, uint64_t uid) {
      return {Type::kOpen, ts, pid, ppid, uid};
    }

    static Event Open(uint64_t ts, int32_t pid, int32_t ppid) {
      return {Type::kOpen, ts, pid, ppid, kUnknownUid};
    }

    static Event Close(uint64_t ts, int32_t pid) {
      return {Type::kClose, ts, pid, kUnknownPid, kUnknownUid};
    }

    Type type = Type::kInvalid;

    // The time when the event occurred. Undefined when type is kInvalid.
    uint64_t ts = 0;

    // The subject of the event. Undefined when type is kInvalid.
    int32_t pid = kUnknownPid;

    // The parent of the subject. kUnknownPid if the parent is unknown.
    // Undefined when type is kClose or kInvalid.
    int32_t ppid = kUnknownPid;

    // The package containing the subject. kUnknownUid if the package is
    // unknown. Undefined when type is kClose or kInvalid.
    uint64_t uid = kUnknownUid;
  };

  ProcessThreadTimeline() = default;

  ProcessThreadTimeline(const ProcessThreadTimeline&) = delete;
  ProcessThreadTimeline& operator=(const ProcessThreadTimeline&) = delete;
  ProcessThreadTimeline(ProcessThreadTimeline&&) = delete;
  ProcessThreadTimeline& operator=(ProcessThreadTimeline&&) = delete;

  void Append(const Event& event);

  // REQUIRED: Sorts all events by pid, making it possible to locate the subset
  // of events connected to a pid. Events are not sorted by time because the
  // subset of events will, on average, be trivially small.
  void Sort();

  // Returns true if a process/thread is connected to a package.
  bool PidConnectsToUid(uint64_t ts, int32_t pid, uint64_t uid) const;

  // Get the opening event for a pid. If pid ends at ts, an opening event will
  // still be returned.
  const Event* GetOpeningEvent(uint64_t ts, int32_t pid) const;

  // SELECT MAX(ts), * FROM events WHERE pid=@pid AND type=@type AND ts<=@ts
  const Event* QueryLeftMax(uint64_t ts,
                            int32_t pid,
                            ProcessThreadTimeline::Event::Type type) const;

 private:
  enum class Mode {
    // The timeline can safely be queried. If the timeline is in read mode, and
    // a user writes to the timeline, the timeline will change to write mode.
    kRead,

    // The timeline change be changed. If the timeline is not in write mode,
    // reading from the timeline will throw an error. Sort() must be called to
    // change the timeline from write to read mode.
    kWrite
  };

  std::vector<Event> events_;

  Mode mode_ = Mode::kRead;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_PROCESS_THREAD_TIMELINE_H_
