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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CLOCK_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CLOCK_TRACKER_H_

#include <stdint.h>

#include <array>
#include <cinttypes>
#include <cstdint>
#include <map>
#include <optional>
#include <random>
#include <set>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/importers/common/metadata_tracker.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/status_macros.h"

namespace perfetto {
namespace trace_processor {

class ClockTrackerTest;
class TraceProcessorContext;

// This class handles synchronization of timestamps across different clock
// domains. This includes multi-hop conversions from two clocks A and D, e.g.
// A->B -> B->C -> C->D, even if we never saw a snapshot that contains A and D
// at the same time.
// The API is fairly simple (but the inner operation is not):
// - AddSnapshot(map<clock_id, timestamp>): pushes a set of clocks that have
//   been snapshotted at the same time (within technical limits).
// - ToTraceTime(src_clock_id, src_timestamp):
//   converts a timestamp between clock domain and TraceTime.
//
// Concepts:
// - Snapshot hash:
//   As new snapshots are pushed via AddSnapshot() we compute a snapshot hash.
//   Such hash is the hash(clock_ids) (only IDs, not their timestamps) and is
//   used to find other snapshots that involve the same clock domains.
//   Two clock snapshots have the same hash iff they snapshot the same set of
//   clocks (the order of clocks is irrelevant).
//   This hash is used to efficiently go from the clock graph pathfinder to the
//   time-series obtained by appending the various snapshots.
// - Snapshot id:
//   A simple monotonic counter that is incremented on each AddSnapshot() call.
//
// Data structures:
//  - For each clock domain:
//    - For each snapshot hash:
//      - A logic vector of (snapshot_id, timestamp) tuples (physically stored
//        as two vectors of the same length instead of a vector of pairs).
// This allows to efficiently binary search timestamps within a clock domain
// that were obtained through a particular snapshot.
//
// - A graph of edges (source_clock, target_clock) -> snapshot hash.
//
// Operation:
// Upon each AddSnapshot() call, we incrementally build an unweighted, directed
// graph, which has clock domains as nodes.
// The graph is timestamp-oblivious. As long as we see one snapshot that
// connects two clocks, we assume we'll always be able to convert between them.
// This graph is queried by the Convert() function to figure out the shortest
// path between clock domain, possibly involving hopping through snapshots of
// different type (i.e. different hash).
//
// Example:

// We see a snapshot, with hash S1, for clocks (A,B,C). We build the edges in
// the graph: A->B, B->C, A->C (and the symmetrical ones). In other words we
// keep track of the fact that we can convert between any of them using S1.
// Later we get another snapshot containing (C,E), this snapshot will have a
// different hash (S2, because Hash(C,E) != Hash(A,B,C)) and will add the edges
// C->E, E->C [via S2] to the graph.
// At this point when we are asked to convert a timestamp from A to E, or
// viceversa, we use a simple BFS to figure out a conversion path that is:
// A->C [via S1] + C->E [via S2].
//
// Visually:
// Assume we make the following calls:
//  - AddSnapshot(A:10, B:100)
//  - AddSnapshot(A:20, C:2000)
//  - AddSnapshot(B:400, C:5000)
//  - AddSnapshot(A:30, B:300)

// And assume Hash(A,B) = S1, H(A,C) = S2, H(B,C) = S3
// The vectors in the tracker will look as follows:
// Clock A:
//   S1        {t:10, id:1}                                      {t:30, id:4}
//   S2        |               {t:20, id:2}                      |
//             |               |                                 |
// Clock B:    |               |                                 |
//   S1        {t:100, id:1}   |                                 {t:300, id:4}
//   S3                        |                  {t:400, id:3}
//                             |                  |
// Clock C:                    |                  |
//   S2                        {t: 2000, id: 2}   |
//   S3                                           {t:5000, id:3}

class ClockTracker {
 public:
  using ClockId = int64_t;

  explicit ClockTracker(TraceProcessorContext*);
  virtual ~ClockTracker();

  // Clock description.
  struct Clock {
    explicit Clock(ClockId clock_id) : id(clock_id) {}
    Clock(ClockId clock_id, int64_t unit, bool incremental)
        : id(clock_id), unit_multiplier_ns(unit), is_incremental(incremental) {}

    ClockId id;
    int64_t unit_multiplier_ns = 1;
    bool is_incremental = false;
  };

  // Timestamp with clock.
  struct ClockTimestamp {
    ClockTimestamp(ClockId id, int64_t ts) : clock(id), timestamp(ts) {}
    ClockTimestamp(ClockId id, int64_t ts, int64_t unit, bool incremental)
        : clock(id, unit, incremental), timestamp(ts) {}

    Clock clock;
    int64_t timestamp;
  };

  // IDs in the range [64, 128) are reserved for sequence-scoped clock ids.
  // They can't be passed directly in ClockTracker calls and must be resolved
  // to 64-bit global clock ids by calling SeqScopedClockIdToGlobal().
  static bool IsSequenceClock(ClockId clock_id) {
    return clock_id >= 64 && clock_id < 128;
  }

