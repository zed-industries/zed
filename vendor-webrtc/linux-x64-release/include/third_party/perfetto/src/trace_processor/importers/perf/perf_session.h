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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_SESSION_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_SESSION_H_

#include <sys/types.h>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/perf/perf_event.h"
#include "src/trace_processor/importers/perf/perf_event_attr.h"
#include "src/trace_processor/tables/profiler_tables_py.h"
#include "src/trace_processor/util/build_id.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

namespace perf_importer {

// Helper to deal with perf_event_attr instances in a perf file.
class PerfSession : public RefCounted {
 public:
  class Builder {
   public:
    explicit Builder(TraceProcessorContext* context) : context_(context) {}
    base::StatusOr<RefPtr<PerfSession>> Build();
    Builder& AddAttrAndIds(perf_event_attr attr, std::vector<uint64_t> ids) {
      attr_with_ids_.push_back({attr, std::move(ids)});
      return *this;
    }

   private:
    struct PerfEventAttrWithIds {
      perf_event_attr attr;
      std::vector<uint64_t> ids;
    };
    TraceProcessorContext* const context_;
    std::vector<PerfEventAttrWithIds> attr_with_ids_;
  };

  tables::PerfSessionTable::Id perf_session_id() const {
    return perf_session_id_;
  }

  RefPtr<PerfEventAttr> FindAttrForEventId(uint64_t id) const;

  base::StatusOr<RefPtr<PerfEventAttr>> FindAttrForRecord(
      const perf_event_header& header,
      const TraceBlobView& payload) const;

  void SetCmdline(const std::vector<std::string>& args);
  void SetEventName(uint64_t event_id, std::string name);
  void SetEventName(uint32_t type, uint64_t config, const std::string& name);

  void AddBuildId(int32_t pid, std::string filename, BuildId build_id);
  std::optional<BuildId> LookupBuildId(uint32_t pid,
                                       const std::string& filename) const;

  // The kernel stores the return address for non leaf frames in call chains.
  // Simpleperf accounts for this when writing perf data files, linux perf does
  // not. This method returns true if we need to convert return addresses to
  // call sites when parsing call chains (i.e. if the trace comes from linux
  // perf).
  bool needs_pc_adjustment() const { return is_simpleperf_ == false; }

  void SetIsSimpleperf() { is_simpleperf_ = true; }

  bool HasPerfClock() const;

 private:
  struct BuildIdMapKey {
    int32_t pid;
    std::string filename;

    struct Hasher {
      size_t operator()(const BuildIdMapKey& k) const {
        return static_cast<size_t>(base::Hasher::Combine(k.pid, k.filename));
      }
    };

    bool operator==(const BuildIdMapKey& o) const {
      return pid == o.pid && filename == o.filename;
    }
  };

  PerfSession(TraceProcessorContext* context,
              tables::PerfSessionTable::Id perf_session_id,
              RefPtr<PerfEventAttr> first_attr,
              base::FlatHashMap<uint64_t, RefPtr<PerfEventAttr>> attrs_by_id,
              bool has_single_perf_event_attr)
      : context_(context),
        perf_session_id_(perf_session_id),
        first_attr_(std::move(first_attr)),
        attrs_by_id_(std::move(attrs_by_id)),
        has_single_perf_event_attr_(has_single_perf_event_attr) {}

  bool ReadEventId(const perf_event_header& header,
                   const TraceBlobView& payload,
                   uint64_t& id) const;

  TraceProcessorContext* const context_;
  tables::PerfSessionTable::Id perf_session_id_;
  RefPtr<PerfEventAttr> first_attr_;
  base::FlatHashMap<uint64_t, RefPtr<PerfEventAttr>> attrs_by_id_;

  // Multiple ids can map to the same perf_event_attr. This member tells us
  // whether there was only one perf_event_attr (with potentially different ids
  // associated). This makes the attr lookup given a record trivial and not
  // dependant no having any id field in the records.
  bool has_single_perf_event_attr_;

  bool is_simpleperf_ = false;

  base::FlatHashMap<BuildIdMapKey, BuildId, BuildIdMapKey::Hasher> build_ids_;
};

}  // namespace perf_importer
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_SESSION_H_
