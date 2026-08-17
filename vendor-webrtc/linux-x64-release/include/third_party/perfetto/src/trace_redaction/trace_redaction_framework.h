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

#ifndef SRC_TRACE_REDACTION_TRACE_REDACTION_FRAMEWORK_H_
#define SRC_TRACE_REDACTION_TRACE_REDACTION_FRAMEWORK_H_

#include <bitset>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <unordered_set>
#include <vector>

#include "perfetto/base/flat_set.h"
#include "perfetto/base/status.h"
#include "src/trace_redaction/frame_cookie.h"
#include "src/trace_redaction/process_thread_timeline.h"

#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto::trace_redaction {

// Multiple packages can share the same name. This is common when a device has
// multiple users. When this happens, each instance shares the 5 least
// significant digits.
constexpr uint64_t NormalizeUid(uint64_t uid) {
  return uid % 1000000;
}

class SystemInfo {
 public:
  int32_t AllocateSynthThread() { return ++next_synth_thread_; }

  uint32_t ReserveCpu(uint32_t cpu) {
    last_cpu_ = std::max(last_cpu_, cpu);
    return last_cpu_;
  }

  uint32_t cpu_count() const { return last_cpu_ + 1; }

 private:
  // This is the last allocated tid. Using a tid equal to or less than this tid
  // risks a collision with another tid. If a tid is ever created (by a
  // primitive) this should be advanced to the max between this value and the
  // new tid.
  //
  // On a 64 bit machine, the max pid limit is 2^22 (approximately 4 million).
  // Perfetto uses a 32 (signed) int for the pid. Even in this case, there is
  // room for 2^9 synthetic threads (2 ^ (31 - 22) = 2 ^ 9).
  //
  // Furthermore, ther Android source code return 4194304 (2 ^ 22) on 64 bit
  // devices.
  //
  //  /proc/sys/kernel/pid_max (since Linux 2.5.34)
  //      This file specifies the value at which PIDs wrap around
  //      (i.e., the value in this file is one greater than the
  //      maximum PID).  PIDs greater than this value are not
  //      allocated; thus, the value in this file also acts as a
  //      system-wide limit on the total number of processes and
  //      threads.  The default value for this file, 32768, results
  //      in the same range of PIDs as on earlier kernels.  On
  //      32-bit platforms, 32768 is the maximum value for pid_max.
  //      On 64-bit systems, pid_max can be set to any value up to
  //      2^22 (PID_MAX_LIMIT, approximately 4 million).
  //
  // SOURCE: https://man7.org/linux/man-pages/man5/proc.5.html
  int32_t next_synth_thread_ = 1 << 22;

  // The last CPU index seen. If this value is 7, it means there are at least
  // 8 CPUs.
  uint32_t last_cpu_ = 0;
};

class SyntheticProcess {
 public:
  explicit SyntheticProcess(const std::vector<int32_t>& tids) : tids_(tids) {}

  // Use the SYSTEM_UID (i.e. 1000) because it best represents this "type" of
  // process.
  int32_t uid() const { return 1000; }

  // Use ppid == 1 which is normally considered to be init on Linux?
  int32_t ppid() const { return 1; }

  int32_t tgid() const { return tids_.front(); }

  const std::vector<int32_t>& tids() const { return tids_; }

  int32_t RunningOn(uint32_t cpu) const { return tids_.at(1 + cpu); }

  int32_t RunningOn(int32_t cpu) const {
    return tids_.at(1 + static_cast<size_t>(cpu));
  }

 private:
  std::vector<int32_t> tids_;
};

// Primitives should be stateless. All state should be stored in the context.
// Primitives should depend on data in the context, not the origin of the data.
// This allows primitives to be swapped out or work together to populate data
// needed by another primitive.
//
// For this to work, primitives are divided into three types:
//
//  `CollectPrimitive` :  Reads data from trace packets and saves low-level data
//                        in the context.
//
//  `BuildPrimitive` :    Reads low-level data from the context and builds
//                        high-level (read-optimized) data structures.
//
//  `TransformPrimitive`: Reads high-level data from the context and modifies
//                        trace packets.
class Context {
 public:
  // Each packet will have a trusted uid. This is the package emitting the
  // event. In production we only expect to see system uids. 9999 is the
  // last allowed uid (allow all uids less than or equal to 9999).
  static constexpr int32_t kMaxTrustedUid = 9999;

  // The package that should not be redacted. This must be populated before
  // running any primitives.
  std::string package_name;