  // Converts a sequence-scoped clock ids to a global clock id that can be
  // passed as argument to ClockTracker functions.
  static ClockId SequenceToGlobalClock(uint32_t seq_id, uint32_t clock_id) {
    PERFETTO_DCHECK(IsSequenceClock(clock_id));
    return (static_cast<int64_t>(seq_id) << 32) | clock_id;
  }

  void set_timezone_offset(int64_t offset) { timezone_offset_ = offset; }

  std::optional<int64_t> timezone_offset() const { return timezone_offset_; }

  // Appends a new snapshot for the given clock domains.
  // This is typically called by the code that reads the ClockSnapshot packet.
  // Returns the internal snapshot id of this set of clocks.
  base::StatusOr<uint32_t> AddSnapshot(const std::vector<ClockTimestamp>&);

  // Sets clock offset for the given clock domain to convert to the host trace
  // time. This is typically called by the code that reads the RemoteClockSync
  // packet. Typically only the offset of |trace_time_clock_id_| (which is
  // CLOCK_BOOTTIME) is used.
  void SetClockOffset(ClockId clock_id, int64_t offset) {
    clock_offsets_[clock_id] = offset;
  }

  // Apply the clock offset to convert remote trace times to host trace time.
  PERFETTO_ALWAYS_INLINE int64_t ToHostTraceTime(int64_t timestamp) {
    if (PERFETTO_LIKELY(!context_->machine_id())) {
      // No need to convert host timestamps.
      return timestamp;
    }

    // Find the offset for |trace_time_clock_id_| and apply the offset, or
    // default offset 0 if not offset is found for |trace_time_clock_id_|.
    int64_t clock_offset = clock_offsets_[trace_time_clock_id_];
    return timestamp - clock_offset;
  }

  PERFETTO_ALWAYS_INLINE base::StatusOr<int64_t> ToTraceTime(
      ClockId clock_id,
      int64_t timestamp) {
    if (PERFETTO_UNLIKELY(!trace_time_clock_id_used_for_conversion_)) {
      context_->metadata_tracker->SetMetadata(
          metadata::trace_time_clock_id,
          Variadic::Integer(trace_time_clock_id_));
      trace_time_clock_id_used_for_conversion_ = true;
    }
    trace_time_clock_id_used_for_conversion_ = true;

    if (clock_id == trace_time_clock_id_) {
      return ToHostTraceTime(timestamp);
    }

    ASSIGN_OR_RETURN(int64_t ts,
                     Convert(clock_id, timestamp, trace_time_clock_id_));
    return ToHostTraceTime(ts);
  }

  // If trace clock and source clock are available in the snapshot will return
  // the trace clock time in snapshot.
  std::optional<int64_t> ToTraceTimeFromSnapshot(
      const std::vector<ClockTimestamp>&);

  void SetTraceTimeClock(ClockId clock_id) {
    PERFETTO_DCHECK(!IsSequenceClock(clock_id));
    if (trace_time_clock_id_used_for_conversion_ &&
        trace_time_clock_id_ != clock_id) {
      PERFETTO_ELOG("Not updating trace time clock from %" PRId64 " to %" PRId64
                    " because the old clock was already used for timestamp "
                    "conversion - ClockSnapshot too late in trace?",
                    trace_time_clock_id_, clock_id);
      return;
    }
    trace_time_clock_id_ = clock_id;
    context_->metadata_tracker->SetMetadata(
        metadata::trace_time_clock_id, Variadic::Integer(trace_time_clock_id_));
  }

  bool HasPathToTraceTime(ClockId clock_id) {
    if (clock_id == trace_time_clock_id_) {
      return true;
    }
    return FindPath(clock_id, trace_time_clock_id_).valid();
  }

  void set_cache_lookups_disabled_for_testing(bool v) {
    cache_lookups_disabled_for_testing_ = v;
  }

  const base::FlatHashMap<ClockId, int64_t>& clock_offsets_for_testing() {
    return clock_offsets_;
  }

 private:
  using SnapshotHash = uint32_t;

  // 0th argument is the source clock, 1st argument is the target clock.
  using ClockGraphEdge = std::tuple<ClockId, ClockId, SnapshotHash>;

  // TODO(b/273263113): Remove in the next stages.
  friend class ClockTrackerTest;

  // A value-type object that carries the information about the path between
  // two clock domains. It's used by the BFS algorithm.
  struct ClockPath {
    static constexpr size_t kMaxLen = 4;
    ClockPath() = default;
    ClockPath(const ClockPath&) = default;

    // Constructs an invalid path with just a source node.
    explicit ClockPath(ClockId clock_id) : last(clock_id) {}

