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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_FEATURES_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_FEATURES_H_

#include <cstdint>
#include <functional>
#include <limits>
#include <string>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/status.h"
#include "src/trace_processor/importers/perf/perf_event.h"

namespace perfetto ::trace_processor {
class TraceBlobView;

namespace perf_importer::feature {

enum Id : uint8_t {
  ID_RESERVED = 0,
  ID_TRACING_DATA = 1,
  ID_BUILD_ID = 2,
  ID_HOSTNAME = 3,
  ID_OS_RELEASE = 4,
  ID_VERSION = 5,
  ID_ARCH = 6,
  ID_NR_CPUS = 7,
  ID_CPU_DESC = 8,
  ID_CPU_ID = 9,
  ID_TOTAL_MEM = 10,
  ID_CMD_LINE = 11,
  ID_EVENT_DESC = 12,
  ID_CPU_TOPOLOGY = 13,
  ID_NUMA_TOPOLOGY = 14,
  ID_BRANCH_STACK = 15,
  ID_PMU_MAPPINGS = 16,
  ID_GROUP_DESC = 17,
  ID_AUX_TRACE = 18,
  ID_STAT = 19,
  ID_CACHE = 20,
  ID_SAMPLE_TIME = 21,
  ID_SAMPLE_TOPOLOGY = 22,
  ID_CLOCK_ID = 23,
  ID_DIR_FORMAT = 24,
  ID_BPF_PROG_INFO = 25,
  ID_BPF_BTF = 26,
  ID_COMPRESSED = 27,
  ID_CPU_PUM_CAPS = 28,
  ID_CLOCK_DATA = 29,
  ID_HYBRID_TOPOLOGY = 30,
  ID_PMU_CAPS = 31,
  ID_SIMPLEPERF_FILE = 128,
  ID_SIMPLEPERF_META_INFO = 129,
  ID_SIMPLEPERF_FILE2 = 132,
  ID_MAX = std::numeric_limits<uint8_t>::max(),
};

struct BuildId {
  static base::Status Parse(TraceBlobView,
                            std::function<base::Status(BuildId)> cb);
  int32_t pid;
  std::string build_id;
  std::string filename;
};

struct HeaderGroupDesc {
  static base::Status Parse(TraceBlobView, HeaderGroupDesc& out);
  struct Entry {
    std::string string;
    uint32_t leader_idx;
    uint32_t nr_members;
  };
  std::vector<Entry> entries;
};

struct EventDescription {
  static base::Status Parse(TraceBlobView,
                            std::function<base::Status(EventDescription)> cb);
  perf_event_attr attr;
  std::string event_string;
  std::vector<uint64_t> ids;
};

struct SimpleperfMetaInfo {
  static base::Status Parse(const TraceBlobView&, SimpleperfMetaInfo& out);
  base::FlatHashMap<std::string, std::string> entries;
  struct EventTypeAndConfig {
    uint32_t type;
    uint64_t config;
    bool operator==(const EventTypeAndConfig& other) {
      return type == other.type && config == other.config;
    }
    bool operator!=(const EventTypeAndConfig& other) {
      return !(*this == other);
    }
    struct Hasher {
      size_t operator()(const EventTypeAndConfig& o) const {
        return static_cast<size_t>(base::Hasher::Combine(o.config, o.type));
      }
    };
  };
  using EventName = std::string;
  base::FlatHashMap<EventTypeAndConfig, EventName, EventTypeAndConfig::Hasher>
      event_type_info;
};

base::Status ParseSimpleperfFile2(TraceBlobView,
                                  std::function<void(TraceBlobView)> cb);

base::StatusOr<std::vector<std::string>> ParseCmdline(TraceBlobView blob);

}  // namespace perf_importer::feature

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_FEATURES_H_