  // The package list maps a package name to a uid. It is possible for multiple
  // package names to map to the same uid, for example:
  //
  //    packages {
  //      name: "com.google.android.gms"
  //      uid: 10113
  //      debuggable: false
  //      profileable_from_shell: false
  //      version_code: 235013038
  //    }
  //
  // Processes reference their package using a uid:
  //
  //    processes {
  //      pid: 18176
  //      ppid: 904
  //      cmdline: "com.google.android.gms.persistent"
  //      uid: 10113
  //    }
  //
  // An oddity within Android is that two or more processes can reference the
  // same package using different uids:
  //
  //    A = package(M * 100000 + X)
  //    B = package(N * 100000 + X)
  //
  // A and B map to the same package. This happens when there are two or more
  // profiles on the device (e.g. a work profile and a personal profile).
  //
  // From the example above:
  //
  //  uid = package_uid_for("com.google.android.gms")
  //  pid = main_thread_for(uid)
  //  ASSERT(pid == 18176)
  //
  // However, if there is another profile:
  //
  //    processes {
  //      pid: 18176
  //      ppid: 904
  //      cmdline: "com.google.android.gms.persistent"
  //      uid: 10113
  //    }
  //    processes {
  //      pid: 21388
  //      ppid: 904
  //      cmdline: "com.google.android.gms.persistent"
  //      uid: 1010113
  //    }
  //
  // The logic from before still hold, however, if the traced process was pid
  // 21388, it will be merged with the other threads.
  //
  // To avoid this problem from happening, we normalize the uids and treat
  // both instances as a single process:
  //
  //    processes {
  //      pid: 18176
  //      ppid: 904
  //      cmdline: "com.google.android.gms.persistent"
  //      uid: 10113
  //    }
  //    processes {
  //      pid: 21388
  //      ppid: 904
  //      cmdline: "com.google.android.gms.persistent"
  // -    uid: 1010113
  // +    uid: 10113
  //    }
  //
  // It sounds like there would be a privacy concern, but because both processes
  // are from the same app and are being collected from the same user, there
  // are no new privacy issues by doing this.
  //
  // But where should the uids be normalized? The dividing line is the timeline
  // interface, specifically, should the timeline know anything about uids
  // (other than "it's a number").
  //
  // To avoid expanding the timeline's scope, the uid normalizations is done
  // outside of the timeline. When a uid is passed into the timeline, it should
  // be normalized (i.e. 5 != 100005). When the timeline is queried, the uid
  // should be normalized. This increases the risk for error, but there are only
  // two places where uids are set, writing the uid to the context and writing
  // the uid to the timeline.
  std::optional<uint64_t> package_uid;

  // Trace packets contain a "one of" entry called "data". This field can be
  // thought of as the message. A track packet with have other fields along
  // side "data" (e.g. "timestamp"). These fields can be thought of as metadata.
  //
  // A message should be removed if:
  //
  //  ...we know it contains too much sensitive information
  //
  //  ...we know it contains sensitive information and we know how to remove
  //        the sensitive information, but don't have the resources to do it
  //        right now
  //
  //  ...we know it provide little value
  //
  // "trace_packet_allow_list" contains the field ids of trace packets we want
  // to pass onto later transformations. Examples are:
  //
  //    - protos::pbzero::TracePacket::kProcessTreeFieldNumber
  //    - protos::pbzero::TracePacket::kProcessStatsFieldNumber
  //    - protos::pbzero::TracePacket::kClockSnapshotFieldNumber
  //
  // If the mask is set to 0x00, all fields would be removed. This should not
  // happen as some metadata provides context between packets.
  //
  // TracePacket has kForTestingFieldNumber which is set to 900.
  using TracePacketMask = std::bitset<1024>;
  TracePacketMask packet_mask;