    // Constructs a path by appending a node to |prefix|.
    // If |prefix| = [A,B] and clock_id = C, then |this| = [A,B,C].
    ClockPath(const ClockPath& prefix, ClockId clock_id, SnapshotHash hash) {
      PERFETTO_DCHECK(prefix.len < kMaxLen);
      len = prefix.len + 1;
      path = prefix.path;
      path[prefix.len] = ClockGraphEdge{prefix.last, clock_id, hash};
      last = clock_id;
    }

    bool valid() const { return len > 0; }
    const ClockGraphEdge& at(uint32_t i) const {
      PERFETTO_DCHECK(i < len);
      return path[i];
    }

    uint32_t len = 0;
    ClockId last = 0;
    std::array<ClockGraphEdge, kMaxLen> path;  // Deliberately uninitialized.
  };

  struct ClockSnapshots {
    // Invariant: both vectors have the same length.
    std::vector<uint32_t> snapshot_ids;
    std::vector<int64_t> timestamps_ns;
  };

  struct ClockDomain {
    // One time-series for each hash.
    std::map<SnapshotHash, ClockSnapshots> snapshots;

    // Multiplier for timestamps given in this domain.
    int64_t unit_multiplier_ns = 1;

    // Whether this clock domain encodes timestamps as deltas. This is only
    // supported on sequence-local domains.
    bool is_incremental = false;

    // If |is_incremental| is true, this stores the most recent absolute
    // timestamp in nanoseconds.
    int64_t last_timestamp_ns = 0;

    // Treats |timestamp| as delta timestamp if the clock uses incremental
    // encoding, and as absolute timestamp otherwise.
    int64_t ToNs(int64_t timestamp) {
      if (!is_incremental)
        return timestamp * unit_multiplier_ns;

      int64_t delta_ns = timestamp * unit_multiplier_ns;
      last_timestamp_ns += delta_ns;
      return last_timestamp_ns;
    }

    const ClockSnapshots& GetSnapshot(uint32_t hash) const {
      auto it = snapshots.find(hash);
      PERFETTO_DCHECK(it != snapshots.end());
      return it->second;
    }
  };

  // Holds data for cached entries. At the moment only single-path resolution
  // are cached.
  struct CachedClockPath {
    ClockId src;
    ClockId target;
    ClockDomain* src_domain;
    int64_t min_ts_ns;
    int64_t max_ts_ns;
    int64_t translation_ns;
  };

  ClockTracker(const ClockTracker&) = delete;
  ClockTracker& operator=(const ClockTracker&) = delete;

  base::StatusOr<int64_t> ConvertSlowpath(ClockId src_clock_id,
                                          int64_t src_timestamp,
                                          ClockId target_clock_id);

  // Converts a timestamp between two clock domains. Tries to use the cache
  // first (only for single-path resolutions), then falls back on path finding
  // as described in the header.
  base::StatusOr<int64_t> Convert(ClockId src_clock_id,
                                  int64_t src_timestamp,
                                  ClockId target_clock_id) {
    if (PERFETTO_LIKELY(!cache_lookups_disabled_for_testing_)) {
      for (const auto& cached_clock_path : cache_) {
        if (cached_clock_path.src != src_clock_id ||
            cached_clock_path.target != target_clock_id)
          continue;
        int64_t ns = cached_clock_path.src_domain->ToNs(src_timestamp);
        if (ns >= cached_clock_path.min_ts_ns &&
            ns < cached_clock_path.max_ts_ns)
          return ns + cached_clock_path.translation_ns;
      }
    }
    return ConvertSlowpath(src_clock_id, src_timestamp, target_clock_id);
  }

  // Returns whether |global_clock_id| represents a sequence-scoped clock, i.e.
  // a ClockId returned by SeqScopedClockIdToGlobal().
  static bool IsConvertedSequenceClock(ClockId global_clock_id) {
    // If the id is > 2**32, this is a sequence-scoped clock id translated into
    // the global namespace.
    return (global_clock_id >> 32) > 0;
  }

  ClockPath FindPath(ClockId src, ClockId target);

  ClockDomain* GetClock(ClockId clock_id) {
    auto it = clocks_.find(clock_id);
    PERFETTO_DCHECK(it != clocks_.end());
    return &it->second;
  }

  TraceProcessorContext* const context_;
  ClockId trace_time_clock_id_ = 0;
  std::map<ClockId, ClockDomain> clocks_;
  std::set<ClockGraphEdge> graph_;
  std::set<ClockId> non_monotonic_clocks_;
  std::array<CachedClockPath, 2> cache_{};
  bool cache_lookups_disabled_for_testing_ = false;
  std::minstd_rand rnd_;  // For cache eviction.
  uint32_t cur_snapshot_id_ = 0;
  bool trace_time_clock_id_used_for_conversion_ = false;
  base::FlatHashMap<ClockId, int64_t> clock_offsets_;
  std::optional<int64_t> timezone_offset_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CLOCK_TRACKER_H_