  // Ftrace packets contain a "one of" entry called "event". Within the scope of
  // a ftrace event, the event can be considered the payload and other other
  // values can be considered metadata (e.g. timestamp and pid).
  //
  // A ftrace event should be removed if:
  //
  //  ... we know it contains too much sensitive information
  //
  //  ... we know it contains sensitive information and we have some ideas on
  //      to remove it, but don't have the resources to do it right now (e.g.
  //      print).
  //
  //  ... we don't see value in including it
  //
  // "ftrace_packet_allow_list" contains field ids of ftrace packets that we
  // want to pass onto later transformations. An example would be:
  //
  //  ... kSchedWakingFieldNumber because it contains cpu activity information
  //
  // Compared against track days, the rules around removing ftrace packets are
  // complicated because...
  //
  //  packet {
  //    ftrace_packets {  <-- ONE-OF    (1)
  //      event {         <-- REPEATED  (2)
  //        cpu_idle { }  <-- ONE-OF    (3)
  //      }
  //      event { ... }
  //    }
  //  }
  //
  //  1.  A ftrace packet will populate the one-of slot in the trace packet.
  //
  //  2.  A ftrace packet can have multiple events
  //
  //  3.  In this example, a cpu_idle event populates the one-of slot in the
  //      ftrace event
  //
  // Ftrace event has kMaliMaliPMMCURESETWAITFieldNumber which is set to 532.
  using FtraceEventMask = std::bitset<1024>;
  FtraceEventMask ftrace_mask;

  //  message SuspendResumeFtraceEvent {
  //    optional string action = 1 [(datapol.semantic_type) = ST_NOT_REQUIRED];
  //    optional int32 val = 2;
  //    optional uint32 start = 3 [(datapol.semantic_type) = ST_NOT_REQUIRED];
  //  }
  //
  // The "action" in SuspendResumeFtraceEvent is a free-form string. There are
  // some know and expected values. Those values are stored here and all events
  // who's action value is not found here, the ftrace event will be dropped.
  base::FlatSet<std::string> suspend_result_allow_list;

  // The timeline is a query-focused data structure that connects a pid to a
  // uid at specific point in time.
  //
  // A timeline has two modes:
  //
  //    1. write-only
  //    2. read-only
  //
  // Attempting to use the timeline incorrectly results in undefined behaviour.
  //
  // To use a timeline, the primitive needs to be "built" (add events) and then
  // "sealed" (transition to read-only).
  //
  // A timeline must have Sort() called to change from write-only to read-only.
  // After Sort(), Flatten() and Reduce() can be called (optional) to improve
  // the practical look-up times (compared to theoretical look-up times).
  std::unique_ptr<ProcessThreadTimeline> timeline;

  // All frame events:
  //
  //  - ActualDisplayFrame
  //  - ActualSurfaceFrame
  //  - ExpectedDisplayFrame
  //  - ExpectedSurfaceFrame
  //
  // Connect a time, a pid, and a cookie value. Cookies are unique within a
  // trace, so if a cookie was connected to the target package, it can always be
  // used.
  //
  // End events (i.e. FrameEnd) only have a time and cookie value. The cookie
  // value connects it to its start time.
  //
  // In the collect phase, all start events are collected and converted to a
  // simpler structure.
  //
  // In the build phase, the cookies are filtered to only include the ones that
  // belong to the target package. This is down in the build phase, and not the
  // collect phase, because the timeline is needed to determine if the cookie
  // belongs to the target package.
  std::vector<FrameCookie> global_frame_cookies;

  // The collect of cookies that belong to the target package. Because cookie
  // values are unique within the scope of the trace, pid and time are no longer
  // needed and a set can be used for faster queries.
  std::unordered_set<int64_t> package_frame_cookies;

  std::optional<SystemInfo> system_info;

  std::unique_ptr<SyntheticProcess> synthetic_process;
};

// Extracts low-level data from the trace and writes it into the context. The
// life cycle of a collect primitive is:
//
//  primitive.Begin(&context);
//
//  for (auto& packet : packets) {
//    primitive.Collect(packet, &context);
//  }
//
//  primitive.End(&context);
class CollectPrimitive {
 public:
  virtual ~CollectPrimitive();

  // Called once before the first call to Collect(...).
  virtual base::Status Begin(Context*) const;

  // Reads a trace packet and updates the context.
  virtual base::Status Collect(const protos::pbzero::TracePacket::Decoder&,
                               Context*) const = 0;

  // Called once after the last call to Collect(...).
  virtual base::Status End(Context*) const;
};

// Responsible for converting low-level data from the context and storing it in
// the context (high-level data).
class BuildPrimitive {
 public:
  virtual ~BuildPrimitive();

  // Reads low-level data from the context and writes high-level data to the
  // context.
  virtual base::Status Build(Context* context) const = 0;
};

// Responsible for modifying trace packets using data from the context.
class TransformPrimitive {
 public:
  virtual ~TransformPrimitive();

  // Modifies a packet using data from the context.
  virtual base::Status Transform(const Context& context,
                                 std::string* packet) const = 0;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_TRACE_REDACTION_FRAMEWORK_H_
